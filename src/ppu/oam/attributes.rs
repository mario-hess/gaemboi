#![cfg_attr(rustfmt, rustfmt::skip)]
/*
 * @file    ppu/oam/attributes.rs
 * @brief   OAM Attributes register
 * @author  Mario Hess
 * @date    September 13, 2024
 */

const CGB_PALETTE_MASK: u8 = 0x07;
const CGB_BANK_MASK: u8 = 0x08;
const DMG_PALETTE_MASK: u8 = 0x10;
const X_FLIP_MASK: u8 = 0x20;
const Y_FLIP_MASK: u8 = 0x40;
const PRIORITY_MASK: u8 = 0x80;

#[derive(Copy, Clone)]
pub struct Attributes {
    cgb_palette: u8,
    cgb_bank: bool,
    dmg_palette: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            cgb_palette: 0,
            cgb_bank: false,
            dmg_palette: false,
            x_flip: false,
            y_flip: false,
            priority: false,
        }
    }

    pub fn dmg_palette_enabled(&self) -> bool { self.dmg_palette }
    pub fn x_flip_enabled(&self) -> bool { self.x_flip }
    pub fn y_flip_enabled(&self) -> bool { self.y_flip }
    pub fn priority_enabled(&self) -> bool { self.priority }
}

impl std::convert::From<&Attributes> for u8 {
    fn from(attributes: &Attributes) -> u8 {
        attributes.cgb_palette & CGB_PALETTE_MASK
            | (if attributes.cgb_bank { CGB_BANK_MASK } else { 0 })
            | (if attributes.dmg_palette { DMG_PALETTE_MASK } else { 0 })
            | (if attributes.x_flip { X_FLIP_MASK } else { 0 })
            | (if attributes.y_flip { Y_FLIP_MASK } else { 0 })
            | (if attributes.priority { PRIORITY_MASK} else { 0 })
    }
}

impl std::convert::From<u8> for Attributes {
    fn from(byte: u8) -> Self {
        Self {
            cgb_palette: byte & CGB_PALETTE_MASK,
            cgb_bank: (byte & CGB_BANK_MASK) != 0,
            dmg_palette: (byte & DMG_PALETTE_MASK) != 0,
            x_flip: (byte & X_FLIP_MASK) != 0,
            y_flip: (byte & Y_FLIP_MASK) != 0,
            priority: (byte & PRIORITY_MASK) != 0,
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}
