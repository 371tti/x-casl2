pub struct CPUState {
    /// マシンサイクルのカウンタ
    pub cycle: u8,
    /// 汎用レジスタ
    pub GR: GeneralResister,
    /// プログラムレジスタ 
    pub PR: u16,
    /// メモリアドレスレジスタ
    pub MAR: u16,
    /// メモリリードレジスタ
    pub MRR: u16,
    /// スタックポインタ
    pub SP: u16,
    /// メモリ
    pub memory: Memory,
    /// 命令レジスタ
    pub IR: [u16; 2],
    /// フラグレジスタ
    pub FR: [bool; 3],
}

/// 汎用レジスタの構造体
pub struct GeneralResister {
    pub GR0: u16,
    pub GR1: u16,
    pub GR2: u16,
    pub GR3: u16,
    pub GR4: u16,
    pub GR5: u16,
    pub GR6: u16,
    pub GR7: u16,
}

/// メインメモリ
pub struct Memory(pub [u16; 65536]);