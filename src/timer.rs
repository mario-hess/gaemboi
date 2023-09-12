pub const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

pub struct Timer {
    div: u8,
    div_counter: u32,
    tima: u8,
    tima_counter: i64,
    tima_ratio: u32,
    tima_enabled: bool,
    tma: u8,
    tac: u8,
    pub interrupt_request: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            div_counter: 0,
            tima: 0,
            tima_counter: 0,
            tima_ratio: 1024,
            tima_enabled: false,
            tma: 0,
            tac: 0,
            interrupt_request: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let cycles = m_cycles as u32;
        self.div_counter += cycles;

        while self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }

        if self.tima_enabled {
            self.tima_counter += cycles as i64;
            while self.tima_counter >= self.tima_ratio as i64 {
                if self.tima == 255 {
                    self.interrupt_request |= 0x04;
                    self.tima = self.tma;
                } else {
                    self.tima = self.tima.wrapping_add(1);
                }

                self.tima_counter -= self.tima_ratio as i64;
            }
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            DIV => self.div,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            DIV => {
                self.div = 0;
                self.div_counter = 0;
                self.tima_counter = 0;
            }
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => {
                self.tac = value;
                match value & 0x03 {
                    0x00 => self.tima_ratio = 1024,
                    0x01 => self.tima_ratio = 16,
                    0x02 => self.tima_ratio = 64,
                    0x03 => self.tima_ratio = 256,
                    value => panic!("Invalid TAC value 0x{:02x}", value),
                }
                self.tima_enabled = (value & 0x04) == 0x04;
            }
            _ => unreachable!(),
        }
    }
}
