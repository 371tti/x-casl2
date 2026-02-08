use std::{collections::HashMap, hash::Hash};

use crate::instruction::{Instruction, OpeCode};

/// Assembler
/// # 仕様
/// - DCのアドレスにJUMP可能
/// - 適当なアドレスにJUMP可能
/// - ルーチン外のDCはMAINルーチンのみで有効
pub struct Assembler {

}

impl Assembler {
    pub fn new() -> Self {
        Assembler {

        }
    }

    pub fn str_to_tokens(&self, source: &str) -> Vec<Lexer> {
        let mut tokens = Vec::new();
        for line in source.lines() {
            for word in line.split(&[' ', ','][..]).filter(|w| !w.is_empty()) {
                if word.starts_with(';') {
                    break; // Ignore comments
                }
                let token = Lexer::from_str(word);
                tokens.push(token);
            }
            tokens.push(Lexer::LF);
        }
        tokens
    }

    pub fn assemble(&self, source: &str) -> Result<Vec<u16>, AssemblerError> {
        let tokens = self.str_to_tokens(source);
        let ast_nodes = ASTNode::from_tokens(&tokens)?;
        let binary = ASTNode::to_bin(ast_nodes);
        Ok(binary)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexer {
    /// Special Tokens
    LF,

    /// Assembler Instructions
    DS,
    DC,
    START,
    END,

    /// Instructions
    NOP,
    LD,
    ST,
    LAD,
    ADDA,
    SUBA,
    ADDL,
    SUBL,
    AND,
    OR,
    XOR,
    CPA,
    CPL,
    SLA,
    SRA,
    SLL,
    SRL,
    JMI,
    JNZ,
    JZE,
    JUMP,
    JPL,
    JOV,
    PUSH,
    POP,
    CALL,
    RET,
    IN,
    OUT,
    RPUSH,
    RPOP,
    SVC,


    /// Literals
    GR(u8),
    NUM(i32),
    HEX(u16),
    STR(String),
    OTHER(String),
}

impl Lexer {
    pub fn from_str(s: &str) -> Self {
        match s {
            // Assembler Instructions
            "DS" => Lexer::DS,
            "DC" => Lexer::DC,
            "START" => Lexer::START,
            "END" => Lexer::END,

            // Instructions
            "NOP" => Lexer::NOP,
            "LD" => Lexer::LD,
            "ST" => Lexer::ST,
            "LAD" => Lexer::LAD,
            "ADDA" => Lexer::ADDA,
            "SUBA" => Lexer::SUBA,
            "ADDL" => Lexer::ADDL,
            "SUBL" => Lexer::SUBL,
            "AND" => Lexer::AND,
            "OR" => Lexer::OR,
            "XOR" => Lexer::XOR,
            "CPA" => Lexer::CPA,
            "CPL" => Lexer::CPL,
            "SLA" => Lexer::SLA,
            "SRA" => Lexer::SRA,
            "SLL" => Lexer::SLL,
            "SRL" => Lexer::SRL,
            "JMI" => Lexer::JMI,
            "JNZ" => Lexer::JNZ,
            "JZE" => Lexer::JZE,
            "JUMP" => Lexer::JUMP,
            "JPL" => Lexer::JPL,
            "JOV" => Lexer::JOV,
            "PUSH" => Lexer::PUSH,
            "POP" => Lexer::POP,
            "CALL" => Lexer::CALL,
            "RET" => Lexer::RET,
            "IN" => Lexer::IN,
            "OUT" => Lexer::OUT,
            "RPUSH" => Lexer::RPUSH,
            "RPOP" => Lexer::RPOP,
            "SVC" => Lexer::SVC,

            _ if s.starts_with("GR") && s.len() == 3 && s.chars().nth(2).unwrap().is_digit(10) => {
                let reg_num = s[2..].parse::<u8>().unwrap();
                Lexer::GR(reg_num)
            }
            _ if s.starts_with("#") => {
                let hex_value = u16::from_str_radix(&s[1..], 16).unwrap();
                Lexer::HEX(hex_value)
            }
            _ if s.starts_with("'") && s.ends_with("'") => {
                let str_value = s[1..s.len()-1].to_string();
                Lexer::STR(str_value)
            }
            _ if s.parse::<i32>().is_ok() => {
                let num_value = s.parse::<i32>().unwrap();
                Lexer::NUM(num_value)
            }
            _ => Lexer::OTHER(s.to_string()),
        }
    }
}

pub enum ASTNode {
    DC {
        label: String,
        value: Expr,
    },
    DS {
        label: String,
        size: usize,
    },
    MAIN {
        instructions: Vec<Instruction>,
    },
    SUBROUTINE {
        name: String,
        instructions: Vec<Instruction>,
    },
}

impl ASTNode {
    pub fn from_tokens(tokens: &[Lexer]) -> Result<Vec<ASTNode>, AssemblerError> {
        let mut token_itr = tokens.iter();
        let mut nodes = Vec::new();
        let mut stack = Vec::new();
        let mut line_count = 0;
        let mut is_routine = false;
        // todo: これ今Globalにラベル管理してるけど、ルーチンラベル以外はLocalで分けるべき
        let mut label_table = LabelTable::new();
        while let Some(token) = token_itr.next() {
            if token == &Lexer::LF && !is_routine{
                // AST Node 単位で処理
                line_count += 1;
                match stack.as_slice() {
                    // DC Directive
                    [Lexer::DC, Lexer::OTHER(label), expr @ ..] => {
                        label_table.get(label);
                        nodes.push(ASTNode::DC {
                            label: label.clone(),
                            value: Expr::from_tokens(expr).map_err(|e| e.with_line(line_count))?,
                        });
                    }
                    // DS Directive
                    [Lexer::DS, Lexer::OTHER(label), Lexer::NUM(size)] => {
                        label_table.get(label);
                        nodes.push(ASTNode::DS {
                            label: label.clone(),
                            size: *size as usize,
                        });
                    }
                    // MAIN Routine
                    [Lexer::OTHER(name), Lexer::START, Lexer::OTHER(main), Lexer::LF, tokens @ .., Lexer::END] if main == "MAIN" => {
                        label_table.get(name);
                        let mut inner_stack = Vec::new();
                        let mut instructions = Vec::new();
                        while let Some(t) = tokens.iter().next() {
                            if t == &Lexer::LF {
                                if !inner_stack.is_empty() {
                                    let instruction = Instruction::from_tokens(tokens, &mut label_table)
                                        .map_err(|e| e.with_line(line_count))?;
                                    instructions.push(instruction);
                                    inner_stack.clear();
                                }
                            } else {
                                inner_stack.push(t.clone());
                            }
                        }
                        nodes.push(ASTNode::MAIN {
                            instructions,
                        });
                    }
                    // Subroutine
                    [Lexer::OTHER(name), Lexer::START, tokens @ .., Lexer::END] => {
                        label_table.get(name);
                        let mut inner_stack = Vec::new();
                        let mut instructions = Vec::new();
                        while let Some(t) = tokens.iter().next() {
                            if t == &Lexer::LF {
                                if !inner_stack.is_empty() {
                                    let instruction = Instruction::from_tokens(tokens, &mut label_table)
                                        .map_err(|e| e.with_line(line_count))?;
                                    instructions.push(instruction);
                                    inner_stack.clear();
                                }
                            } else {
                                inner_stack.push(t.clone());
                            }
                        }
                        nodes.push(ASTNode::SUBROUTINE {
                            name: name.clone(),
                            instructions,
                        });
                    }
                    _ => {
                        stack.clear();
                    }
                }
            } else {
                match token {
                    Lexer::START => is_routine = true,
                    Lexer::END => is_routine = false,
                    _ => {}
                }
                stack.push(token.clone());
            }
        }
        Ok(nodes)
    }

    pub fn to_bin(nodes: Vec<ASTNode>) -> Vec<u16> {
        let mut binary = Vec::new();
        let mut to_link: HashMap<u64, Vec<>
        for node in nodes {
            match node {
                ASTNode::DC { label: _, value } => {
                    match value {
                        Expr::Word(w) => binary.push(w),
                        Expr::Data(d) => binary.extend(d),
                    }
                }
                ASTNode::DS { label: _, size } => {
                    binary.extend(vec![0; size]);
                }
                ASTNode::MAIN { instructions } | ASTNode::SUBROUTINE { name: _, instructions } => {
                    for instruction in instructions {
                        let encoded = instruction.encode();
                        binary.push(encoded);
                    }
                }
            }
        }
        binary
    }
}

pub struct LabelTable {
    table: HashMap<String, u32>,
    next_id: u32,
}

impl LabelTable {
    pub fn new() -> Self {
        LabelTable {
            table: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn get(&mut self, label: &str) -> u32 {
        if let Some(&id) = self.table.get(label) {
            id
        } else {
            let id = self.next_id;
            self.table.insert(label.to_string(), id);
            self.next_id += 1;
            id
        }
    }
}

impl Instruction {
    pub fn from_tokens(tokens: &[Lexer], label_table: &mut LabelTable) -> Result<Instruction, AssemblerError> {
        match tokens {
            [Lexer::NOP] => Ok(Instruction::NOP),
            [Lexer::LD, Lexer::GR(r1), Lexer::GR(r2)] => Ok(Instruction::LD(OpeCode::R2 { r1: *r1, r2: *r2 })),
            [Lexer::LD, Lexer::GR(r), Lexer::OTHER(label), Lexer::GR(x)] => {
                let address = label_table.get(label) as u16;
                Ok(Instruction::LD(OpeCode::RAddr { r: *r, addr: address, x: *x }) )
            }
            [Lexer::LD, Lexer::GR(r), expr @ .., Lexer::GR(x)] => {
                let address = match Expr::from_tokens(expr)? {
                    Expr::Word(w) => w,
                    _ => return Err(AssemblerError::SyntaxError("Expected word expression for address".to_string())),
                };
                Ok(Instruction::LD(OpeCode::RAddr { r: *r, addr: address, x: *x }) )
            }
            /*
            ここに全命令のやつかく
             */
            _ => Err(AssemblerError::SyntaxError(format!("Invalid instruction: {:?}", tokens))),
        }
    }
}

pub enum Expr {
    Word(u16),
    Data(Vec<u16>),
}

impl Expr {
    pub fn from_tokens(tokens: &[Lexer]) -> Result<Expr, AssemblerError> {
        match tokens {
            [] => Err(AssemblerError::SyntaxError("Empty expression".to_string())),
            [Lexer::NUM(n)] => Ok(Expr::Word(*n as u16)),
            [Lexer::HEX(h)] => Ok(Expr::Word(*h)),
            [Lexer::STR(s)] if s.len() == 1 => Ok(Expr::Word(s.chars().next().unwrap() as u16)),
            [Lexer::STR(s)] => Ok(Expr::Data(s.bytes().map(|b| b as u16).collect())),
            [tokens @ ..] if tokens.len() > 1 => {
                let mut data = Vec::new();
                for token in tokens {
                    match Self::from_tokens(&[token.clone()])? {
                        Expr::Word(w) => data.push(w),
                        Expr::Data(d) => data.extend(d),
                    }
                }
                Ok(Expr::Data(data))
            }
            e => Err(AssemblerError::SyntaxError(format!("Invalid expression: {:?}", e)))
        }
    }
}

pub enum AssemblerError {
    SyntaxError(String),
    SyntaxErrorWithLine(usize, String),
    SemanticError(String),
}

impl AssemblerError {
    pub fn with_line(self, line: usize) -> Self {
        match self {
            AssemblerError::SyntaxError(msg) => AssemblerError::SyntaxErrorWithLine(line, msg),
            other => other,
        }
    }
}












