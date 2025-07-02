use std::collections::HashMap;

use crate::emurator::{casl2::parser::ASTNode, commet2::prefix::opecode_to_binary};


pub struct MemLine {
    pub addr: u16,
    pub node: ASTNode,
}

/// 実際のバイナリを生成する
/// 
/// ## 手順
/// 1. DC命令をすべて集め、mem_linesにアドレスを決めて格納 同時にlabel_mapにラベルとアドレスを登録
/// 2. 各命令をおなじくmem_linesにアドレスを決めて格納 label_mapにラベルとアドレスを登録
/// 3. 先頭からバイナリを生成。labelはlabel_mapからアドレスを取得していく
pub struct CodeGenerator {
    pub nodes: Vec<ASTNode>,
    pub label_map: HashMap<String, u16>,
    pub mem_lines: Vec<MemLine>,
}

impl ASTNode {
    pub fn gen_binary(&self, label_map: &HashMap<String, u16>) -> Vec<u16> {
        let mut binary: Vec<u16> = Vec::new();
        match self {
            ASTNode::Machine1wInstruction { label, opcode, r1, r2, comment } => {
                match opcode.as_str() {
                    "DC" => {
                        
                    }
                    _ => {
                        panic!("Unsupported 1w instruction: {}", opcode);
                    }
                }
            },
            ASTNode::Machine2wInstruction { label, opcode, r, x, addr, comment } => {
                let opcode = opecode_to_binary(&opcode, true);
            },
            ASTNode::AssemblerInstruction { label, opcode, operands, comment } => todo!(),
            ASTNode::START { label, addr } => todo!(),
            ASTNode::END => todo!(),
            ASTNode::EMPTY => todo!(),
        }
        binary
    }
}
