use std::fmt;

#[derive(Debug)]
pub enum Casl2AssemblerError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidInstruction(String),
    OutOfMemory,
    UnknownLabel(String),
    RuntimeError(String),
}

impl fmt::Display for Casl2AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Casl2AssemblerError::IoError(e) => write!(f, "IO error: {}", e),
            Casl2AssemblerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Casl2AssemblerError::InvalidInstruction(inst) => write!(f, "Invalid instruction: {}", inst),
            Casl2AssemblerError::OutOfMemory => write!(f, "Out of memory"),
            Casl2AssemblerError::UnknownLabel(label) => write!(f, "Unknown label: {}", label),
            Casl2AssemblerError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for Casl2AssemblerError {}

impl From<std::io::Error> for Casl2AssemblerError {
    fn from(err: std::io::Error) -> Self {
        Casl2AssemblerError::IoError(err)
    }
}