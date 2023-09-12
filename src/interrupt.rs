use crate::cpu::Cpu;

const VBLANK_MASK: u8 = 0x01;
const VBLANK_ISR: u16 = 0x0040;

const LCD_STAT_MASK: u8 = 0x02;
const LCD_STAT_ISR: u16 = 0x0048;

pub const TIMER_MASK: u8 = 0x04;
const TIMER_ISR: u16 = 0x0050;

const SERIAL_MASK: u8 = 0x08;
const SERIAL_ISR: u16 = 0x0058;

const JOYPAD_MASK: u8 = 0x10;
const JOYPAD_ISR: u16 = 0x0060;

#[derive(Copy, Clone)]
pub struct Interrupt {
    interrupts: [(u8, u16); 5], // (bit_position, isr_address)
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            interrupts: [
                (VBLANK_MASK, VBLANK_ISR), // VBlank
                (LCD_STAT_MASK, LCD_STAT_ISR), // LCDStat
                (TIMER_MASK, TIMER_ISR), // Timer
                (SERIAL_MASK, SERIAL_ISR), // Serial
                (JOYPAD_MASK, JOYPAD_ISR), // Joypad
            ],
        }
    }

    pub fn interrupt_enabled(self, i_enable: u8, i_flag: u8) -> bool {
        for (interrupt, _) in self.interrupts {
            if self.is_enabled(i_enable, i_flag, interrupt) {
                return true;
            }
        }

        false
    }

    pub fn is_enabled(self, i_enable: u8, i_flag: u8, value: u8) -> bool {
        let is_requested = i_flag & value;
        let is_enabled = i_enable & value;

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
        let i_enable = cpu.memory_bus.interrupt_enable;
        let i_flag = cpu.memory_bus.io.interrupt_flag;

        if !self.is_enabled(i_enable, i_flag, value) {
            return false;
        }

        cpu.interrupt_service_routine(isr_address, value);

        true
    }
}
