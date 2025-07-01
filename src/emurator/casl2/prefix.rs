pub mod assembler_instructions {
    pub const DC: &str = "DC";
    pub const START: &str = "START";
    pub const END: &str = "END";
    pub const NOP: &str = "NOP";
    pub const RET: &str = "RET";
    pub const LD: &str = "LD";
    pub const ADDA: &str = "ADDA";
    pub const SUBA: &str = "SUBA";
    pub const ADDL: &str = "ADDL";
    pub const SUBL: &str = "SUBL";
    pub const AND: &str = "AND";
    pub const OR: &str = "OR";
    pub const XOR: &str = "XOR";
    pub const CPA: &str = "CPA";
    pub const CPL: &str = "CPL";
    pub const POP: &str = "POP";
    pub const ST: &str = "ST";
    pub const LAD: &str = "LAD";
    pub const SLA: &str = "SLA";
    pub const SRA: &str = "SRA";
    pub const SLL: &str = "SLL";
    pub const SRL: &str = "SRL";
    pub const JMI: &str = "JMI";
    pub const JNZ: &str = "JNZ";
    pub const JZE: &str = "JZE";
    pub const JUMP: &str = "JUMP";
    pub const JPL: &str = "JPL";
    pub const JOV: &str = "JOV";
    pub const PUSH: &str = "PUSH";
    pub const CALL: &str = "CALL";
    pub const SVC: &str = "SVC";
}

pub enum OpCode {
    DC(u16, Vec<u16>),
    NOP,
    RET,
    LD1W(u8, u8),
    ADDA1W(u8, u8),
    SUBA1W(u8, u8),
    ADDL1W(u8, u8),
    SUBL1W(u8, u8),
    AND1W(u8, u8),
    OR1W(u8, u8),
    XOR1W(u8, u8),
    CPA1W(u8, u8),
    CPL1W(u8, u8),
    POP1W(u8),
    LD2W(u8, u8, u16),
    ST2W(u8, u8, u16),
    LAD2W(u8, u8, u16),
    ADDA2W(u8, u8, u16),
    SUBA2W(u8, u8, u16),
    ADDL2W(u8, u8, u16),
    SUBL2W(u8, u8, u16),
    AND2W(u8, u8, u16),
    OR2W(u8, u8, u16),
    XOR2W(u8, u8, u16),
    CPA2W(u8, u8, u16),
    CPL2W(u8, u8, u16),
    SLA2W(u8, u8, u16),
    SRA2W(u8, u8, u16),
    SLL2W(u8, u8, u16),
    SRL2W(u8, u8, u16),
    JMI2W(u8, u16),
    JNZ2W(u8, u16),
    JZE2W(u8, u16),
    JUMP2W(u8, u16),
    JPL2W(u8, u16),
    JOV2W(u8, u16),
    PUSH2W(u8, u16),
    CALL2W(u8, u16),
    SVC2W(u8, u16),
}

