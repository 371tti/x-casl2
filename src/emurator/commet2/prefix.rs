pub mod machine_cycle {
    pub const FETCH: u8 = 0x00;
    pub const DECODE: u8 = 0x01;
    pub const ADDR_GEN: u8 = 0x02;
    pub const EXECUTE: u8 = 0x03;
    pub const END: u8 = 0xFF;
}

pub mod fetch_cycle {
    pub const READ_PR2MAR: u8 = 0x00;
    pub const READ_MEM2MDR: u8 = 0x01;
    pub const MDR2IR1: u8 = 0x02;
    pub const READ_PR2MAR_FOR2W: u8 = 0x03;
    pub const READ_MEM2MDR_FOR2W: u8 = 0x04;
    pub const MDR2IR2: u8 = 0x05;
}

pub mod decoder_cycle {
    pub const DECODE: u8 = 0x00;
    pub const SYNC_CONTROLLER: u8 = 0x01;
}

pub mod instruction {
    pub mod w1 {
        pub const NOP: u8 = 0x00;
        pub const LD: u8 = 0x14;

        pub const ADDA: u8 = 0x24;
        pub const SUBA: u8 = 0x25;
        pub const ADDL: u8 = 0x26;
        pub const SUBL: u8 = 0x27;

        pub const AND: u8 = 0x34;
        pub const OR: u8 = 0x35;
        pub const XOR: u8 = 0x36;

        pub const CPA: u8 = 0x44;
        pub const CPL: u8 = 0x45;

        pub const POP: u8 = 0x71;

        pub const RET: u8 = 0x81;
    }
    pub mod w2 {
        pub const LD: u8 = 0x10;
        pub const ST: u8 = 0x11;
        pub const LDA: u8 = 0x12;

        pub const ADDA: u8 = 0x20;
        pub const SUBA: u8 = 0x21;
        pub const ADDL: u8 = 0x22;
        pub const SUBL: u8 = 0x23;

        pub const AND: u8 = 0x30;
        pub const OR: u8 = 0x31;
        pub const XOR: u8 = 0x32;

        pub const CPA: u8 = 0x40;
        pub const CPL: u8 = 0x41;

        pub const SLA : u8 = 0x50;
        pub const SRA : u8 = 0x51;
        pub const SLL : u8 = 0x52;
        pub const SRL : u8 = 0x53;

        pub const JMI: u8 = 0x61;
        pub const JNZ: u8 = 0x62;
        pub const JZE: u8 = 0x63;
        pub const JUMP: u8 = 0x64;
        pub const JPL: u8 = 0x65;
        pub const JOV: u8 = 0x66;

        pub const PUSH: u8 = 0x70;

        pub const CALL: u8 = 0x80;

        pub const SVC: u8 = 0xF0;
    }
}

pub fn opecode_to_4char(opcode: u8) -> [char; 4] {
    let chars = match opcode {
        instruction::w1::NOP => ['N', 'O', 'P', ' '],
        instruction::w1::LD => ['L', 'D', ' ', ' '],
        instruction::w1::ADDA => ['A', 'D', 'D', 'A'],
        instruction::w1::SUBA => ['S', 'U', 'B', 'A'],
        instruction::w1::ADDL => ['A', 'D', 'D', 'L'],
        instruction::w1::SUBL => ['S', 'U', 'B', 'L'],
        instruction::w1::AND => ['A', 'N', 'D', ' '],
        instruction::w1::OR => ['O', 'R', ' ', ' '],
        instruction::w1::XOR => ['X', 'O', 'R', ' '],
        instruction::w1::CPA => ['C', 'P', 'A', ' '],
        instruction::w1::CPL => ['C', 'P', 'L', ' '],
        instruction::w1::POP => ['P', 'O', 'P', ' '],
        instruction::w1::RET => ['R', 'E', 'T', ' '],
        instruction::w2::LD => ['L', 'D', ' ', ' '],
        instruction::w2::ST => ['S', 'T', ' ', ' '],
        instruction::w2::LDA => ['L', 'D', 'A', ' '],
        instruction::w2::ADDA => ['A', 'D', 'D', 'A'],
        instruction::w2::SUBA => ['S', 'U', 'B', 'A'],
        instruction::w2::ADDL => ['A', 'D', 'D', 'L'],
        instruction::w2::SUBL => ['S', 'U', 'B', 'L'],
        instruction::w2::AND => ['A', 'N', 'D', ' '],
        instruction::w2::OR => ['O', 'R', ' ', ' '],
        instruction::w2::XOR => ['X', 'O', 'R', ' '],
        instruction::w2::CPA => ['C', 'P', 'A', ' '],
        instruction::w2::CPL => ['C', 'P', 'L', ' '],
        instruction::w2::SLA => ['S', 'L', 'A', ' '],
        instruction::w2::SRA => ['S', 'R', 'A', ' '],
        instruction::w2::SLL => ['S', 'L', 'L', ' '],
        instruction::w2::SRL => ['S', 'R', 'L', ' '],
        instruction::w2::JMI => ['J', 'M', 'I', ' '],
        instruction::w2::JNZ => ['J', 'N', 'Z', ' '],
        instruction::w2::JZE => ['J', 'Z', 'E', ' '],
        instruction::w2::JUMP => ['J', 'U', 'M', 'P'],
        instruction::w2::JPL => ['J', 'P', 'L', ' '],
        instruction::w2::JOV => ['J', 'O', 'V', ' '],
        instruction::w2::PUSH => ['P', 'U', 'S', 'H'],
        instruction::w2::CALL => ['C', 'A', 'L', 'L'],
        instruction::w2::SVC => ['S', 'V', 'C', ' '],
        _ => ['I', 'D', 'K', '?'], // Unknown opcode
    };
    chars
}