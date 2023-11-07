/**
                   return;
* @file    ppu/lcd_status.rs
* @brief   Handles the PPU's LCD Status register.
* @author  Mario Hess
* @date    November 07, 2023
*/
use crate::ppu::{Mode, LCD_STAT_MASK};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct LCD_status {
    pub mode: Mode,
    pub compare_flag: bool,
    pub interrupt_hblank: bool,
    pub interrupt_vblank: bool,
    pub interrupt_oam: bool,
    pub interrupt_stat: bool,
}

impl LCD_status {
    pub fn new() -> Self {
        Self {
            mode: Mode::HBlank,
            compare_flag: false,
            interrupt_hblank: false,
            interrupt_vblank: false,
            interrupt_oam: false,
            interrupt_stat: false,
        }
    }

    pub fn set(&mut self, value: u8) {
        self.mode = match value & 0x03 {
            0x00 => Mode::HBlank,
            0x01 => Mode::VBlank,
            0x02 => Mode::OAM,
            0x03 => Mode::Transfer,
            _ => unreachable!(),
        };

        self.interrupt_hblank = (value & 0x08) == 0x08;
        self.interrupt_vblank = (value & 0x10) == 0x10;
        self.interrupt_oam = (value & 0x20) == 0x20;
        self.interrupt_stat = (value & 0x40) == 0x40;
    }

    pub fn get(self) -> u8 {
        let compare_flag = if self.compare_flag { 0x04 } else { 0 };
        let interrupt_hblank = if self.interrupt_hblank { 0x08 } else { 0 };
        let interrupt_vblank = if self.interrupt_vblank { 0x10 } else { 0 };
        let interrupt_oam = if self.interrupt_oam { 0x20 } else { 0 };
        let interrupt_stat = if self.interrupt_stat { 0x40 } else { 0 };

        self.mode as u8
            | compare_flag
            | interrupt_hblank
            | interrupt_vblank
            | interrupt_oam
            | interrupt_stat
    }

    pub fn set_mode(&mut self, mode: Mode, interrupts: &mut u8) {
        self.mode = mode;
        self.check_interrupts(interrupts);
    }

    fn check_interrupts(&mut self, interrupts: &mut u8) {
        match self.mode {
            Mode::HBlank => {
                if self.interrupt_hblank {
                    *interrupts |= LCD_STAT_MASK;
                }
            }
            Mode::VBlank => {
                if self.interrupt_vblank {
                    *interrupts |= LCD_STAT_MASK;
                }
            }
            Mode::OAM => {
                if self.interrupt_oam {
                    *interrupts |= LCD_STAT_MASK;
                }
            }
            _ => {}
        }
    }
}
