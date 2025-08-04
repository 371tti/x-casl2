use std::{fmt::Debug, thread::LocalKey};

use crate::emurator::commet2::{alu::{ALUExecution, ALU}, decoder::{DecResult, Decoder, DecoderExecution}, prefix::{decoder_cycle, fetch_cycle, instruction, machine_cycle, opecode_to_4char}};

use super::state::CPUState;

pub struct CPU {
    /// CPUの状態を保持するやつ
    pub state: CPUState,
    /// ALU (算術論理演算ユニット)
    pub alu: ALU,
    /// decoder: 命令デコーダ
    pub decoder: Decoder,
}

pub trait CPUExecution {
    type UpdateNotify;
    /// CPUの初期化
    /// `mode`に応じて、メモリ、レジスタを負の値で埋めるか、ゼロで埋めるかを決定する
    fn init(&mut self, mode: InitMode);
    /// 命令取り出しサイクル
    fn execute_fetch(&mut self) -> Self::UpdateNotify;
    /// 命令解読サイクル
    fn execute_decode(&mut self) -> Self::UpdateNotify;
    /// アドレス生成サイクル
    fn execute_addr_gen(&mut self) -> Self::UpdateNotify;
    /// 命令実行サイクル
    fn execute_execute(&mut self) -> Self::UpdateNotify;
    /// コメットステップ実行
    fn commet2_step(&mut self) -> Self::UpdateNotify;
    /// キャッスルステップ実行
    fn casl_step(&mut self);
}

pub enum UpdateNotify {
    PR(u16),
    SP(u16),
    MAR(u16),
    MDR(u16),
    IR0(u16),
    IR1(u16),
    DECODER([u16; 2]),
    CONTOROLER([char; 4]),
    GENADDR(u16),
    ACCSGR(u8, u16),
    EXEALU(u8, u16, [bool; 3]),
    NONE,
    END,
}

impl Debug for UpdateNotify {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UpdateNotify::PR(val) => write!(f, "SET PR(0x{:04X})", val),
            UpdateNotify::SP(val) => write!(f, "SET SP(0x{:04X})", val),
            UpdateNotify::MAR(val) => write!(f, "SET MAR(0x{:04X})", val),
            UpdateNotify::MDR(val) => write!(f, "SET MDR(0x{:04X})", val),
            UpdateNotify::IR0(val) => write!(f, "SET IR0(0x{:04X})", val),
            UpdateNotify::IR1(val) => write!(f, "SET IR1(0x{:04X})", val),
            UpdateNotify::DECODER(val) => write!(f, "SET DECODER([0x{:04X}, 0x{:04X}])", val[0], val[1]),
            UpdateNotify::CONTOROLER(val) => write!(f, "SET CONTOROLER({:?})", val),
            UpdateNotify::GENADDR(val) => write!(f, "GEN ADDR(0x{:04X})", val),
            UpdateNotify::ACCSGR(r, val) => write!(f, "ACCESS SGR({}, 0x{:04X})", r, val),
            UpdateNotify::EXEALU(r, val, flags) => write!(
                f,
                "EXE ALU(r{}, 0x{:04X}, [OF: {}, CF: {}, ZF: {}])",
                r, val, flags[0], flags[1], flags[2]
            ),
            UpdateNotify::NONE => write!(f, "NONE"),
            UpdateNotify::END => write!(f, "END"),
        }
    }
}

pub enum InitMode {
    NegativeFill,
    ZeroFill,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            state: CPUState::new(),
            alu: ALU,
            decoder: Decoder,
        }
    }
}

impl CPUExecution for CPU {
    type UpdateNotify = UpdateNotify;
    
    fn init(&mut self, mode: InitMode) {
        todo!()
    }
    
    fn execute_fetch(&mut self) -> Self::UpdateNotify {
        let now_fetch_cycle = self.state.step_cycle;
        match now_fetch_cycle {
            fetch_cycle::READ_PR2MAR
            | fetch_cycle::READ_PR2MAR_FOR2W => {
                // プログラムレジスタからメモリアドレスレジスタへアドレスを転送
                self.state.mar = self.state.pr;
                self.state.step_cycle += 1;
                UpdateNotify::MAR(self.state.mar)
            },
            fetch_cycle::READ_MEM2MDR
            | fetch_cycle::READ_MEM2MDR_FOR2W => {
                // メモリからメモリデータレジスタへデータを転送
                self.state.mdr = self.state.memory.0[self.state.mar as usize];
                self.state.step_cycle += 1;
                UpdateNotify::MDR(self.state.mdr)
            },
            fetch_cycle::MDR2IR1 => {
                // メモリデータレジスタの内容を命令レジスタの1番目へ転送
                self.state.ir[0] = self.state.mdr;
                if !Decoder::is_2w(&self.state.ir) {
                    // 1ワード命令の場合、次のマシンサイクルへ進む
                    self.state.next_step_cycle();
                } else {
                    // 2ワード命令の場合、次のフェッチサイクルへ進む
                    self.state.next_line();
                    self.state.step_cycle += 1;
                }
                UpdateNotify::IR0(self.state.ir[0])
            },
            fetch_cycle::MDR2IR2 => {
                // メモリデータレジスタの内容を命令レジスタの2番目へ転送
                self.state.ir[1] = self.state.mdr;
                // 次のマシンサイクルへ進む
                self.state.next_step_cycle();
                UpdateNotify::IR1(self.state.ir[1])
            },
            _ => {
                panic!("Unknown fetch cycle: {}", now_fetch_cycle);
            }
        }
    }
    
    fn execute_decode(&mut self) -> Self::UpdateNotify {
        let now_decode_cycle = self.state.step_cycle;
        match now_decode_cycle {
            decoder_cycle::DECODE => {
                self.state.decoder_state = Decoder::dec(&self.state.ir);
                self.state.step_cycle += 1;
                UpdateNotify::DECODER(self.state.ir)
            },
            decoder_cycle::SYNC_CONTROLLER => {
                let op_chars = opecode_to_4char((self.state.ir[0] >> 8) as u8);
                self.state.next_step_cycle();
                UpdateNotify::CONTOROLER(op_chars)
            },
            _ => {
                panic!("Unknown decode cycle: {}", now_decode_cycle);
            }
        }
    }
    
    fn execute_addr_gen(&mut self) -> Self::UpdateNotify {
        let opcode = self.state.decoder_state.opcode;
        match opcode {
            instruction::w1::LD => {
                // アドレス生成
                self.state.gen_addr = self.state.gr.get(
                    self.state.decoder_state.r2
                );
                self.state.next_step_cycle();
                UpdateNotify::GENADDR(self.state.gen_addr)
            },
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
            | instruction::w2::SRL
            | instruction::w2::SLL
            | instruction::w2::JMI
            | instruction::w2::JNZ
            | instruction::w2::JZE
            | instruction::w2::JUMP
            | instruction::w2::JPL
            | instruction::w2::JOV
            | instruction::w2::CALL => {
                if self.state.decoder_state.r2 == 0 {
                    let gen_addr = self.state.decoder_state.addr;
                    self.state.gen_addr = gen_addr;
                    self.state.next_step_cycle();
                    UpdateNotify::GENADDR(self.state.gen_addr)
                } else {
                    let x = self.state.gr.get(
                        self.state.decoder_state.r2
                    );
                    let addr = self.state.decoder_state.addr;
                    let gen_addr = unsafe {
                        x.unchecked_add(addr)
                    }; 
                    self.state.gen_addr = gen_addr;
                    self.state.next_step_cycle();
                    UpdateNotify::GENADDR(self.state.gen_addr)
                }
            }
            _ => {
                // アドレス生成は不要な命令
                self.state.next_step_cycle();
                UpdateNotify::NONE
            }
        }
    }
    
    fn execute_execute(&mut self) -> Self::UpdateNotify {
        let opcode = self.state.decoder_state.opcode;
        let r1 = self.state.decoder_state.r1;
        let r2 = self.state.decoder_state.r2;
        let gen_addr = self.state.gen_addr;
        let step_cycle = self.state.step_cycle;

        match opcode {
            instruction::w1::NOP => {
                // NOP命令は何もしない
                self.state.next_cycle();
                UpdateNotify::NONE
            },
            instruction::w2::LD => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    }
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.or(self.state.mdr, 0);
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for LD: {}", step_cycle);
                    }
                }
            },
            instruction::w2::ST => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.gr.get(r1);
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    }
                    2 => {
                        // メモリにデータを書き込む
                        self.state.memory.0[self.state.mar as usize] = self.state.mdr;
                        self.state.next_cycle();
                        UpdateNotify::NONE
                    }
                    _ => {
                        panic!("Unknown step cycle for ST: {}", step_cycle);
                    }
                }
            },
            instruction::w2::LAD => {
                // ALUを通してフラグセット&汎用レジスタにデータをセット
                let exers = self.alu.or(self.state.gen_addr, 0);
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            }
            instruction::w2::ADDA => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.adda(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for ADDA: {}", step_cycle);
                    }
                } 
            }
            instruction::w2::SUBA => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.suba(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SUBA: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::ADDL => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.addl(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for ADDL: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::SUBL => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.subl(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SUBL: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::AND => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.and(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for AND: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::OR => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.or(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for OR: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::XOR => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.xor(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for XOR: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::CPA => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.cpa(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for CPA: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::CPL => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.cpl(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for CPL: {}", step_cycle);
                    }
                } 
            },
            instruction::w2::SLA => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.sla(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SLA: {}", step_cycle);
                    }
                }
            },
            instruction::w2::SRA => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.sra(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SRA: {}", step_cycle);
                    }
                }
            },
            instruction::w2::SLL => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.sll(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SLL: {}", step_cycle);
                    }
                }
            },
            instruction::w2::SRL => {
                match step_cycle {
                    0 => {
                        // MARにアドレスをセット
                        self.state.mar = gen_addr;
                        self.state.step_cycle += 1;
                        UpdateNotify::MAR(self.state.mar)
                    },
                    1 => {
                        // MDRにデータをセット
                        self.state.mdr = self.state.memory.0[self.state.mar as usize];
                        self.state.step_cycle += 1;
                        UpdateNotify::MDR(self.state.mdr)
                    },
                    2 => {
                        // ALUを通してフラグセット&汎用レジスタにデータをセット
                        let exers = self.alu.srl(
                            self.state.gr.get(r1),
                            self.state.mdr
                        );
                        self.state.fr = exers.flags;
                        *self.state.gr.get_mut(r1) = exers.result;
                        self.state.next_cycle();
                        UpdateNotify::EXEALU(r1, exers.result, exers.flags)
                    }
                    _ => {
                        panic!("Unknown step cycle for SRL: {}", step_cycle);
                    }
                }
            },
            instruction::w1::ADDA => {
                // ALUを通してフラグセット&汎用レジスタにデータをセット
                let exers = self.alu.adda(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::SUBA => {
                let exers = self.alu.suba(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::ADDL => {
                let exers = self.alu.addl(
                                        self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::SUBL => {
                let exers = self.alu.subl(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::AND => {
                let exers = self.alu.and(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::OR => {
                let exers = self.alu.or(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::XOR => {
                let exers = self.alu.xor(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::CPA => {
                let exers = self.alu.cpa(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w1::CPL => {
                let exers = self.alu.cpl(
                    self.state.gr.get(r1),
                    self.state.gr.get(r2)
                );
                self.state.fr = exers.flags;
                *self.state.gr.get_mut(r1) = exers.result;
                self.state.next_cycle();
                UpdateNotify::EXEALU(r1, exers.result, exers.flags)
            },
            instruction::w2::JMI => {
                // MAR から PR へ
                let fr = self.state.fr;
                if fr[1] == true{
                    self.state.pr = gen_addr;
                    self.state.machine_cycle = machine_cycle::FETCH;
                    UpdateNotify::PR(self.state.pr)
                } else {
                    self.state.next_cycle();
                    UpdateNotify::NONE
                }
            },
            instruction::w2::JNZ => {
                // MAR から PR へ
                let fr = self.state.fr;
                if fr[2] == false {
                    self.state.pr = gen_addr;
                    self.state.machine_cycle = machine_cycle::FETCH;
                    UpdateNotify::PR(self.state.pr)
                } else {
                    self.state.next_cycle();
                    UpdateNotify::NONE
                }
            },
            instruction::w2::JZE => {
                // MAR から PR へ
                let fr = self.state.fr;
                if fr[2] == true {
                    self.state.pr = gen_addr;
                    self.state.machine_cycle = machine_cycle::FETCH;
                    UpdateNotify::PR(self.state.pr)
                } else {
                    self.state.next_cycle();
                    UpdateNotify::NONE
                }
            },
            instruction::w2::JUMP => {
                // MAR から PR へ
                self.state.pr = gen_addr;
                self.state.machine_cycle = machine_cycle::FETCH;
                UpdateNotify::PR(self.state.pr)
            },
            instruction::w2::JPL => {
                // MAR から PR へ
                let fr = self.state.fr;
                if fr[1] == false && fr[2] == false {
                    self.state.pr = gen_addr;
                    self.state.machine_cycle = machine_cycle::FETCH;
                    UpdateNotify::PR(self.state.pr)
                } else {
                    self.state.next_cycle();
                    UpdateNotify::NONE
                }
            },
            instruction::w2::JOV => {
                // MAR から PR へ
                let fr = self.state.fr;
                if fr[0] == true {
                    self.state.pr = gen_addr;
                    self.state.machine_cycle = machine_cycle::FETCH;
                    UpdateNotify::PR(self.state.pr)
                } else {
                    self.state.next_cycle();
                    UpdateNotify::NONE
                }
            },
            // instruction::w2::CALL => {
            //     // MAR から PR へ
            // スタックにアドレス積んでない！！！！
            //     self.state.pr = gen_addr;
            //     self.state.machine_cycle = machine_cycle::FETCH;
            //     UpdateNotify::PR(self.state.pr)
            // },
            _ => {
                println!("Unknown opcode: {}", opcode);
                self.state.next_cycle();
                UpdateNotify::NONE
            }
        }

    }
    
    fn commet2_step(&mut self) -> Self::UpdateNotify {
        let now_machine_cycle = self.state.machine_cycle;
        match now_machine_cycle {
            machine_cycle::FETCH => {
                self.execute_fetch()
            }
            machine_cycle::DECODE => {
                self.execute_decode()
            }
            machine_cycle::ADDR_GEN => {
                self.execute_addr_gen()
            }
            machine_cycle::EXECUTE => {
                self.execute_execute()
            }
            machine_cycle::END => {
                UpdateNotify::END
            }
            _ => {
                panic!("Unknown machine cycle: {}", now_machine_cycle);
            }
        }
    }
    
    fn casl_step(&mut self) {
        loop {
            self.commet2_step();
            if self.state.machine_cycle == machine_cycle::FETCH {
                break;
            }
        }
    }

    
}