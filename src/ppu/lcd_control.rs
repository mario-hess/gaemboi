#![cfg_attr(rustfmt, rustfmt::skip)]
/*
 * @file    ppu/lcd_control.rs
 * @brief   Handles the PPU's LCD Control register.
 * @author  Mario Hess
 * @date    May 30, 2024
 */

use crate::ppu::{TILEMAP_START_0, TILEMAP_START_1};

const BG_ENABLED_MASK: u8 = 0x01;
const OBJECT_ENABLED_MASK: u8 = 0x02;
const OBJECT_SIZE_MASK: u8 = 0x04;
const BG_TILEMAP_MASK: u8 = 0x08;
const ADDRESSING_MODE_MASK: u8 = 0x10;
const WINDOW_ENABLED_MASK: u8 = 0x20;
const WINDOW_TILEMAP_MASK: u8 = 0x40;
const LCD_ENABLED_MASK: u8 = 0x80;

pub const TILE_BLOCK_0: u16 = 0x8000;
const TILE_BLOCK_1: u16 = 0x8800;
pub const TILE_BLOCK_2: u16 = 0x9000;

const TILE_OFFSET: u16 = 16;

// https://gbdev.io/pandocs/LCDC.html
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct LCD_control {
    bg_enabled: bool,
    object_enabled: bool,
    object_size: bool,
    bg_tilemap: bool,
    addressing_mode: bool,
    window_enabled: bool,
    window_tilemap: bool,
    lcd_enabled: bool,
}

impl LCD_control {
    pub fn new() -> Self {
        Self {
            bg_enabled: true,
            object_enabled: false,
            object_size: false,
            bg_tilemap: false,
            addressing_mode: true,
            window_enabled: false,
            window_tilemap: false,
            lcd_enabled: true,
        }
    }

    pub fn bg_enabled(&self) -> bool { self.bg_enabled }
    pub fn object_enabled(&self) -> bool { self.object_enabled }
    pub fn object_size(&self) -> bool { self.object_size }
    pub fn bg_tilemap(&self) -> bool { self.bg_tilemap }
    pub fn addressing_mode(&self) -> bool { self.addressing_mode }
    pub fn window_enabled(&self) -> bool { self.window_enabled }
    pub fn window_tilemap(&self) -> bool { self.window_tilemap }
    pub fn lcd_enabled(&self) -> bool { self.lcd_enabled }

    pub fn get_bg_address(self) -> u16 {
        if !self.bg_tilemap() { TILEMAP_START_0 } else { TILEMAP_START_1 }
    }

    pub fn get_window_address(self) -> u16 {
        if !self.window_tilemap() { TILEMAP_START_0 } else { TILEMAP_START_1 }
    }

    // https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data
    pub fn get_address(self, tile_index: u8) -> u16 {
        if self.addressing_mode() {
            TILE_BLOCK_0 + (tile_index as u16 * TILE_OFFSET)
        } else if tile_index < 128 {
            TILE_BLOCK_2 + (tile_index as u16 * TILE_OFFSET)
        } else {
            TILE_BLOCK_1 + ((tile_index - 128) as u16 * TILE_OFFSET)
        }
    }
}

impl std::convert::From<&LCD_control> for u8 {
    fn from(lcd_control: &LCD_control) -> u8 {
        (if lcd_control.bg_enabled { BG_ENABLED_MASK } else { 0 })
            | (if lcd_control.object_enabled { OBJECT_ENABLED_MASK } else { 0 })
            | (if lcd_control.object_size { OBJECT_SIZE_MASK } else { 0 })
            | (if lcd_control.bg_tilemap { BG_TILEMAP_MASK } else { 0 })
            | (if lcd_control.addressing_mode { ADDRESSING_MODE_MASK } else { 0 })
            | (if lcd_control.window_enabled { WINDOW_ENABLED_MASK } else { 0 })
            | (if lcd_control.window_tilemap { WINDOW_TILEMAP_MASK } else { 0 })
            | (if lcd_control.lcd_enabled { LCD_ENABLED_MASK } else { 0 })
    }
}

impl std::convert::From<u8> for LCD_control {
    fn from(byte: u8) -> Self {
        Self {
            bg_enabled: (byte & BG_ENABLED_MASK) != 0,
            object_enabled: (byte & OBJECT_ENABLED_MASK) != 0,
            object_size: (byte & OBJECT_SIZE_MASK) != 0,
            bg_tilemap: (byte & BG_TILEMAP_MASK) != 0,
            addressing_mode: (byte & ADDRESSING_MODE_MASK) != 0,
            window_enabled: (byte & WINDOW_ENABLED_MASK) != 0,
            window_tilemap: (byte & WINDOW_TILEMAP_MASK) != 0,
            lcd_enabled: (byte & LCD_ENABLED_MASK) != 0,
        }
    }
}

impl Default for LCD_control {
    fn default() -> Self {
        Self::new()
    }
}
