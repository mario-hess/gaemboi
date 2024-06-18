use super::{TILE_HEIGHT, TILE_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

/*
 * The window is sort of a second background layer on top of the background.
 * It is fairly limited: it has no transparency, it’s always a rectangle and
 * only the position of the top-left pixel can be controlled.
 */
pub struct Window {
    // Horizontal position of the window + 7
    x_coord: u8,
    // Vertical position of the window
    y_coord: u8,
    // Determines what window line is to be rendered on the current scanline
    line_counter: u8,
}

impl Window {
    pub fn new() -> Self {
        Self {
            x_coord: 0,
            y_coord: 0,
            line_counter: 0,
        }
    }

    pub fn get_x_coord(&self) -> u8 {
        self.x_coord
    }

    pub fn set_x_coord(&mut self, value: u8) {
        // Values lower than 7 cause strange edge cases to occur
        if value < 7 {
            return;
        }

        self.x_coord = value
    }

    pub fn get_y_coord(&self) -> u8 {
        self.y_coord
    }

    pub fn set_y_coord(&mut self, value: u8) {
        self.y_coord = value
    }

    pub fn reset_line_counter(&mut self) {
        self.line_counter = 0;
    }

    pub fn is_pixel_in_window(&self, enabled: bool, scan_x:u8, scan_y: u8) -> bool {
        // Determine if the current pixel is within the range of the window's
        // span on the x-axis
        let x_in_window = enabled && scan_x >= self.x_coord.wrapping_sub(7);
        // Determine if the current pixel is within the range of the window's
        // span on the y-axis
        let y_in_window = enabled && scan_y >= self.y_coord;

        x_in_window && y_in_window
    }

    // The window keeps an internal line counter that’s functionally similar to scan_y, and
    // increments alongside it. However, it only gets incremented when the window is visible.
    pub fn increase_line_counter(&mut self, enabled: bool, scan_y: u8) {
        // The Window is visible (if enabled) when both coordinates are in
        // the ranges x=0..166 and y=0..143 respectively. Values x=7, y=0 place the
        // Window at the top left of the screen, completely covering the background.
        let is_visible = enabled
            && self.x_coord - 7 < VIEWPORT_WIDTH as u8
            && self.y_coord < VIEWPORT_HEIGHT as u8
            && scan_y >= self.y_coord;

        if is_visible {
            self.line_counter = self.line_counter.saturating_add(1);
        }
    }

    // Calculate the pixels position within the tilemap
    pub fn tilemap_coordinates(&self, scan_x: u8) -> (u8, u8) {
        let x_offset = scan_x.wrapping_sub(self.x_coord.wrapping_sub(7));
        let y_offset = self.line_counter;

        (x_offset, y_offset)
    }

    // Calculate the pixels position within the tile
    pub fn pixel_offsets(&self, scan_x: u8, scan_y: u8) -> (u8, u8) {
        let x_offset = self.x_coord.wrapping_sub(scan_x) % TILE_WIDTH as u8;
        // Since each line consists of 2 bytes, the offset has to be multiplied by 2
        let y_offset = ((scan_y - self.y_coord) % TILE_HEIGHT as u8) * 2;

        (x_offset, y_offset)
    }
}
