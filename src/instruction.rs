#[derive(Debug)]
pub enum Mnemonic {
    Nop,
    Rst(u16),
    JPnn,
    CPn,
    CALLnn,
    INCPair(Target),
    Add(Target),
    XORReg(Target),
    LDRegReg(Target, Target),
    LDRegN(Target),
    LDPairReg(Target, Target),
    LDRegPair(Target, Target),
    LDnnA,
    LDHnA,
    LDHAn,
    JRce(Flag),
    JRnce(Flag),
    JRe,
    DisableInterrupt,
    Prefix,
    ResBReg(u8, Target),
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
    C,
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
            0x20 => Instruction::new(Mnemonic::JRnce(Flag::Z), 2, 3),
            0x1A => Instruction::new(Mnemonic::LDRegPair(Target::A, Target::DE), 1, 2),
            0x1E => Instruction::new(Mnemonic::LDRegN(Target::E), 2, 2),
            0x23 => Instruction::new(Mnemonic::INCPair(Target::HL), 1, 2),
            0x28 => Instruction::new(Mnemonic::JRce(Flag::Z), 2, 2),
            0x2E => Instruction::new(Mnemonic::LDRegN(Target::L), 2, 2),
            0x3E => Instruction::new(Mnemonic::LDRegN(Target::A), 2, 2),
            0x40 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::B), 1, 1),
            0x41 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::C), 1, 1),
            0x42 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::D), 1, 1),
            0x43 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::E), 1, 1),
            0x44 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::H), 1, 1),
            0x45 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::L), 1, 1),
            0x46 => Instruction::new(Mnemonic::LDRegPair(Target::B, Target::HL), 1, 2),
            0x47 => Instruction::new(Mnemonic::LDRegReg(Target::B, Target::A), 1, 1),
            0x48 => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::B), 1, 1),
            0x49 => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::C), 1, 1),
            0x4A => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::D), 1, 1),
            0x4B => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::E), 1, 1),
            0x4C => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::H), 1, 1),
            0x4D => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::L), 1, 1),
            0x4E => Instruction::new(Mnemonic::LDRegPair(Target::C, Target::HL), 1, 2),
            0x4F => Instruction::new(Mnemonic::LDRegReg(Target::C, Target::A), 1, 1),
            0x50 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::B), 1, 1),
            0x51 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::C), 1, 1),
            0x52 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::D), 1, 1),
            0x53 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::E), 1, 1),
            0x54 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::H), 1, 1),
            0x55 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::L), 1, 1),
            0x56 => Instruction::new(Mnemonic::LDRegPair(Target::D, Target::HL), 1, 2),
            0x57 => Instruction::new(Mnemonic::LDRegReg(Target::D, Target::A), 1, 1),
            0x58 => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::B), 1, 1),
            0x59 => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::C), 1, 1),
            0x5A => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::D), 1, 1),
            0x5B => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::E), 1, 1),
            0x5C => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::H), 1, 1),
            0x5D => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::L), 1, 1),
            0x5E => Instruction::new(Mnemonic::LDRegPair(Target::E, Target::HL), 1, 2),
            0x5F => Instruction::new(Mnemonic::LDRegReg(Target::E, Target::A), 1, 1),
            0x60 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::B), 1, 1),
            0x61 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::C), 1, 1),
            0x62 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::D), 1, 1),
            0x63 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::E), 1, 1),
            0x64 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::H), 1, 1),
            0x65 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::L), 1, 1),
            0x66 => Instruction::new(Mnemonic::LDRegPair(Target::H, Target::HL), 1, 2),
            0x67 => Instruction::new(Mnemonic::LDRegReg(Target::H, Target::A), 1, 1),
            0x68 => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::B), 1, 1),
            0x69 => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::C), 1, 1),
            0x6A => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::D), 1, 1),
            0x6B => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::E), 1, 1),
            0x6C => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::H), 1, 1),
            0x6D => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::L), 1, 1),
            0x6E => Instruction::new(Mnemonic::LDRegPair(Target::L, Target::HL), 1, 2),
            0x6F => Instruction::new(Mnemonic::LDRegReg(Target::L, Target::A), 1, 1),
            0x70 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::B), 1, 2),
            0x71 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::C), 1, 2),
            0x72 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::D), 1, 2),
            0x73 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::E), 1, 2),
            0x74 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::H), 1, 2),
            0x75 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::L), 1, 2),
            0x77 => Instruction::new(Mnemonic::LDPairReg(Target::HL, Target::A), 1, 2),
            0x78 => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::B), 1, 1),
            0x79 => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::C), 1, 1),
            0x7A => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::D), 1, 1),
            0x7B => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::E), 1, 1),
            0x7C => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::H), 1, 1),
            0x7D => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::L), 1, 1),
            0x7E => Instruction::new(Mnemonic::LDRegPair(Target::A, Target::HL), 1, 2),
            0x7F => Instruction::new(Mnemonic::LDRegReg(Target::A, Target::A), 1, 1),
            0xA8 => Instruction::new(Mnemonic::XORReg(Target::B), 1, 1),
            0xA9 => Instruction::new(Mnemonic::XORReg(Target::C), 1, 1),
            0xAA => Instruction::new(Mnemonic::XORReg(Target::D), 1, 1),
            0xAB => Instruction::new(Mnemonic::XORReg(Target::E), 1, 1),
            0xAC => Instruction::new(Mnemonic::XORReg(Target::H), 1, 1),
            0xAD => Instruction::new(Mnemonic::XORReg(Target::L), 1, 1),
            0xAF => Instruction::new(Mnemonic::XORReg(Target::A), 1, 1),
            0xC3 => Instruction::new(Mnemonic::JPnn, 3, 4),
            0xCB => Instruction::new(Mnemonic::Prefix, 1, 1),
            0xCD => Instruction::new(Mnemonic::CALLnn, 3, 6),
            0xCF => Instruction::new(Mnemonic::Rst(0x0008), 1, 4),
            0xDF => Instruction::new(Mnemonic::Rst(0x0018), 1, 4),
            0xE0 => Instruction::new(Mnemonic::LDHnA, 2, 3),
            0xEA => Instruction::new(Mnemonic::LDnnA, 3, 4),
            0xEF => Instruction::new(Mnemonic::Rst(0x0028), 1, 4),
            0xF0 => Instruction::new(Mnemonic::LDHAn, 2, 3),
            0xF3 => Instruction::new(Mnemonic::DisableInterrupt, 1, 1),
            0xFE => Instruction::new(Mnemonic::CPn, 2, 2),
            0xFF => Instruction::new(Mnemonic::Rst(0x0038), 1, 4),
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
    }

    pub fn from_prefix(value: u8) -> Self {
        match value {
            0x87 => Instruction::new(Mnemonic::ResBReg(0, Target::A), 2, 2),
            _ => panic!("PREFIX Instruction for byte {:#X} not implemented.", value),
        }
    }
}
