/**
 * @file    ppu/lcd_control.rs
 * @brief   Handles the PPU's LCD Control register.
 * @author  Mario Hess
 * @date    October 16, 2023
 */
use crate::ppu::{TILEMAP_START_0, TILEMAP_START_1};

pub const TILE_BLOCK_0: u16 = 0x8000;
const TILE_BLOCK_1: u16 = 0x8800;
pub const TILE_BLOCK_2: u16 = 0x9000;

const TILE_OFFSET: u16 = 16;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct LCD_control {
    pub bg_enabled: bool,
    pub object_enabled: bool,
    pub object_size: bool,
    pub bg_tilemap: bool,
    pub addressing_mode: bool,
    pub window_enabled: bool,
    pub window_tilemap: bool,
    pub lcd_enabled: bool,
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

    pub fn get(self) -> u8 {
        let bg_enabled: u8 = if self.bg_enabled { 0x01 } else { 0 };
        let object_enabled: u8 = if self.object_enabled { 0x02 } else { 0 };
        let object_size: u8 = if self.object_size { 0x04 } else { 0 };
        let bg_tilemap: u8 = if self.bg_tilemap { 0x08 } else { 0 };
        let addressing_mode: u8 = if self.addressing_mode { 0x10 } else { 0 };
        let window_enabled: u8 = if self.window_enabled { 0x20 } else { 0 };
        let window_tilemap: u8 = if self.window_tilemap { 0x40 } else { 0 };
        let lcd_enable: u8 = if self.lcd_enabled { 0x80 } else { 0 };

        bg_enabled
            | object_enabled
            | object_size
            | bg_tilemap
            | addressing_mode
            | window_enabled
            | window_tilemap
            | lcd_enable
    }

    pub fn set(&mut self, value: u8) {
        self.bg_enabled = value & 0x01 == 0x01;
        self.object_enabled = value & 0x02 == 0x02;
        self.object_size = value & 0x04 == 0x04;
        self.bg_tilemap = value & 0x08 == 0x08;
        self.addressing_mode = value & 0x10 == 0x10;
        self.window_enabled = value & 0x20 == 0x20;
        self.window_tilemap = value & 0x40 == 0x40;
        self.lcd_enabled = value & 0x80 == 0x80;
    }

    pub fn get_bg_address(self) -> u16 {
        if !self.bg_tilemap {
            TILEMAP_START_0
        } else {
            TILEMAP_START_1
        }
    }

    pub fn get_window_address(self) -> u16 {
        if !self.window_tilemap {
            TILEMAP_START_0
        } else {
            TILEMAP_START_1
        }
    }

    pub fn get_address(self, tile_index: u8) -> u16 {
        if self.addressing_mode {
            TILE_BLOCK_0 + (tile_index as u16 * TILE_OFFSET)
        } else if tile_index < 128 {
            TILE_BLOCK_2 + (tile_index as u16 * TILE_OFFSET)
        } else {
            TILE_BLOCK_1 + ((tile_index - 128) as u16 * TILE_OFFSET)
        }
    }
}
