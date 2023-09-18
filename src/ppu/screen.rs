const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const SCALE: u8 = 4;

struct Screen {
    viewport: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
