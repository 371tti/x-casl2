use std::fmt::Display;

use x_casl2::instruction::{Decoder, Instruction, OpeCode};
use x_casl2::prefix::{MEMORY_SIZE, REGISTER_NUM, STACK_START};

fn main() {
    let mut comet2 = COMET2::new();
    comet2.m[0] = 0x1200; // LAD GR0
    comet2.m[1] = 0x0001; // 0x0001
    comet2.m[2] = 0x1210; // LAD GR1
    comet2.m[3] = 0x0000; // 0x0000
    comet2.m[4] = 0x2410; // ADDA GR1, GR0
    comet2.m[5] = 0x6600; // JOV
    comet2.m[6] = 0xFF00; // 0xFF00
    comet2.m[7] = 0x6400; // JUMP
    comet2.m[8] = 0x0004; // 0x0004
    println!("{}", comet2);
    let mut step_count = 0;
    loop {
        comet2.step();
        step_count += 1;
        if step_count % 1000 == 0 {
            println!("After {} steps:", step_count);
            println!("{}", comet2);
        }
    }
}

pub struct COMET2 {
    /// registers
    r: [u16; REGISTER_NUM],
    /// memory
    m: [u16; MEMORY_SIZE],
    /// stack pointer
    sp: u16,
    /// program counter
    pc: u16,
    /// arithmetic logic unit
    alu: ALU,
}

impl Display for COMET2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PC: {:04X} SP: {:04X}", self.pc, self.sp)?;
        writeln!(f, "ALU Flags - Z: {} O: {} S: {}", self.alu.z, self.alu.o, self.alu.s)?;
        for i in 0..REGISTER_NUM {
            writeln!(f, "GR{}: {:04X}", i, self.r[i])?;
        }
        Ok(())
    }
}

impl COMET2 {
    pub fn new() -> Self {
        Self {
            r: [0xFFFF; REGISTER_NUM],
            m: [0xFFFF; MEMORY_SIZE],
            sp: STACK_START,
            pc: 0,
            alu: ALU {
                z: false,
                o: false,
                s: false,
            },
        }
    }

    pub fn effective_address(&self, addr: u16, x: u8) -> u16 {
        match x {
            0 => addr,
            _ => addr.wrapping_add(self.r[x as usize]),
        }
    }

    pub fn step(&mut self) {
        let decoder = Decoder::new(&self.m, self.pc);
        let instruction = decoder.decode();
        match instruction {
            Instruction::NOP => self.pc += 1,
            Instruction::LD(OpeCode::R2 { r1, r2 }) => {
                self.r[r1 as usize] = self.r[r2 as usize];
                self.pc += 1;
            }
            Instruction::LD(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.r[r as usize] = self.m[effective_addr as usize];
                self.pc += 2;
            }
            Instruction::ST(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.m[effective_addr as usize] = self.r[r as usize];
                self.pc += 2;
            }
            Instruction::LAD(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.r[r as usize] = effective_addr;
                self.pc += 2;
            }
            Instruction::ADDA(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.adda(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::ADDA(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .adda(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::ADDL(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.addl(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::ADDL(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .addl(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::SUBA(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.suba(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::SUBA(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .suba(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::SUBL(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.subl(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::SUBL(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .subl(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::AND(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.and(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::AND(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .and(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::OR(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.or(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::OR(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .or(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::XOR(OpeCode::R2 { r1, r2 }) => {
                let result = self.alu.xor(self.r[r1 as usize], self.r[r2 as usize]);
                self.r[r1 as usize] = result;
                self.pc += 1;
            }
            Instruction::XOR(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let result = self
                    .alu
                    .xor(self.r[r as usize], self.m[effective_addr as usize]);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::CPA(OpeCode::R2 { r1, r2 }) => {
                self.alu.cpa(self.r[r1 as usize], self.r[r2 as usize]);
                self.pc += 1;
            }
            Instruction::CPA(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.alu
                    .cpa(self.r[r as usize], self.m[effective_addr as usize]);
                self.pc += 2;
            }
            Instruction::CPL(OpeCode::R2 { r1, r2 }) => {
                self.alu.cpl(self.r[r1 as usize], self.r[r2 as usize]);
                self.pc += 1;
            }
            Instruction::CPL(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.alu
                    .cpl(self.r[r as usize], self.m[effective_addr as usize]);
                self.pc += 2;
            }
            Instruction::SLA(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let n = self.m[effective_addr as usize] as u8;
                let result = self.alu.sla(self.r[r as usize], n);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::SRA(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let n = self.m[effective_addr as usize] as u8;
                let result = self.alu.sra(self.r[r as usize], n);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::SLL(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let n = self.m[effective_addr as usize] as u8;
                let result = self.alu.sll(self.r[r as usize], n);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::SRL(OpeCode::RAddr { r, addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                let n = self.m[effective_addr as usize] as u8;
                let result = self.alu.srl(self.r[r as usize], n);
                self.r[r as usize] = result;
                self.pc += 2;
            }
            Instruction::JPL(OpeCode::Addr { addr, x }) => {
                if !self.alu.s && !self.alu.z {
                    let effective_addr = self.effective_address(addr, x);
                    self.pc = effective_addr;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JMI(OpeCode::Addr { addr, x }) => {
                if self.alu.s {
                    let effective_addr = self.effective_address(addr, x);
                    self.pc = effective_addr;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JNZ(OpeCode::Addr { addr, x }) => {
                if !self.alu.z {
                    let effective_addr = self.effective_address(addr, x);
                    self.pc = effective_addr;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JZE(OpeCode::Addr { addr, x }) => {
                if self.alu.z {
                    let effective_addr = self.effective_address(addr, x);
                    self.pc = effective_addr;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JOV(OpeCode::Addr { addr, x }) => {
                if self.alu.o {
                    let effective_addr = self.effective_address(addr, x);
                    self.pc = effective_addr;
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JUMP(OpeCode::Addr { addr, x }) => {
                let effective_addr = self.effective_address(addr, x);
                self.pc = effective_addr;
            }
            Instruction::PUSH(OpeCode::Addr { addr, x }) => {
                self.sp = self.sp.wrapping_sub(1);
                let effective_addr = self.effective_address(addr, x);
                self.m[self.sp as usize] = self.m[effective_addr as usize];
                self.pc += 2;
            }
            Instruction::POP(OpeCode::R { r }) => {
                self.r[r as usize] = self.m[self.sp as usize];
                self.sp = self.sp.wrapping_add(1);
                self.pc += 1;
            }
            Instruction::CALL(OpeCode::Addr { addr, x }) => {
                self.sp = self.sp.wrapping_sub(1);
                self.m[self.sp as usize] = self.pc + 2;
                let effective_addr = self.effective_address(addr, x);
                self.pc = effective_addr;
            }
            Instruction::RET => {
                self.pc = self.m[self.sp as usize];
                self.sp = self.sp.wrapping_add(1);
            }
            Instruction::SVC(_ope_code) => panic!("SVC instruction not implemented!!"),
            _ => panic!("undefined instruction!!"),
        }
    }
}

pub struct ALU {
    /// zero flag
    z: bool,
    /// overflow flag
    o: bool,
    /// sign flag
    s: bool,
}

impl ALU {
    pub const TOP_BIT: u16 = 0x8000;
    pub fn adda(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_add(b);
        self.z = result == 0;
        // != のほうが若干早いんだよ？？
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = overflow;
        result
    }

    pub fn addl(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_add(b);
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = overflow;
        result
    }

    pub fn suba(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_sub(b);
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = overflow;
        result
    }

    pub fn subl(&mut self, a: u16, b: u16) -> u16 {
        let (result, overflow) = a.overflowing_sub(b);
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = overflow;
        result
    }

    pub fn and(&mut self, a: u16, b: u16) -> u16 {
        let result = a & b;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn or(&mut self, a: u16, b: u16) -> u16 {
        let result = a | b;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn xor(&mut self, a: u16, b: u16) -> u16 {
        let result = a ^ b;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn cpa(&mut self, a: u16, b: u16) {
        let (result, overflow) = (a as i16).overflowing_sub(b as i16);
        self.z = result as u16 == 0;
        self.s = (result as u16 & Self::TOP_BIT) != 0;
        self.o = overflow;
    }

    pub fn cpl(&mut self, a: u16, b: u16) {
        let (result, overflow) = a.overflowing_sub(b);
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = overflow;
    }

    pub fn sla(&mut self, a: u16, n: u8) -> u16 {
        let result = a << n;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn sra(&mut self, a: u16, n: u8) -> u16 {
        let result = a >> n;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn sll(&mut self, a: u16, n: u8) -> u16 {
        let result = a << n;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }

    pub fn srl(&mut self, a: u16, n: u8) -> u16 {
        let result = a >> n;
        self.z = result == 0;
        self.s = (result & Self::TOP_BIT) != 0;
        self.o = false;
        result
    }
}
