/*
 * @file    ppu/oam.rs
 * @brief   Handles the Object Attribute Memory
 * @author  Mario Hess
 * @date    May 23, 2024
 */

mod attributes;
use attributes::Attributes;

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub struct OAM {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    pub attributes: Attributes,
}

impl OAM {
    pub fn new() -> Self {
        Self {
            y_pos: 0,
            x_pos: 0,
            tile_index: 0,
            attributes: Attributes::new(),
        }
    }

    pub fn get_y_pos(&self) -> u8 { self.y_pos }
    pub fn set_y_pos(&mut self, value: u8) { self.y_pos = value; }

    pub fn get_x_pos(&self) -> u8 { self.x_pos }
    pub fn set_x_pos(&mut self, value: u8) { self.x_pos = value; }

    pub fn get_tile_index(&self) -> u8 { self.tile_index }
    pub fn set_tile_index(&mut self, value: u8) { self.tile_index = value; }

    pub fn get_attributes(&self) -> u8 { (&self.attributes).into() }
    pub fn set_attributes(&mut self, value: u8) { self.attributes = value.into() }
}
