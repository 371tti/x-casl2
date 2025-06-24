use crate::emurator::commet2::{alu::ALU, decoder::{Decoder, DecoderExecution}, prefix::{decoder_cycle, fetch_cycle, machine_cycle, opecode_to_4char}};

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
    fn castle_step(&mut self) -> Self::UpdateNotify;
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
    EXEALU(u16, [bool; 3]),
    END,
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
                self.state.mrr = self.state.memory.0[self.state.mar as usize];
                self.state.step_cycle += 1;
                UpdateNotify::MDR(self.state.mrr)
            },
            fetch_cycle::MDR2IR1 => {
                // メモリデータレジスタの内容を命令レジスタの1番目へ転送
                self.state.ir[0] = self.state.mrr;
                if !(Decoder::dec_1w(self.state.mrr).need_2w) {
                    // 1ワード命令の場合、次のマシンサイクルへ進む
                    self.state.machine_cycle = machine_cycle::DECODE;
                } else {
                    // 2ワード命令の場合、次のフェッチサイクルへ進む
                    self.state.step_cycle += 1;
                }
                UpdateNotify::IR0(self.state.ir[0])
            },
            fetch_cycle::MDR2IR2 => {
                // メモリデータレジスタの内容を命令レジスタの2番目へ転送
                self.state.ir[1] = self.state.mrr;
                // 次のマシンサイクルへ進む
                self.state.machine_cycle = machine_cycle::DECODE;
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
                // 実際には何もしなくて良さそう
                self.state.step_cycle += 1;
                UpdateNotify::DECODER(self.state.ir)
            },
            decoder_cycle::SYNC_CONTROLLER => {
                let op_chars = opecode_to_4char((self.state.ir[0] >> 8) as u8);
                self.state.machine_cycle = machine_cycle::ADDR_GEN;
                UpdateNotify::CONTOROLER(op_chars)
            },
            _ => {
                panic!("Unknown decode cycle: {}", now_decode_cycle);
            }
        }
    }
    
    fn execute_addr_gen(&mut self) -> Self::UpdateNotify {
        
    }
    
    fn execute_execute(&mut self) -> Self::UpdateNotify {
        todo!()
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
    
    fn castle_step(&mut self) -> Self::UpdateNotify {
        todo!()
    }

    
}