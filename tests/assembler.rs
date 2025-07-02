
#[cfg(test)]
mod tests {
    use x_casl2::emurator::casl2::ast::ASTNode;

    #[test]
    fn test_ast_node_de() {
        let input = "KD1\tSTART\tADDR\nDAT\tDC\t3,3,3,3;this is comment\nADDR\tLD\tGR0,1000\n\tEND";
        let nodes = ASTNode::de(input).unwrap();
        println!("{:?}", nodes);
    }
}