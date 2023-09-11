pub const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    tima_overflow: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_overflow: false,
        }
    }

    pub fn tick(&mut self, m_cycles: u8, interrupt_flag: &mut u8) {}

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            DIV => (self.div >> 8) as u8,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            DIV => self.div = 0,
            TIMA => {
                self.tima = value;
                self.tima_overflow = false;
            }
            TMA => self.tma = value,
            TAC => self.tac = value & 0b111,
            _ => unreachable!(),
        }
    }
}
