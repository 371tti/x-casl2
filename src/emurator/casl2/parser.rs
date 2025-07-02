use crate::emurator::casl2::{err::Casl2AssemblerError, prefix::{assembler_instructions, GR_LIST}};

#[derive(Debug, Clone)]
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
    /// casl2を解析してASTノードのベクタを生成する
    pub fn de(str: &str) -> Result<Vec<Self>, Casl2AssemblerError> {
        let lines = str.lines();
        let mut nodes = Vec::new();

        for (i, line) in lines.enumerate() {
            nodes.push(Self::analyze(i, line)?);
        }
        Ok(nodes)
    }

    /// 1行の文字列を解析してASTノードを生成する
    /// regexをつかった簡単な解析を行う
    /// 
    /// 存在する命令しか処理しないことを保証しますが論理エラーなどは解析できない
    /// あとエラーメッセージも不十分なので todo エラーメッセージをわかりやすくする
    pub fn analyze(line_number: usize, str: &str) -> Result<Self, Casl2AssemblerError> {
        if str.trim().is_empty() {
            return Ok(Self::EMPTY);
        }

        let re = regex::Regex::new(
            r"^\s*(?:(?P<label>\w{1,4})\t)?(?P<opcode>\w+)(?:\t(?P<operand>[^\t;]*))?(?:\s*;\s*(?P<comment>.*))?$"
        ).unwrap();

        if let Some(cap) = re.captures(str) {
            let label = cap.name("label").map(|m| m.as_str().to_string());
            let opcode = cap.name("opcode").unwrap().as_str().to_string();
            let operand = cap.name("operand").map(|m| m.as_str().to_string()).unwrap_or_default();
            let comment = cap.name("comment").map(|m| m.as_str().to_string());
            println!("{:?} {:?} {:?} {:?}", label, opcode, operand, comment);

            match opcode.as_str() {
                assembler_instructions::DC => {
                    // アセンブラ命令
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
                assembler_instructions::START => {
                    // START命令
                    let operands: Vec<String> = operand.split(',').map(|s| s.trim().to_string()).collect();
                    if operands.len() != 1 {
                        return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid START instruction, line: {}\n\t{}", line_number, str)));
                    }
                    Ok(Self::START {
                        label: label.unwrap_or_default(),
                        addr: operands[0].clone(),
                    })
                },
                assembler_instructions::END => {
                    // END命令
                    if label.is_some() || !operand.is_empty() {
                        return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid END instruction, line: {}\n\t{}", line_number, str)));
                    }
                    Ok(Self::END)
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
                | assembler_instructions::LAD
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
                    match operands.len() {
                        0 => {
                            // ラベルとオペコードのみ
                            Ok(Self::Machine1wInstruction {
                                label: label,
                                opcode,
                                r1: 0,
                                r2: 0,
                                comment,
                            })
                        }
                        1 => {
                            // オペランドがレジスタ番号か確認
                            if let Some(r) = GR_LIST.iter().position(|&x| x == operands[0]) {
                                let r = r as u8;
                                Ok(Self::Machine1wInstruction {
                                    label: label,
                                    opcode,
                                    r1: r,
                                    r2: 0,
                                    comment,
                                })
                            } else {
                                // 出ない場合labelのはず JUMP addr みたいな
                                Ok(Self::Machine2wInstruction {
                                    label: label,
                                    opcode,
                                    r: 0,
                                    x: 0,
                                    addr: operands[0].clone(),
                                    comment,
                                })
                            }
                        }
                        2 => {
                            if let Some(r1) = GR_LIST.iter().position(|&x| x == operands[0]) {
                                // [0] がレジスタ番号か確認
                                // GRn GRm になってるはず
                                let r1 = r1 as u8;
                                if let Some(r2) = GR_LIST.iter().position(|&x| x == operands[1]) {
                                    // [1] もレジスタ番号か確認
                                    let r2 = r2 as u8;
                                    Ok(Self::Machine1wInstruction {
                                        label: label,
                                        opcode,
                                        r1,
                                        r2,
                                        comment,
                                    })
                                } else {
                                    // GRn addr になってるはず
                                    Ok(Self::Machine2wInstruction {
                                        label: label,
                                        opcode,
                                        r: 0,
                                        x: r1,
                                        addr: operands[0].clone(),
                                        comment,
                                    })
                                }
                            } else {
                                return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid first operand for {} instruction, line: {}\n\t{}", opcode, line_number, str)));
                            } 
                        }
                        3 => {
                            if let Some(r1) = GR_LIST.iter().position(|&x| x == operands[0]) {
                                // [0] がレジスタ番号か確認
                                let r1 = r1 as u8;
                                if let Some(r2) = GR_LIST.iter().position(|&x| x == operands[2]) {
                                    // [1] もレジスタ番号か確認
                                    let r2 = r2 as u8;
                                    Ok(Self::Machine2wInstruction {
                                        label: label,
                                        opcode,
                                        r: r1,
                                        x: r2,
                                        addr: operands[1].clone(),
                                        comment,
                                    })
                                } else {
                                    return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid second operand for {} instruction, line: {}\n\t{}", opcode, line_number, str)));
                                }
                            } else {
                                return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid first operand for {} instruction, line: {}\n\t{}", opcode, line_number, str)));
                            }
                        }
                        _ => {
                            return Err(Casl2AssemblerError::AnalyzeError(format!("Invalid number of operands for {} instruction, line: {}\n\t{}", opcode, line_number, str)));
                        }
                    }
                },
                _ => {
                    todo!()
                }
            }

        } else {
            Err(Casl2AssemblerError::ParseError(format!("Failed to parse line: {}", str)))
        }
    }
}