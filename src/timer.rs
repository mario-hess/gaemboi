use crate::interrupt::TIMER_MASK;

pub const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

const DIV_CYCLES: u16 = 256;

pub struct Timer {
    div: u8,
    div_counter: u32,
    tima: u8,
    tima_counter: i64,
    tima_rate: u32,
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
            tima_rate: 1024,
            tima_enabled: false,
            tma: 0,
            tac: 0,
            interrupt_request: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u32;
        self.div_counter += t_cycles;

        while self.div_counter >= DIV_CYCLES as u32 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= DIV_CYCLES as u32;
        }

        if self.tima_enabled {
            self.tima_counter += t_cycles as i64;
            while self.tima_counter >= self.tima_rate as i64 {
                if self.tima == 255 {
                    self.interrupt_request = TIMER_MASK;
                    self.tima = self.tma;
                } else {
                    self.tima = self.tima.wrapping_add(1);
                }

                self.tima_counter -= self.tima_rate as i64;
            }
        }
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt_request = 0;
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
            DIV => self.div = 0,
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => {
                self.tac = value;
                match value & 0x03 {
                    0x00 => self.tima_rate = 1024,
                    0x01 => self.tima_rate = 16,
                    0x02 => self.tima_rate = 64,
                    0x03 => self.tima_rate = 256,
                    _ => panic!("Invalid TAC value: {:#X}", value),
                }
                self.tima_enabled = (value & TIMER_MASK) == TIMER_MASK;
            }
            _ => unreachable!(),
        }
    }
}

