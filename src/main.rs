use crate::emurator::commet2::cpu::CPUExecution;

pub mod emurator;

fn main() {
    let mut commet2 = emurator::commet2::cpu::CPU::new();
    let mut write_memory: [u16; 65536] = [0; 65536];
    write_memory[0] = 0x1200;
    write_memory[1] = 0x0001;
    write_memory[2] = 0x2410;
    write_memory[3] = 0x6400;
    write_memory[4] = 0x0000;
    // 必要に応じて他の値もwrite_memory[n] = ...;で追加
    commet2.state.memory.0.copy_from_slice(&write_memory);
    commet2.state.pr = 0x0000; // プログラムレジスタを初期化
    commet2.state.sp = 0x0000; // スタックポイン

    loop {
        let res = commet2.commet2_step();
        println!("Result: {:?}", res);
    }
}
