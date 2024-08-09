/**
 * @file    ppu/oam.rs
 * @brief   Handles the Object Attribute Memory
 * @author  Mario Hess
 * @date    May 23, 2024
 */

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub struct OAM {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_index: u8,
    pub attributes: u8,
}

impl OAM {
    pub fn new() -> Self {
        Self {
            y_pos: 0,
            x_pos: 0,
            tile_index: 0,
            attributes: 0,
        }
    }

    pub fn palette_enabled(&self) -> bool {
        self.attributes & 0x10 != 0
    }

    pub fn x_flip_enabled(&self) -> bool {
        self.attributes & 0x20 != 0
    }

    pub fn y_flip_enabled(&self) -> bool {
        self.attributes & 0x40 != 0
    }

    pub fn priority_enabled(&self) -> bool {
        self.attributes & 0x80 != 0
    }
}
