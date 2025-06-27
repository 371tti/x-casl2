use crate::emurator::casl2::{err::Casl2AssemblerError, prefix::assembler_instructions};

pub enum ASTNode {
    Machine1wInstruction {
        label: Option<String>,
        opcode: String,
        r1: u8,
        r2: u8,
        comment: Option<String>,
    },
    Machine2wInstruction {
        label: Option<String>,
        opcode: String,
        r: u8,
        x: u8,
        addr: String,
        comment: Option<String>,
    },
    AssemblerInstruction {
        label: String,
        opcode: String,
        operands: Vec<String>,
        comment: Option<String>,
    },
    START {
        label: String,
        addr: String,
    }, 
    END,
    EMPTY,
}

impl ASTNode {
    pub fn de(str: &str) -> Result<Vec<Self>, Casl2AssemblerError> {
        let lines = str.lines();
        let mut nodes = Vec::new();

        for (i, line) in lines.enumerate() {
            nodes.push(Self::analyze(i, line)?);
        }
        Ok(nodes)
    }

    pub fn analyze(line_number: usize, str: &str) -> Result<Self, Casl2AssemblerError> {
        if str.trim().is_empty() {
            return Ok(Self::EMPTY);
        }

        let re = regex::Regex::new(
            r"^(?:(?P<label>\w{1,4})\t)?(?P<opcode>\w+)\t(?P<operand>[^\t;]*)(?:\s*;\s*(?P<comment>.*))?$"
        ).unwrap();

        if let Some(cap) = re.captures(str) {
            let label = cap.name("label").map(|m| m.as_str().to_string());
            let opcode = cap.name("opcode").unwrap().as_str().to_string();
            let operand = cap.name("operand").unwrap().as_str().to_string();
            let comment = cap.name("comment").map(|m| m.as_str().to_string());
            println!("{:?} {:?} {:?} {:?}", label, opcode, operand, comment);

            match opcode.as_str() {
                assembler_instructions::DC => {
                    let operands: Vec<String> = operand.split(',').map(|s| s.trim().to_string()).collect();
                    if label.is_none() && operands.is_empty() {
                        return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid DC instruction, line: {}\n\t{}", line_number, str)));
                    }
                    Ok(Self::AssemblerInstruction {
                        label: label.unwrap_or_default(),
                        opcode,
                        operands,
                        comment,
                    })
                },
                assembler_instructions::NOP
                | assembler_instructions::LD
                | assembler_instructions::ADDA
                | assembler_instructions::SUBA
                | assembler_instructions::ADDL
                | assembler_instructions::SUBL
                | assembler_instructions::AND
                | assembler_instructions::OR
                | assembler_instructions::XOR
                | assembler_instructions::CPA
                | assembler_instructions::CPL
                | assembler_instructions::POP
                | assembler_instructions::ST
                | assembler_instructions::LDA
                | assembler_instructions::SLA
                | assembler_instructions::SRA
                | assembler_instructions::SLL
                | assembler_instructions::SRL
                | assembler_instructions::JMI
                | assembler_instructions::JNZ
                | assembler_instructions::JZE
                | assembler_instructions::JUMP
                | assembler_instructions::JPL
                | assembler_instructions::JOV
                | assembler_instructions::PUSH
                | assembler_instructions::CALL
                | assembler_instructions::SVC => {
                    let operands: Vec<String> = operand.split(',').map(|s| s.trim().to_string()).collect();
                    if 
                },
                _ => {
                    return Err(Casl2AssemblerError::AnalyzeError(format!("Unknown opcode: {}, line: {}\n\t{}", opcode, line_number, str)));
                }
            }

        } else {
            Err(Casl2AssemblerError::ParseError(format!("Failed to parse line: {}", str)))
        }
    }
}