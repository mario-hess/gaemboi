/*
 * @file    ppu/lcd_status.rs
 * @brief   Handles the PPU's LCD Status register.
 * @author  Mario Hess
 * @date    May 30, 2024
 */

use crate::ppu::LCD_STAT_MASK;

pub const MODE_HBLANK: u8 = 0x00;
pub const MODE_VBLANK: u8 = 0x01;
pub const MODE_OAM: u8 = 0x02;
pub const MODE_TRANSFER: u8 = 0x03;

const MODE_MASK: u8 = 0x03;
const COMPARE_MASK: u8 = 0x04;
const HBLANK_MASK: u8 = 0x08;
const VBLANK_MASK: u8 = 0x10;
const OAM_MASK: u8 = 0x20;
const STAT_MASK: u8 = 0x40;

#[allow(non_camel_case_types)]
pub struct LCD_status {
    pub mode: u8,
    compare_flag: bool,
    interrupt_hblank: bool,
    interrupt_vblank: bool,
    interrupt_oam: bool,
    interrupt_stat: bool,
}

impl LCD_status {
    pub fn new() -> Self {
        Self {
            mode: 0,
            compare_flag: false,
            interrupt_hblank: false,
            interrupt_vblank: false,
            interrupt_oam: false,
            interrupt_stat: false,
        }
    }

    pub fn get_mode(&self) -> u8 { self.mode & MODE_MASK }
    pub fn set_mode(&mut self, value: u8, interrupts: &mut u8) {
        self.mode = value & MODE_MASK;
        self.check_interrupts(interrupts);
    }

    pub fn get_compare_flag(&self) -> bool { self.compare_flag }
    pub fn set_compare_flag(&mut self, value: bool) { self.compare_flag = value; }

    pub fn get_interrupt_hblank(&self) -> bool { self.interrupt_hblank }
    pub fn set_interrupt_hblank(&mut self, value: bool) { self.interrupt_hblank = value; }

    pub fn get_interrupt_vblank(&self) -> bool { self.interrupt_vblank }
    pub fn set_interrupt_vblank(&mut self, value: bool) { self.interrupt_vblank = value; }

    pub fn get_interrupt_oam(&self) -> bool { self.interrupt_oam }
    pub fn set_interrupt_oam(&mut self, value: bool) { self.interrupt_oam = value; }

    pub fn get_interrupt_stat(&self) -> bool { self.interrupt_stat }
    pub fn set_interrupt_stat(&mut self, value: bool) { self.interrupt_stat = value; }

    #[rustfmt::skip]
    fn check_interrupts(&mut self, interrupts: &mut u8) {
        match self.mode {
            MODE_HBLANK => {
                if self.interrupt_hblank { *interrupts |= LCD_STAT_MASK; }
            }
            MODE_VBLANK => {
                if self.interrupt_vblank { *interrupts |= LCD_STAT_MASK; }
            }
            MODE_OAM => {
                if self.interrupt_oam { *interrupts |= LCD_STAT_MASK; }
            }
            _ => {}
        }
    }
}

#[rustfmt::skip]
impl std::convert::From<&LCD_status> for u8 {
    fn from(lcd_status: &LCD_status) -> u8 {
        lcd_status.mode & MODE_MASK
            | (if lcd_status.compare_flag { COMPARE_MASK } else { 0 })
            | (if lcd_status.interrupt_hblank { HBLANK_MASK } else { 0 })
            | (if lcd_status.interrupt_vblank { VBLANK_MASK } else { 0 })
            | (if lcd_status.interrupt_oam { OAM_MASK } else { 0 })
            | (if lcd_status.interrupt_stat { STAT_MASK } else { 0 })
    }
}

impl std::convert::From<u8> for LCD_status {
    fn from(byte: u8) -> Self {
        Self {
            mode: byte & MODE_MASK,
            compare_flag: (byte & COMPARE_MASK) != 0,
            interrupt_hblank: (byte & HBLANK_MASK) != 0,
            interrupt_vblank: (byte & VBLANK_MASK) != 0,
            interrupt_oam: (byte & OAM_MASK) != 0,
            interrupt_stat: (byte & STAT_MASK) != 0,
        }
    }
}

impl Default for LCD_status {
    fn default() -> Self {
        Self::new()
    }
}
