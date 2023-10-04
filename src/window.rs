/**
 * @file    window.rs
 * @brief   Handles window management.
 * @author  Mario Hess
 * @date    October 04, 2023
 */
use sdl2::render::{Canvas, CanvasBuilder, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window as SDL_Window, WindowContext};
use sdl2::VideoSubsystem;

use crate::ppu::tile::{TILE_HEIGHT, TILE_WIDTH};

pub struct Window<'a> {
    pub canvas: Canvas<SDL_Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub font: Font<'a, 'static>,
}

impl<'a> Window<'a> {
    pub fn build(
        video_subsystem: &VideoSubsystem,
        ttf_context: &'a Sdl2TtfContext,
        title: &str,
        width: usize,
        height: usize,
        scale: usize,
    ) -> Self {
        let font_path = "fonts/Early GameBoy.ttf".to_string();
        let font_size = 8;

        let window = video_subsystem
            .window(
                title,
                (width as u32 * TILE_WIDTH as u32) * scale as u32,
                (height as u32 * TILE_HEIGHT as u32) * scale as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = CanvasBuilder::new(window).accelerated().build().unwrap();

        canvas
            .set_logical_size(
                width as u32 * TILE_WIDTH as u32,
                height as u32 * TILE_HEIGHT as u32,
            )
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let font = ttf_context.load_font(font_path, font_size).unwrap();

        Self {
            canvas,
            texture_creator,
            font,
        }
    }
}
