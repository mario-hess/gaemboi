/*
 * @file    interrupt.rs
 * @brief   Handles CPU interrupts.
 * @author  Mario Hess
 * @date    May 20, 2024
 */

use crate::cpu::Cpu;

pub const VBLANK_MASK: u8 = 0x01;
const VBLANK_ISR: u16 = 0x0040;

pub const LCD_STAT_MASK: u8 = 0x02;
const LCD_STAT_ISR: u16 = 0x0048;

pub const TIMER_MASK: u8 = 0x04;
const TIMER_ISR: u16 = 0x0050;

const SERIAL_MASK: u8 = 0x08;
const SERIAL_ISR: u16 = 0x0058;

const JOYPAD_MASK: u8 = 0x10;
const JOYPAD_ISR: u16 = 0x0060;

// https://gbdev.io/pandocs/Interrupts.html#interrupt-handling
#[derive(Copy, Clone)]
pub struct Interrupt {
    interrupts: [(u8, u16); 5], // (bit_position, isr_address)
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            interrupts: [
                (VBLANK_MASK, VBLANK_ISR),
                (LCD_STAT_MASK, LCD_STAT_ISR),
                (TIMER_MASK, TIMER_ISR),
                (SERIAL_MASK, SERIAL_ISR),
                (JOYPAD_MASK, JOYPAD_ISR),
            ],
        }
    }

    pub fn interrupt_enabled(self, interrupt_enabled: u8, interrupt_flag: u8) -> bool {
        for (interrupt, _) in self.interrupts {
            if self.is_enabled(interrupt_enabled, interrupt_flag, interrupt) {
                return true;
            }
        }

        false
    }

    pub fn is_enabled(self, interrupt_enabled: u8, interrupt_flag: u8, value: u8) -> bool {
        let is_enabled = interrupt_enabled & value;
        let is_requested = interrupt_flag & value;

        is_requested == value && is_enabled == value
    }

    pub fn handle_interrupts(self, cpu: &mut Cpu) -> Option<u8> {
        for (interrupt, isr_address) in self.interrupts {
            if self.handle_interrupt(cpu, interrupt, isr_address) {
                return Some(5);
            }
        }

        None
    }

    fn handle_interrupt(self, cpu: &mut Cpu, value: u8, isr_address: u16) -> bool {
        let interrupt_enabled = cpu.memory_bus.interrupt_enabled;
        let interrupt_flag = cpu.memory_bus.interrupt_flag;

        if !self.is_enabled(interrupt_enabled, interrupt_flag, value) {
            return false;
        }

        cpu.interrupt_service_routine(isr_address, value);

        true
    }
}
