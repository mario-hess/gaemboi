/**
 * @file    window.rs
 * @brief   Handles window management.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, CanvasBuilder, TextureCreator},
    rwops::RWops,
    ttf::{Font, Sdl2TtfContext},
    video::{Window as SDL_Window, WindowContext},
    VideoSubsystem,
};

use crate::ppu::WHITE;

pub struct Window<'a> {
    pub canvas: Canvas<SDL_Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    font: Font<'a, 'static>,
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
        let bytes = include_bytes!("../../fonts/OpenSans-Regular.ttf");
        let rw_bytes = RWops::from_bytes(bytes).unwrap();
        let font_size = 20;

        let window = video_subsystem
            .window(
                title,
                width as u32 * scale as u32,
                height as u32 * scale as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = CanvasBuilder::new(window).accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();

        canvas
            .set_logical_size(width as u32, height as u32)
            .unwrap();

        let font = ttf_context
            .load_font_from_rwops(rw_bytes, font_size)
            .unwrap();

        Self {
            canvas,
            texture_creator,
            font,
        }
    }

    pub fn render_text(&mut self, text: &str, color: Color) {
        let text_surface = self.font.render(text).blended(color).unwrap();

        let text_texture = self
            .texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        let position = Point::new(136, 0);

        let texture_query = text_texture.query();
        let target_rect = Rect::new(
            position.x(),
            position.y(),
            texture_query.width / 4,
            texture_query.height / 4,
        );

        self.canvas.copy(&text_texture, None, target_rect).unwrap();
    }
}

pub fn clear_canvas(canvas: &mut Canvas<SDL_Window>) {
    canvas.set_draw_color(WHITE);
    canvas.clear();
}
