/**
 * @file    ppu/oam.rs
 * @brief   Handles the Object Attribute Memory
 * @author  Mario Hess
 * @date    September 22, 2023
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
}
