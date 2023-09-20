/**
 * @file    ppu/screen.rs
 * @brief   Represents and manages the screen display.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCALE: usize = 4;

pub struct Screen {
    viewport: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
