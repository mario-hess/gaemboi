use super::{TILE_HEIGHT, TILE_WIDTH};

// The background can be made to scroll as a whole, writing to two registers.
pub struct Background {
    // These two registers specify the top-left coordinates
    // of the visible viewport within the background map
    x_scroll: u8,
    y_scroll: u8,
}

impl Background {
    pub fn new() -> Self {
        Self {
            x_scroll: 0,
            y_scroll: 0,
        }
    }

    pub fn get_x_scroll(&self) -> u8 {
        self.x_scroll
    }

    pub fn set_x_scroll(&mut self, value: u8) {
        self.x_scroll = value;
    }

    pub fn get_y_scroll(&self) -> u8 {
        self.y_scroll
    }

    pub fn set_y_scroll(&mut self, value: u8) {
        self.y_scroll = value;
    }

    // Calculate the pixels position within the tilemap
    pub fn tilemap_coordinates(&self, scan_x: u8, scan_y: u8) -> (u8, u8) {
        let x_pos = scan_x.wrapping_add(self.x_scroll);
        let y_pos = scan_y.wrapping_add(self.y_scroll);

        (x_pos, y_pos)
    }

    // Calculate the pixels position within the tile
    pub fn pixel_offsets(&self, x_coord: u8, y_coord: u8) -> (u8, u8) {
        let x_offset = 7 - (x_coord % TILE_WIDTH);
        // Since each line consists of 2 bytes, the offset has to be multiplied by 2
        let y_offset = (y_coord % TILE_HEIGHT) * 2;

        (x_offset, y_offset)
    }
}
