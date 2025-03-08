use crate::gb_classic::cpu::{clock::CPU_CLOCK_SPEED, interrupt::TIMER_MASK};

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

const TAC_ENABLE_MASK: u8 = 0x04;
const TAC_CLOCK_SELECT_MASK: u8 = 0x03;

const TIMER_CLOCK_SPEED: u16 = 16384;
const CYCLES_DIV: u16 = (CPU_CLOCK_SPEED / TIMER_CLOCK_SPEED as u32) as u16;

const CYCLES_TAC_0: u16 = 1024;
const CYCLES_TAC_1: u16 = 16;
const CYCLES_TAC_2: u16 = 64;
const CYCLES_TAC_3: u16 = 256;

// https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    div_counter: u16,
    tima_counter: u16,
    tima_overflowed: bool,
    tac_cycles: u16,
    enabled: bool,
    pub interrupt: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAB,
            tima: 0,
            tma: 0,
            tac: 0xF8,
            div_counter: 0,
            tima_counter: 0,
            tima_overflowed: false,
            tac_cycles: CYCLES_TAC_0,
            enabled: false,
            interrupt: 0,
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
            // Writing any value to this register resets it to 0
            DIV => self.div = 0,
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => self.set_tac(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;
        self.div_counter += t_cycles;

        // DIV (0xFF04) is incremented at a rate of 16384Hz
        while self.div_counter >= CYCLES_DIV {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= CYCLES_DIV;
        }

        // Controls whether TIMA (0xFF05) is incremented. DIV (0xFF04)
        // is always counting, regardless of this bit (0xFF07 & 0x04)
        if !self.enabled {
            return;
        }

        // TIMA (0xFF05) overflow is handled on the next cycle iteration
        if self.tima_overflowed {
            self.interrupt = TIMER_MASK;
            self.tima = self.tma;
            self.tima_overflowed = false;
        }

        // TIMA (0xFF05) is incremented at the clock frequency specified by the
        // TAC (0xFF07) register. When the value overflows, it is reset to
        // the value specified in TMA (0xFF06) and an interrupt is requested
        self.tima_counter += t_cycles;

        while self.tima_counter >= self.tac_cycles {
            let (value, overflowed) = self.tima.overflowing_add(1);
            self.tima = value;
            self.tima_overflowed = overflowed;

            if self.tima_overflowed {
                return;
            }

            self.tima_counter -= self.tac_cycles;
        }
    }

    /**
     * Controls the frequency at which TIMA is incremented, as follows:
     * Clock select     Increment every
     * 00               256 M-cycles
     * 01               4 M-cycles
     * 10               16 M-cycles
     * 11               64 M-cycles
     */
    fn set_tac(&mut self, value: u8) {
        self.tac = value;

        self.tac_cycles = match value & TAC_CLOCK_SELECT_MASK {
            0b00 => CYCLES_TAC_0,
            0b01 => CYCLES_TAC_1,
            0b10 => CYCLES_TAC_2,
            0b11 => CYCLES_TAC_3,
            _ => unreachable!(),
        };

        self.enabled = (self.tac & TAC_ENABLE_MASK) != 0;
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt = 0;
    }
}
