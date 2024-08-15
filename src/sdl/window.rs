/**
 * @file    window.rs
 * @brief   Handles window management.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
/*
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window as SDL_Window, WindowContext},
    VideoSubsystem,
};

use crate::ppu::WHITE;

pub struct Window {
    pub canvas: Canvas<SDL_Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Window {
    pub fn build(
        video_subsystem: &VideoSubsystem,
        title: &str,
        width: usize,
        height: usize,
        scale: usize,
    ) -> Self {
        let window = video_subsystem
            .window(
                title,
                width as u32 * scale as u32,
                height as u32 * scale as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().accelerated().build().unwrap();

        canvas
            .set_logical_size(width as u32, height as u32)
            .unwrap();

        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(WHITE);
        canvas.clear();
        canvas.present();

        Self {
            canvas,
            texture_creator,
        }
    }
}

pub fn clear_canvas(canvas: &mut Canvas<SDL_Window>) {
    canvas.set_draw_color(WHITE);
    canvas.clear();
}
*/
