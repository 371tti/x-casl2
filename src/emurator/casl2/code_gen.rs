use std::collections::{HashMap, VecDeque};

use crate::emurator::{casl2::{err::Casl2AssemblerError, parser::ASTNode, prefix::assembler_instructions}, commet2::prefix::opecode_to_binary};


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

pub struct Routine {
    pub name: String,
    pub data_addr: u16,
    pub program_addr: u16,
    pub mem_lines: Vec<MemLine>,
}

// impl CodeGenerator {
//     /// ASTNodesからルーチンごとに切り分ける
//     pub fn routine_formatting(ast_nodes: Vec<ASTNode>) -> Result<Vec<Routine>, Casl2AssemblerError> {
//         let mut inst_buf: Vec<ASTNode> = Vec::new();
//         let mut data_buf: Vec<ASTNode> = Vec::new();
//         for node in &ast_nodes {
//             match node {
//                 ASTNode::Machine1wInstruction { label, opcode, r1, r2, comment } => {
//                     inst_buf.push(node.clone());
//                 },
//                 ASTNode::Machine2wInstruction { label, opcode, r, x, addr, comment } => {
//                     inst_buf.push(node.clone());
//                 },
//                 ASTNode::AssemblerInstruction { label, opcode, operands, comment } => {
//                     if opcode == assembler_instructions::DC {
//                         data_buf.push(node.clone());
//                     }
//                 },
//                 ASTNode::START { label, addr } => {
//                     // 何もしない バイナリに必要ない
//                 },
//                 ASTNode::END => {
//                     // 何もしない バイナリに必要ない
//                 },
//                 ASTNode::EMPTY => todo!(),
//             }
//         }

//         [12]
//     }
// }
