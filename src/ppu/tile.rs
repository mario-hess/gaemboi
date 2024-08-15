use egui_sdl2_gl::sdl2::pixels::Color;

use crate::ppu::{BLACK, DARK, LIGHT, WHITE};

pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = TILE_WIDTH;

pub struct Tile {
    pub data: [[Color; TILE_WIDTH]; TILE_HEIGHT],
}

impl Tile {
    pub fn new(bytes: &[u8]) -> Self {
        let mut data = [[WHITE; TILE_WIDTH]; TILE_HEIGHT];

        for row in 0..TILE_HEIGHT {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            for col in 0..TILE_WIDTH {
                let bit1 = (first_byte >> (7 - col)) & 0x01;
                let bit2 = (second_byte >> (7 - col)) & 0x01;

                data[row][col] = match (bit2, bit1) {
                    (0, 0) => WHITE,
                    (0, 1) => LIGHT,
                    (1, 0) => DARK,
                    (1, 1) => BLACK,
                    _ => unreachable!(),
                };
            }
        }

        Self { data }
    }
}
