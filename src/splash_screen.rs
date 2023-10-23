/**
 * @file    splash_screen.rs
 * @brief   Display splash screen.
 * @author  Mario Hess
 * @date    October 23, 2023
 */
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
};

use crate::window::{clear_canvas, Window};

pub fn run(viewport: &mut Window) {
    // Include splash in binaries.
    let bytes = include_bytes!("../images/splash.png");
    let texture = viewport.texture_creator.load_texture_bytes(bytes).unwrap();

    let splash_width = texture.query().width;
    let splash_height = texture.query().height;

    let splash_position = Point::new(splash_width as i32 / 2, splash_height as i32 / 2);

    clear_canvas(&mut viewport.canvas);

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
