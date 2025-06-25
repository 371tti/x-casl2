use crate::emurator::commet2::{decoder::DecResult, prefix::machine_cycle};

pub struct CPUState {
    /// マシンサイクルのカウンタ
    pub machine_cycle: u8,
    /// 各ステップのサイクルカウンタ
    pub step_cycle: u8,
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
    /// デコーダーのデコード結果
    pub decoder_state: DecResult,
}

impl CPUState {
    pub fn next_line(&mut self) {
        // プログラムレジスタを次のアドレスに進める
        self.pr = unsafe {
            self.pr.unchecked_add(1)
        };
    }

    pub fn next_cycle(&mut self) {
        self.pr = unsafe {
            self.pr.unchecked_add(1)
        };
        self.machine_cycle = machine_cycle::FETCH; // マシンサイクルはフェッチにリセット
        self.step_cycle = 0; // 各ステップのサイクルはリセット
    }

    pub fn next_step_cycle(&mut self) {
        self.machine_cycle = unsafe {
            self.machine_cycle.unchecked_add(1)
        };
        self.step_cycle = 0; // 各ステップのサイクルはリセット
    }
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

impl GeneralRegister {
    /// 新しい汎用レジスタを初期化する
    pub fn get(&self, index: u8) -> u16 {
        match index {
            0 => self.gr0,
            1 => self.gr1,
            2 => self.gr2,
            3 => self.gr3,
            4 => self.gr4,
            5 => self.gr5,
            6 => self.gr6,
            7 => self.gr7,
            _ => panic!("Invalid general register index"),
        }
    }

    pub fn get_mut(&mut self, index: u8) -> &mut u16 {
        match index {
            0 => &mut self.gr0,
            1 => &mut self.gr1,
            2 => &mut self.gr2,
            3 => &mut self.gr3,
            4 => &mut self.gr4,
            5 => &mut self.gr5,
            6 => &mut self.gr6,
            7 => &mut self.gr7,
            _ => panic!("Invalid general register index"),
        }
    }
}

/// メインメモリ
pub struct Memory(pub [u16; 65536]);


impl CPUState {
    /// 新しいCPUの状態を初期化する
    pub fn new() -> Self {
        CPUState {
            machine_cycle: 0,
            step_cycle: 0,
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
            decoder_state: DecResult {
                w2: false,
                opcode: 0,
                r1: 0,
                r2: 0,
                addr: 0,
            }
        }
    }
    
}