use crate::emurator::commet2::{alu::ALU, decoder::Decoder};

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
        todo!()
    }
    
    fn execute_decode(&mut self) -> Self::UpdateNotify {
        todo!()
    }
    
    fn execute_addr_gen(&mut self) -> Self::UpdateNotify {
        todo!()
    }
    
    fn execute_execute(&mut self) -> Self::UpdateNotify {
        todo!()
    }
    
    fn commet2_step(&mut self) -> Self::UpdateNotify {
        todo!()
    }
    
    fn castle_step(&mut self) -> Self::UpdateNotify {
        todo!()
    }

    
}