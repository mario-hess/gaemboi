pub enum Mnemonic {
    Nop,
    JumpNN,
    CpN,
    IncPair(Target),
    Add(Target),
    XorReg(Target),
    LoadNextToReg(Target),
    LoadRegToPairAddr(Target, Target),
}

pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
}


pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub length: u16,
    m_cycles: u8,
}

impl Instruction {
    fn new(mnemonic: Mnemonic, length: u16, m_cycles: u8) -> Self {
        Self {
            mnemonic,
            length,
            m_cycles,
        }
    }
   pub fn from_byte(value: u8) -> Self {
        match value {
            0x00 => Instruction::new(Mnemonic::Nop, 1, 1),
            0x02 => Instruction::new(Mnemonic::LoadRegToPairAddr(Target::BC, Target::A), 1, 2),
            0x03 => Instruction::new(Mnemonic::IncPair(Target::BC), 1, 2),
            0x3E => Instruction::new(Mnemonic::LoadNextToReg(Target::A), 2, 2),
            0xAF => Instruction::new(Mnemonic::XorReg(Target::A), 1, 1),
            0xC3 => Instruction::new(Mnemonic::JumpNN, 3, 4),
            0xFE => Instruction::new(Mnemonic::CpN, 2, 2),
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
   } 
}
