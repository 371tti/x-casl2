use crate::prefix::MEMORY_SIZE;

pub struct Decoder<'a> {
    m: &'a [u16; MEMORY_SIZE],
    addr: u16,
}

impl<'a> Decoder<'a> {
    pub const TOP_BYTE_MASK: u16 = 0xFF00;
    pub const LOW_BYTE_MASK: u16 = 0x00FF;
    pub const R1_MASK: u16 = 0x00F0;
    pub const R2_MASK: u16 = 0x000F;
    pub const X_MASK: u16 = 0x000F;

    pub fn new(m: &'a [u16; MEMORY_SIZE], addr: u16) -> Self {
        Self { m, addr }
    }

    pub fn word1(&self) -> u16 {
        self.m[self.addr as usize]
    }
    pub fn word2(&self) -> u16 {
        self.m[(self.addr + 1) as usize]
    }

    pub fn opecode(&self) -> u8 {
        let word = self.word1();
        ((word & Self::TOP_BYTE_MASK) >> 8) as u8
    }

    pub fn decode(&self) -> Instruction {
        let opecode = self.opecode();
        match opecode {
            // NOP
            0x00 => Instruction::NOP,
            // LD 2W
            0x10 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::LD(OpeCode::RAddr { r, addr, x })
            }
            // ST 2W
            0x11 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::ST(OpeCode::RAddr { r, addr: addr, x })
            }
            // LAD 2W
            0x12 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::LAD(OpeCode::RAddr { r, addr: addr, x })
            }
            // LD 1W
            0x14 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::LD(OpeCode::R2 { r1, r2 })
            }
            // ADDA 2W
            0x20 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::ADDA(OpeCode::RAddr { r, addr: addr, x })
            }
            // SUBA 2W
            0x21 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SUBA(OpeCode::RAddr { r, addr: addr, x })
            }
            // ADDL 2W
            0x22 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::ADDL(OpeCode::RAddr { r, addr: addr, x })
            }
            // SUBL 2W
            0x23 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SUBL(OpeCode::RAddr { r, addr: addr, x })
            }
            // ADDA 1W
            0x24 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::ADDA(OpeCode::R2 { r1, r2 })
            }
            // SUBA 1W
            0x25 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::SUBA(OpeCode::R2 { r1, r2 })
            }
            // ADDL 1W
            0x26 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::ADDL(OpeCode::R2 { r1, r2 })
            }
            // SUBL 1W
            0x27 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::SUBL(OpeCode::R2 { r1, r2 })
            }
            // AND 2W
            0x30 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::AND(OpeCode::RAddr { r, addr: addr, x })
            }
            // OR 2W
            0x31 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::OR(OpeCode::RAddr { r, addr: addr, x })
            }
            // XOR 2W
            0x32 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::XOR(OpeCode::RAddr { r, addr: addr, x })
            }
            // AND 1W
            0x34 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::AND(OpeCode::R2 { r1, r2 })
            }
            // OR 1W
            0x35 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::OR(OpeCode::R2 { r1, r2 })
            }
            // XOR 1W
            0x36 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::XOR(OpeCode::R2 { r1, r2 })
            }
            // CPA 2W
            0x40 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::CPA(OpeCode::RAddr { r, addr: addr, x })
            }
            // CPL 2W
            0x41 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::CPL(OpeCode::RAddr { r, addr: addr, x })
            }
            // CPA 1W
            0x44 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::CPA(OpeCode::R2 { r1, r2 })
            }
            // CPL 1W
            0x45 => {
                let r1 = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let r2 = (self.word1() & Self::R2_MASK) as u8;
                Instruction::CPL(OpeCode::R2 { r1, r2 })
            }
            // SLA
            0x50 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SLA(OpeCode::RAddr { r, addr: addr, x })
            }
            // SRA
            0x51 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SRA(OpeCode::RAddr { r, addr: addr, x })
            }
            // SLL
            0x52 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SLL(OpeCode::RAddr { r, addr: addr, x })
            }
            // SRL
            0x53 => {
                let r = ((self.word1() & Self::R1_MASK) >> 4) as u8;
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SRL(OpeCode::RAddr { r, addr: addr, x })
            }
            // JMI
            0x61 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JMI(OpeCode::Addr { addr: addr, x })
            }
            // JNZ
            0x62 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JNZ(OpeCode::Addr { addr: addr, x })
            }
            // JZE
            0x63 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JZE(OpeCode::Addr { addr: addr, x })
            }
            // JUMP
            0x64 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JUMP(OpeCode::Addr { addr: addr, x })
            }
            // JPL
            0x65 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JPL(OpeCode::Addr { addr: addr, x })
            }
            // JOV
            0x66 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::JOV(OpeCode::Addr { addr: addr, x })
            }

            // PUSH
            0x70 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::PUSH(OpeCode::Addr { addr: addr, x })
            }
            // POP
            0x71 => {
                let r = (self.word1() & Self::R1_MASK >> 4) as u8;
                Instruction::POP(OpeCode::R { r })
            }
            // CALL
            0x80 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::CALL(OpeCode::Addr { addr: addr, x })
            }
            // RET
            0x81 => Instruction::RET,

            // その他命令
            // IN
            0x90 => todo!(),
            // OUT
            0x91 => todo!(),
            // RPUSH
            0xA0 => todo!(),
            // RPOP
            0xA1 => todo!(),

            // SVC
            0xF0 => {
                let addr = self.word2();
                let x = (self.word1() & Self::X_MASK) as u8;
                Instruction::SVC(OpeCode::Addr { addr: addr, x })
            }

            _ => {
                panic!("Unsupported opcode: {:02X}", self.opecode());
            }
        }
    }
}

/// COMET2 命令セット
#[derive(Debug)]
pub enum Instruction {
    /// なにもしない
    NOP,
    /// 1. r1 <- (r2)
    /// 2. r <- (addr)
    LD(OpeCode),
    /// <addr + x> <- (r)
    ST(OpeCode),
    /// r1 <- addr + x
    LAD(OpeCode),
    /// 1. r1 <- r1 + r2
    /// 2. r <- (r) + (addr + x)
    ADDA(OpeCode),
    /// 1. r1 <- r1 + r2 +L
    /// 2. r <- (r) + (addr + x) +L
    ADDL(OpeCode),
    /// 1. r1 <- r1 - r2
    /// 2. r <- (r) - (addr + x)
    SUBA(OpeCode),
    /// 1. r1 <- r1 - r2 -L
    /// 2. r <- (r) - (addr + x) -L
    SUBL(OpeCode),
    /// 1. r1 <- r1 AND r2
    /// 2. r <- (r) AND (addr + x)
    AND(OpeCode),
    /// 1. r1 <- r1 OR r2
    /// 2. r <- (r) OR (addr + x)
    OR(OpeCode),
    /// 1. r1 <- r1 XOR r2
    /// 2. r <- (r) XOR (addr + x)
    XOR(OpeCode),
    /// 1. Compare (r1) with (r2)
    /// 2. Compare (r) with (addr + x)
    CPA(OpeCode),
    /// 1. Compare (r1) with (r2) -L
    /// 2. Compare (r) with (addr + x) -L
    CPL(OpeCode),
    /// r <- (r) << (addr + x)
    SLA(OpeCode),
    /// r <- (r) >> (addr + x)
    SRA(OpeCode),
    /// r <- (r) << (addr + x) +L
    SLL(OpeCode),
    /// r <- (r) >> (addr + x) +L
    SRL(OpeCode),
    /// if SF=0 & ZF=0 then goto addr + x
    JPL(OpeCode),
    /// if SF=1 then goto addr + x
    JMI(OpeCode),
    /// if ZF=0 then goto addr + x
    JNZ(OpeCode),
    /// if ZF=1 then goto addr + x
    JZE(OpeCode),
    /// if OF=1 then goto addr + x
    JOV(OpeCode),
    /// goto addr + x
    JUMP(OpeCode),
    /// SP <- (SP) -L 1, (SP) <- adr + x
    PUSH(OpeCode),
    /// r <- ((SP)), (SP) <- (SP) +L 1
    POP(OpeCode),
    /// SP <- (SP) -L 1, (SP) <- (PR), PR <- addr + x
    CALL(OpeCode),
    /// PR <- ((SP)), (SP) <- (SP) +L 1
    RET,
    SVC(OpeCode),
}

/// COMET2 命令のオペコード
#[derive(Debug)]
pub enum OpeCode {
    /// r1, r2
    R2 { r1: u8, r2: u8 },
    /// r, addr[,x]
    RAddr { r: u8, addr: u16, x: u8 },
    /// addr[,x]
    Addr { addr: u16, x: u8 },
    /// r
    R { r: u8 },
}

