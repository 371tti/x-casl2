use crate::emurator::commet2::prefix::instruction;

pub struct Decoder;

pub trait DecoderExecution {
    type DecResult;
    fn is_2w(val: &[u16; 2]) -> bool;
    fn dec(val: &[u16; 2]) -> Self::DecResult;
}

pub struct DecResult {
    pub w2: bool,
    pub opcode: u8,
    pub r1: u8,
    pub r2: u8,
    pub addr: u16,
}

impl DecoderExecution for Decoder {
    type DecResult = DecResult;

    fn is_2w(val: &[u16; 2]) -> bool {
        let opcode = (val[0] >> 8) as u8; // 上位8ビットをオペコードとして取得
        match opcode {
            instruction::w1::NOP 
            | instruction::w1::LD 
            | instruction::w1::ADDA 
            | instruction::w1::SUBA 
            | instruction::w1::ADDL 
            | instruction::w1::SUBL 
            | instruction::w1::AND 
            | instruction::w1::OR 
            | instruction::w1::XOR 
            | instruction::w1::CPA 
            | instruction::w1::CPL 
            | instruction::w1::POP 
            | instruction::w1::RET => {
                false
            },
            _ => {
                true
            }
        }
    }

    fn dec(val: &[u16; 2]) -> Self::DecResult {
        let opcode = (val[0] >> 8) as u8; // 上位8ビットをオペコードとして取得
        match opcode {
            instruction::w1::NOP
            | instruction::w1::RET => {
                DecResult {
                    w2: false,
                    opcode,
                    r1: 0,
                    r2: 0,
                    addr: 0,
                }
            }
            instruction::w1::LD 
            | instruction::w1::ADDA
            | instruction::w1::SUBA
            | instruction::w1::ADDL
            | instruction::w1::SUBL
            | instruction::w1::AND
            | instruction::w1::OR
            | instruction::w1::XOR
            | instruction::w1::CPA
            | instruction::w1::CPL => {
                DecResult {
                    w2: false,
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: 0,
                }
            }
            instruction::w1::POP => {
                DecResult {
                    w2: false,
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: 0,
                    addr: 0,
                }
            }
            instruction::w2::LD
            | instruction::w2::ST
            | instruction::w2::LAD
            | instruction::w2::ADDA
            | instruction::w2::SUBA
            | instruction::w2::ADDL
            | instruction::w2::SUBL
            | instruction::w2::AND
            | instruction::w2::OR
            | instruction::w2::XOR
            | instruction::w2::CPA
            | instruction::w2::CPL
            | instruction::w2::SLA
            | instruction::w2::SRA
            | instruction::w2::SLL
            | instruction::w2::SRL => {
                DecResult {
                    w2: true,
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            }
            instruction::w2::JMI
            | instruction::w2::JNZ
            | instruction::w2::JZE
            | instruction::w2::JUMP
            | instruction::w2::JPL
            | instruction::w2::JOV
            | instruction::w2::PUSH
            | instruction::w2::CALL
            | instruction::w2::SVC => {
                DecResult {
                    w2: true,
                    opcode,
                    r1: 0, // JMI, JNZ, JZE, JUMP, JPL, JOV, PUSH, CALL, SVC は r1 を使用しない
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            }
            _ => {
                println!("Unknown instruction: {:#04X}", opcode);
                DecResult {
                    w2: false,
                    opcode,
                    r1: 0,
                    r2: 0,
                    addr: 0,
                }
            }
        }
    }
}