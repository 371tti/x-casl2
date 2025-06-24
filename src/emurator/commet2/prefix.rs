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