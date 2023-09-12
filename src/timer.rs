pub const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

pub struct Timer {
    div: u8,
    div_counter: u32,
    tima: u8,
    tima_counter: i64,
    tma: u8,
    tac: u8,
    overflowed: bool,
    pub interrupt_request: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            div_counter: 0,
            tima: 0,
            tima_counter: 0,
            tma: 0,
            tac: 0,
            overflowed: false,
            interrupt_request: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let cycles = (m_cycles * 4) as u32;
        self.div_counter += cycles;

        while self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }

        let speed = self.tac << 6;
        let running = (1 << 2) & self.tac > 0;

        if !running {
            return;
        }

        self.tima_counter += cycles as i64;

        let timer_cycles = match speed {
            0 => 1024,
            0x40 => 16,
            0x80 => 64,
            0xC0 => 256,
            _ => panic!("Unknown timer speed: 0x{:X}", speed),
        };

        if self.overflowed {
            self.interrupt_request |= 0x04;
            self.tima = self.tma;
            self.overflowed = false;
        }

        while self.tima_counter >= timer_cycles {
            if self.tima == 255 {
                self.overflowed = true;
                self.tima = 0;
                return;
            }

            self.tima += 1;
            self.tima_counter -= timer_cycles;
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
            DIV => self.div = 0,
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => self.tac = value,
            _ => unreachable!(),
        }
    }
}
