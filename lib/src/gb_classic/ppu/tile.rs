use super::{BLACK, DARK, LIGHT, TILE_HEIGHT, TILE_WIDTH, WHITE};

pub struct Tile {
    pub data: [[u8; TILE_WIDTH as usize]; TILE_HEIGHT as usize],
}

impl Tile {
    pub fn new(bytes: &[u8]) -> Self {
        let mut data = [[WHITE; TILE_WIDTH as usize]; TILE_HEIGHT as usize];

        for row in 0..TILE_HEIGHT as usize {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            for col in 0..TILE_WIDTH as usize {
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
