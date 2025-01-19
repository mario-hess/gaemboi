/*
 * @file    ppu/tile.rs
 * @brief   Constructs a tile from 16 bytes tile data.
 * @author  Mario Hess
 * @date    September 13, 2024
 */

use egui_sdl2_gl::egui::Color32;

pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = TILE_WIDTH;

pub struct Tile {
    pub data: [[Color32; TILE_WIDTH]; TILE_HEIGHT],
}

impl Tile {
    pub fn new(bytes: &[u8], black: Color32, dark: Color32, light: Color32, white: Color32) -> Self {
        let mut data = [[white; TILE_WIDTH]; TILE_HEIGHT];

        for row in 0..TILE_HEIGHT {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            for col in 0..TILE_WIDTH {
                let bit1 = (first_byte >> (7 - col)) & 0x01;
                let bit2 = (second_byte >> (7 - col)) & 0x01;

                data[row][col] = match (bit2, bit1) {
                    (0, 0) => white,
                    (0, 1) => light,
                    (1, 0) => dark,
                    (1, 1) => black,
                    _ => unreachable!(),
                };
            }
        }

        Self { data }
    }
}
