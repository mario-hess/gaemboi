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

    pub fn get_interrupts(&self) -> [(u8, u16); 5] {
        self.interrupts
    }

    pub fn interrupt_enabled(&self, interrupt_enabled: u8, interrupt_flag: u8) -> bool {
        for (interrupt, _) in &self.interrupts {
            if self.is_enabled(interrupt_enabled, interrupt_flag, *interrupt) {
                return true;
            }
        }

        false
    }

    pub fn is_enabled(&self, interrupt_enabled: u8, interrupt_flag: u8, value: u8) -> bool {
        let is_enabled = interrupt_enabled & value;
        let is_requested = interrupt_flag & value;

        is_requested == value && is_enabled == value
    }

    pub fn handle_interrupt(&self, interrupt_enabled: u8, interrupt_flag: u8, value: u8) -> bool {
        if !self.is_enabled(interrupt_enabled, interrupt_flag, value) {
            return false;
        }

        true
    }
}
