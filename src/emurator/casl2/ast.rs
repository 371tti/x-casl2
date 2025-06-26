use crate::emurator::casl2::err::Casl2AssemblerError;

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
    END
}

impl ASTNode {
    pub fn de(str: &str) -> Result<Vec<Self>, Casl2AssemblerError> {
        let lines = str.lines();
        let mut nodes = Vec::new();

        for line in lines {
            nodes.push(Self::analyze(line)?);
        }
        Ok(nodes)
    }

    pub fn analyze(str: &str) -> Result<Self, Casl2AssemblerError> {
        let mut iter = str.split_whitespace();
        
    }
}