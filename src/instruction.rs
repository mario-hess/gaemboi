#[derive(Debug)]
pub enum Mnemonic {
    Nop,
    Rst(u16),
    JPnn,
    CPn,
    INCPair(Target),
    Add(Target),
    XORReg(Target),
    LoadNextToReg(Target),
    LDRegPair(Target, Target),
    LDPairReg(Target, Target),
    JRcce(Flag),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C
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
            0x02 => Instruction::new(Mnemonic::LDRegPair(Target::BC, Target::A), 1, 2),
            0x03 => Instruction::new(Mnemonic::INCPair(Target::BC), 1, 2),
            0x1A => Instruction::new(Mnemonic::LDPairReg(Target::A, Target::DE), 1, 2),
            0x28 => Instruction::new(Mnemonic::JRcce(Flag::Z), 2, 2),
            0x3E => Instruction::new(Mnemonic::LoadNextToReg(Target::A), 2, 2),
            0xAF => Instruction::new(Mnemonic::XORReg(Target::A), 1, 1),
            0xC3 => Instruction::new(Mnemonic::JPnn, 3, 4),
            0xCF => Instruction::new(Mnemonic::Rst(0x0008), 1, 4),
            0xFE => Instruction::new(Mnemonic::CPn, 2, 2),
            0xFF => Instruction::new(Mnemonic::Rst(0x0038), 1, 4),
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
   } 
}
