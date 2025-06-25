use crate::emurator::commet2::cpu::CPUExecution;
use std::io::{self, Write};
use std::{thread, time};

pub mod emurator;

fn main() {
    let mut commet2 = emurator::commet2::cpu::CPU::new();
    let mut write_memory: [u16; 65536] = [0; 65536];
    write_memory[0] = 0x1200;
    write_memory[1] = 0x0001;
    write_memory[2] = 0x2410;
    write_memory[3] = 0x6400;
    write_memory[4] = 0x0002;
    // 必要に応じて他の値もwrite_memory[n] = ...;で追加
    commet2.state.memory.0.copy_from_slice(&write_memory);
    commet2.state.pr = 0x0000; // プログラムレジスタを初期化
    commet2.state.sp = 0x0000; // スタックポイン

    loop {
        let res = commet2.castle_step();
        // 画面をクリアしてから状態を表示（1ms周期で表示のみ）
        use std::sync::OnceLock;
        use std::sync::RwLock;
        static LAST_DISPLAY: OnceLock<RwLock<std::time::Instant>> = OnceLock::new();
        let now = std::time::Instant::now();
        let last_display = LAST_DISPLAY.get_or_init(|| RwLock::new(std::time::Instant::now()));
        {
            let last_display_read = last_display.read().unwrap();
            if now.duration_since(*last_display_read) >= time::Duration::from_millis(1) {
                drop(last_display_read); // Release read lock before acquiring write lock
                print!("\x1B[2J\x1B[1;1H");
                commet2.state.display_state();
                io::stdout().flush().unwrap();
                let mut last_display_write = last_display.write().unwrap();
                *last_display_write = now;
            }
        }
        // castle_stepはできるだけ高速に回す
    }
}
