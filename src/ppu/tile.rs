/*
 * @file    ppu/tile.rs
 * @brief   Constructs a tile from 16 bytes tile data.
 * @author  Mario Hess
 * @date    September 13, 2024
 */

use std::{cell::RefCell, rc::Rc};

use egui_sdl2_gl::egui::Color32;

use super::colors::Colors;

pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = TILE_WIDTH;

pub struct Tile {
    pub data: [[Color32; TILE_WIDTH]; TILE_HEIGHT],
}

impl Tile {
    pub fn new(bytes: &[u8], colors: Rc<RefCell<Colors>>) -> Self {
        let colors = colors.as_ref().borrow();
        let mut data = [[colors.white; TILE_WIDTH]; TILE_HEIGHT];

        for row in 0..TILE_HEIGHT {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            for col in 0..TILE_WIDTH {
                let bit1 = (first_byte >> (7 - col)) & 0x01;
                let bit2 = (second_byte >> (7 - col)) & 0x01;

                data[row][col] = match (bit2, bit1) {
                    (0, 0) => colors.white,
                    (0, 1) => colors.light,
                    (1, 0) => colors.dark,
                    (1, 1) => colors.black,
                    _ => unreachable!(),
                };
            }
        }

        Self { data }
    }
}
