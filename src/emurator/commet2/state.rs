pub struct CPUState {
    /// マシンサイクルのカウンタ
    pub cycle: u8,
    /// 汎用レジスタ
    pub gr: GeneralRegister,
    /// プログラムレジスタ
    pub pr: u16,
    /// 生成されたアドレスのレジスタ
    pub gen_addr: u16,
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


impl CPUState {
    /// 新しいCPUの状態を初期化する
    pub fn new() -> Self {
        CPUState {
            cycle: 0,
            gr: GeneralRegister {
                gr0: 0,
                gr1: 0,
                gr2: 0,
                gr3: 0,
                gr4: 0,
                gr5: 0,
                gr6: 0,
                gr7: 0,
            },
            pr: 0,
            gen_addr: 0,
            mar: 0,
            mrr: 0,
            sp: 0xFFFF, // スタックポインタは通常65535で初期化
            memory: Memory([0; 65536]),
            ir: [0, 0],
            fr: [false; 3], // フラグレジスタは全てfalseで初期化
        }
    }
    
}