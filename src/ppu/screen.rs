pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const SCALE: usize = 4;

pub struct Screen {
    viewport: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
