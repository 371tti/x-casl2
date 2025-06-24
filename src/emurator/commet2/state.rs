pub struct CPU {
    pub cicle: u8,
    pub GR: GeneralResister,
    pub PR: u16,
    pub MAR: u16,
    pub MRR: u16,
    pub memory: Memory,
    pub IR: [u16; 2],
    pub FR: [bool; 3],
}

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

pub struct Memory(pub [u16; 65536]);