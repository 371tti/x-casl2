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
