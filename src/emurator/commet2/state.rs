pub struct CPUState {
    /// マシンサイクルのカウンタ
    pub cycle: u8,
    /// 汎用レジスタ
    pub gr: GeneralRegister,
    /// プログラムレジスタ
    pub pr: u16,
    /// メモリアドレスレジスタ
    pub mar: u16,
    /// メモリリードレジスタ
    pub mrr: u16,
    /// スタックポインタ
    pub sp: u16,
    /// メモリ
    pub memory: Memory,
    /// 命令レジスタ
    pub ir: [u16; 2],
    /// フラグレジスタ
    pub fr: [bool; 3],
}

/// 汎用レジスタの構造体
pub struct GeneralRegister {
    pub gr0: u16,
    pub gr1: u16,
    pub gr2: u16,
    pub gr3: u16,
    pub gr4: u16,
    pub gr5: u16,
    pub gr6: u16,
    pub gr7: u16,
}

/// メインメモリ
pub struct Memory(pub [u16; 65536]);