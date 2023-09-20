/**
 * @file    ppu/tile.rs
 * @brief   Handles tile graphics in 2BPP format.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use crate::ppu::{BLACK, DARK, LIGHT, WHITE};
use sdl2::pixels::Color;

pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;

#[derive(Debug)]
pub struct Tile {
    pub data: [[Color; TILE_WIDTH]; TILE_HEIGHT],
}

impl Tile {
    pub fn new(bytes: [u8; 16]) -> Self {
        let mut data = [[WHITE; TILE_WIDTH]; TILE_HEIGHT];

        for row in 0..TILE_HEIGHT {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            let mut row_data = [WHITE; TILE_WIDTH];

            for (index, row_color) in row_data.iter_mut().enumerate().take(TILE_WIDTH) {
                let bit1 = (first_byte >> (7 - index)) & 0x01;
                let bit2 = (second_byte >> (7 - index)) & 0x01;

                // The first byte specifies the least significant bit of the color ID of
                // each pixel, and the second byte specifies the most significant bit.
                let color = match (bit2, bit1) {
                    (0, 0) => WHITE,
                    (0, 1) => LIGHT,
                    (1, 0) => DARK,
                    (1, 1) => BLACK,
                    _ => unreachable!(),
                };

                *row_color = color;
            }

            data[row] = row_data;
        }

        Self { data }
    }
}
