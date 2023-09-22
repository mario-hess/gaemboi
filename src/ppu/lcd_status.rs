/**
 * @file    ppu/lcd_status.rs
 * @brief   Handles the PPU's LCD Status register.
 * @author  Mario Hess
 * @date    September 22, 2023
 */
use crate::ppu::Mode;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct LCD_status {
    pub mode: Mode,
    compare_flag: bool,
    interrupt_hblank: bool,
    interrupt_vblank: bool,
    interrupt_oam: bool,
    interrupt_stat: bool,
}

impl LCD_status {
    pub fn new() -> Self {
        Self {
            mode: Mode::VBlank,
            compare_flag: true,
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
            0x03 => Mode::VRam,
            _ => unreachable!(),
        };

        self.compare_flag = (value & 0x04) != 0;
        self.interrupt_hblank = (value & 0x08) != 0;
        self.interrupt_vblank = (value & 0x10) != 0;
        self.interrupt_oam = (value & 0x20) != 0;
        self.interrupt_stat = (value & 0x40) != 0;
    }

    pub fn get(self) -> u8 {
        let compare_flag = if self.compare_flag { 0x04 } else { 0 };
        let interrupt_hblank = if self.interrupt_hblank { 0x08 } else { 0 };
        let interrupt_vblank = if self.interrupt_vblank { 0x10 } else { 0 };
        let interrupt_oam = if self.interrupt_oam { 0x20 } else { 0 };
        let interrupt_stat = if self.interrupt_stat { 0x40 } else { 0 };

        self.mode as u8 | compare_flag | interrupt_hblank | interrupt_vblank | interrupt_oam | interrupt_stat
    }
}
