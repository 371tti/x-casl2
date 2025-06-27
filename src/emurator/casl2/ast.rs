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
        let mut iter = str.split("/t").into_iter();
        
        let uc_label = iter.next().map(|s| s.trim().to_string());
        let uc_opcode = iter.next().map(|s| s.trim().to_string());
        let uc_operands = iter.next().map(|s| s.trim().to_string()).unwrap_or_default();
        
        let cd_label: Option<String>;
        let cd_opcode: Option<String>;
        let cd_operands: Vec<String>;

        // ラベルがある場合 ラベルを解析
        if let Some(label) = uc_label {
            if label.len() > 4 {
                return Err(Casl2AssemblerError::ParseError(format!("Label '{}' is too long", label)));
            }
            if label.len() == 0 {
                cd_label = None;
            } else {
                cd_label = Some(label);
            }
        } // ラベルがない場合skip
    }
}