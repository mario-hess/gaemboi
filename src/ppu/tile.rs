use crate::ppu::{BLACK, DARK, LIGHT, WHITE};
use sdl2::pixels::Color;

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;

// In the Gameboy’s 2BPP format, 2 bytes make up a row of 8 pixels.
// Each bit of the first byte is combined with the bit at the same
// position of the second byte to calculate the color number.
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

                // The first byte specifies the least significant bit of the color ID of each pixel,
                // and the second byte specifies the most significant bit.
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
