use crate::emurator::commet2::prefix::instruction;

pub struct Decoder;

pub trait DecoderExecution {
    type Dec1wResult;
    type Dec2wResult;
    fn dec_1w(val: u16) -> Self::Dec1wResult;
    fn dec_2w(val: &[u16; 2]) -> Self::Dec2wResult;
}

pub struct Dec1wResult {
    pub opcode: u8,
    pub r1: u8,
    pub r2: u8,
    pub need_2w: bool,
}

impl Dec1wResult {
    pub fn need_2w() -> Self {
        Dec1wResult {opcode: 0, r1: 0, r2: 0, need_2w: true,}
    }
}

pub struct Dec2wResult {
    pub opcode: u8,
    pub r1: u8,
    pub r2: u8,
    pub addr: u16,
}

impl DecoderExecution for Decoder {
    type Dec1wResult = Dec1wResult;
    type Dec2wResult = Dec2wResult;

    /// 1ワード命令のデコード
    fn dec_1w(val: u16) -> Self::Dec1wResult {
        let opcode = (val >> 8) as u8; // 上位8ビットをオペコードとして取得
        let result = match opcode {
            instruction::w1::NOP => {
                Dec1wResult { opcode, r1: 0, r2: 0, need_2w: false }
            },
            instruction::w1::LD => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::ADDA => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::SUBA => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::ADDL => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::SUBL => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::AND => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::OR => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::XOR => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::CPA => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::CPL => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: (val & 0x0F) as u8,
                    need_2w: false,
                }
            },
            instruction::w1::POP => {
                Dec1wResult {
                    opcode,
                    r1: ((val >> 4) & 0x0F) as u8,
                    r2: 0,
                    need_2w: false,
                }
            },
            instruction::w1::RET => {
                Dec1wResult {
                    opcode,
                    r1: 0,
                    r2: 0,
                    need_2w: false,
                }
            },
            _ => {
                Dec1wResult::need_2w()
            }
        };
        result
    }

    /// 2ワード命令のデコード
    fn dec_2w(val: &[u16; 2]) -> Self::Dec2wResult {
        let opcode = (val[0] >> 8) as u8; // 上位8ビットをオペコードとして取得
        match opcode {
            instruction::w2::LD => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::ST => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::LDA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::ADDA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SUBA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::ADDL => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SUBL => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::AND => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::OR => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::XOR => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::CPA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::CPL => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SLA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SRA => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SLL => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SRL => {
                Dec2wResult {
                    opcode,
                    r1: ((val[0] >> 4) & 0x0F) as u8,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JMI => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JNZ => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JZE => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JUMP => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JPL => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::JOV => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::PUSH => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::CALL => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            instruction::w2::SVC => {
                Dec2wResult {
                    opcode,
                    r1: 0,
                    r2: (val[0] & 0x0F) as u8,
                    addr: val[1],
                }
            },
            _ => {
                println!("Unknown 2-word instruction: {:#04X}", opcode);
                Dec2wResult {
                    opcode: 0,
                    r1: 0,
                    r2: 0,
                    addr: 0,
                }
            }
        }

    }
}