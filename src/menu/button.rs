/**
 * @file    menu/button.rs
 * @brief   Handles menu buttons.
 * @author  Mario Hess
 * @date    October 27, 2023
 */
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window as SDL_Window;

pub const BTN_WIDTH: i32 = 32;
pub const BTN_HEIGHT: i32 = 16;

#[derive(Copy, Clone)]
pub enum ButtonType {
    Open,
    Keys,
    Back,
    Exit,
}

#[derive(Copy, Clone)]
pub enum ButtonState {
    Default,
    Hovered,
    Clicked,
}

#[derive(Copy, Clone)]
pub struct Button {
    pub button_type: ButtonType,
    pub button_state: ButtonState,
    pub default_rect: Rect,
    pub hovered_rect: Rect,
    pub clicked_rect: Rect,
    pub dest_rect: Rect,
    pub hovered: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(button_type: ButtonType, default_rect: Rect, dest_rect: Rect) -> Self {
        let button_state = ButtonState::Default;
        let hovered_rect = Rect::new(
            default_rect.x + BTN_WIDTH,
            default_rect.y,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );

        let clicked_rect = Rect::new(
            default_rect.x + BTN_WIDTH * 2,
            default_rect.y,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );

        let hovered = false;
        let clicked = false;

        Self {
            button_type,
            button_state,
            default_rect,
            hovered_rect,
            clicked_rect,
            dest_rect,
            hovered,
            clicked,
        }
    }

    pub fn check_hovered(&self, mouse_x: &i32, mouse_y: &i32) -> bool {
        mouse_x >= &self.dest_rect.left()
            && mouse_x < &self.dest_rect.right()
            && mouse_y >= &self.dest_rect.top()
            && mouse_y < &self.dest_rect.bottom()
    }

    pub fn draw(&self, canvas: &mut Canvas<SDL_Window>, texture: &Texture, dest_rect: Rect) {
        let rect = match self.button_state {
            ButtonState::Default => self.default_rect,
            ButtonState::Hovered => self.hovered_rect,
            ButtonState::Clicked => self.clicked_rect,
        };

        canvas.copy(texture, rect, dest_rect).unwrap();
    }
}
