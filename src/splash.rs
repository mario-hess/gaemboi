/**
 * @file    splash.rs
 * @brief   Display splash screen.
 * @author  Mario Hess
 * @date    October 11, 2023
 */
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
};

use crate::{ppu::WHITE, window::Window};

pub fn run(viewport: &mut Window) {
    let texture = viewport
        .texture_creator
        .load_texture("images/splash.png")
        .unwrap();

    let splash_width = texture.query().width;
    let splash_height = texture.query().height;

    let splash_position = Point::new(splash_width as i32 / 2, splash_height as i32 / 2);

    viewport.canvas.set_draw_color(WHITE);
    viewport.canvas.clear();

    viewport
        .canvas
        .copy(
            &texture,
            None,
            Rect::from_center(splash_position, splash_width, splash_height),
        )
        .unwrap();

    viewport.canvas.present();
}
