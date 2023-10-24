use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window as SDL_Window;

pub const BTN_WIDTH: i32 = 32;
pub const BTN_HEIGHT: i32 = 16;

enum ButtonState {
    Default,
    Hover,
    Click,
}

pub struct Button {
    pub default: Rect,
    pub hover: Rect,
    pub clicked: Rect,
    pub dest: Rect,
}

impl Button {
    pub fn new(default: Rect, dest: Rect) -> Self {
        let hover = Rect::new(
            default.x + BTN_WIDTH,
            default.y,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let clicked = Rect::new(
            default.x + BTN_WIDTH * 2,
            default.y,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );

        Self {
            default,
            hover,
            clicked,
            dest,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<SDL_Window>, texture: &Texture, src: Rect, dest: Rect) {
        canvas.copy(texture, src, dest).unwrap();
    }
}
