pub enum Mnemonic {
    Nop,
    JumpNN,
    Add(ArithmeticTarget),
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
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
            0x00 => {
                Instruction::new(Mnemonic::Nop, 1, 1)
            }
            0xC3 => {
                Instruction::new(Mnemonic::JumpNN, 3, 4)
            }
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
   } 
}
