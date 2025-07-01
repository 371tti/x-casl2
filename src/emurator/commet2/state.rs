use crate::emurator::commet2::{decoder::DecResult, prefix::{machine_cycle, opecode_to_4char}};

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
    pub mdr: u16,
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

impl CPUState {
    pub fn display_state(&self) {
        println!(
            "Machine Cycle: {}\n\
            Step Cycle: {}\n\
            General Registers: GR0: {}, GR1: {}, GR2: {}, GR3: {}, GR4: {}, GR5: {}, GR6: {}, GR7: {}\n\
            Program Register (PR): {}\n\
            Generated Address (GenAddr): {}\n\
            Memory Address Register (MAR): {}\n\
            Memory Read Register (MDR): {}\n\
            Stack Pointer (SP): {}\n\
            Instruction Register (IR): [{}, {}]\n\
            Controler Mode: {:?}\n\
            Flags Register (FR): [OF: {}, SF: {}, ZF: {}]\n\n",
            self.machine_cycle,
            self.step_cycle,
            self.gr.gr0, self.gr.gr1, self.gr.gr2, self.gr.gr3, self.gr.gr4, self.gr.gr5, self.gr.gr6, self.gr.gr7,
            self.pr,
            self.gen_addr,
            self.mar,
            self.mdr,
            self.sp,
            self.ir[0], self.ir[1],
            opecode_to_4char(self.decoder_state.opcode),
            self.fr[0], self.fr[1], self.fr[2]
        );
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
            mdr: 0,
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