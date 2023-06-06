#[derive(Debug)]
pub enum Mnemonic {
    Nop,
    Rst(u16),
    JPnn,
    CPn,
    INCPair(Target),
    Add(Target),
    XORReg(Target),
    LDRegN(Target),
    LDPairReg(Target, Target),
    LDRegPair(Target, Target),
    LDnnA,
    JRcce(Flag),
    JRe,
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
            0x02 => Instruction::new(Mnemonic::LDPairReg(Target::BC, Target::A), 1, 2),
            0x03 => Instruction::new(Mnemonic::INCPair(Target::BC), 1, 2),
            0x0E => Instruction::new(Mnemonic::LDRegN(Target::C), 2, 2),
            0x12 => Instruction::new(Mnemonic::LDPairReg(Target::DE, Target::A), 1, 2),
            0x13 => Instruction::new(Mnemonic::INCPair(Target::DE), 1, 2),
            0x18 => Instruction::new(Mnemonic::JRe, 2, 3),
            0x1A => Instruction::new(Mnemonic::LDRegPair(Target::A, Target::DE), 1, 2),
            0x1E => Instruction::new(Mnemonic::LDRegN(Target::E), 2, 2),
            0x23 => Instruction::new(Mnemonic::INCPair(Target::HL), 1, 2),
            0x28 => Instruction::new(Mnemonic::JRcce(Flag::Z), 2, 2),
            0x2E => Instruction::new(Mnemonic::LDRegN(Target::L), 2, 2),
            0x3E => Instruction::new(Mnemonic::LDRegN(Target::A), 2, 2),
            0x46 => Instruction::new(Mnemonic::LDRegPair(Target::B, Target::HL), 1, 2),
            0x4E => Instruction::new(Mnemonic::LDRegPair(Target::C, Target::HL), 1 ,2),
            0x56 => Instruction::new(Mnemonic::LDRegPair(Target::D, Target::HL), 1, 2),
            0x5E => Instruction::new(Mnemonic::LDRegPair(Target::E, Target::HL), 1, 2),
            0x66 => Instruction::new(Mnemonic::LDRegPair(Target::H, Target::HL), 1, 2),
            0x6E => Instruction::new(Mnemonic::LDRegPair(Target::L, Target::HL), 1, 2),
            0x70 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::B), 1, 2),
            0x71 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::C), 1, 2),
            0x72 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::D), 1, 2),
            0x73 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::E), 1, 2),
            0x74 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::H), 1, 2),
            0x75 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::L), 1, 2),
            0x77 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::A), 1, 2),
            0x7E => Instruction::new(Mnemonic::LDRegPair(Target::A, Target::HL), 1, 2),
            0xA8 => Instruction::new(Mnemonic::XORReg(Target::B), 1, 1),
            0xA9 => Instruction::new(Mnemonic::XORReg(Target::C), 1, 1),
            0xAA => Instruction::new(Mnemonic::XORReg(Target::D), 1 ,1),
            0xAB => Instruction::new(Mnemonic::XORReg(Target::E), 1, 1),
            0xAC => Instruction::new(Mnemonic::XORReg(Target::H), 1, 1),
            0xAD => Instruction::new(Mnemonic::XORReg(Target::L), 1, 1),
            0xAF => Instruction::new(Mnemonic::XORReg(Target::A), 1, 1),
            0xC3 => Instruction::new(Mnemonic::JPnn, 3, 4),
            0xCF => Instruction::new(Mnemonic::Rst(0x0008), 1, 4),
            0xDF => Instruction::new(Mnemonic::Rst(0x0018), 1, 4),
            0xEA => Instruction::new(Mnemonic::LDnnA, 3, 4),
            0xEF => Instruction::new(Mnemonic::Rst(0x0028), 1, 4),
            0xFE => Instruction::new(Mnemonic::CPn, 2, 2),
            0xFF => Instruction::new(Mnemonic::Rst(0x0038), 1, 4),
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
   } 
}
