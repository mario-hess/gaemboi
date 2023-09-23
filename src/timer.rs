/**
 * @file    timer.rs
 * @brief   Handles the timer and divider registers.
 * @author  Mario Hess
 * @date    September 23, 2023
 */
use crate::interrupt::TIMER_MASK;

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

const TIMER_ENABLE_MASK: u8 = 0x04;
const TIMER_CONTROL_MASK: u8 = 0x03;

const CYCLES_DIV: u16 = 256;
const CYCLES_TAC_0: u16 = 1024;
const CYCLES_TAC_1: u16 = 16;
const CYCLES_TAC_2: u16 = 64;
const CYCLES_TAC_3: u16 = 256;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    div_counter: u16,
    tima_counter: i64,
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

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;
        self.div_counter += t_cycles;

        while self.div_counter >= CYCLES_DIV {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= CYCLES_DIV;
        }

        if !self.enabled {
            return;
        }

        if self.tima_overflowed {
            self.interrupt = TIMER_MASK;
            self.tima = self.tma;
            self.tima_overflowed = false;
        }

        self.tima_counter += t_cycles as i64;

        while self.tima_counter >= self.tac_cycles as i64 {
            if self.tima == 255 {
                self.tima_overflowed = true;
                self.tima = 0;
                return;
            }

            self.tima = self.tima.wrapping_add(1);
            self.tima_counter -= self.tac_cycles as i64;
        }
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt = 0;
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

                self.tac_cycles = match value & TIMER_CONTROL_MASK {
                    0x00 => CYCLES_TAC_0,
                    0x01 => CYCLES_TAC_1,
                    0x02 => CYCLES_TAC_2,
                    0x03 => CYCLES_TAC_3,
                    _ => panic!("Invalid TAC value: {:#X}", self.tac),
                };

                self.enabled = (self.tac & TIMER_ENABLE_MASK) == TIMER_ENABLE_MASK;
            }
            _ => unreachable!(),
        }
    }
}
