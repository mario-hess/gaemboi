/**
 * @file    boot_sequence.rs
 * @brief   Custom boot sequence.
 * @author  Mario Hess
 * @date    October 04, 2023
 */
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::EventPump;

use crate::config::Config;
use crate::event_handler::EventHandler;
use crate::ppu::WHITE;
use crate::window::Window;

pub fn run(
    viewport: &mut Window,
    event_handler: &mut EventHandler,
    event_pump: &mut EventPump,
    config: &mut Config,
) {
    let texture = viewport
        .texture_creator
        .load_texture("images/logo.png")
        .unwrap();

    let logo_width = texture.query().width;
    let logo_height = texture.query().height;
    let scroll_speed = 1;

    let mut logo_position = Point::new(logo_width as i32 / 2, 0);

    let frame_duration = std::time::Duration::from_millis((1000.0 / 30.0) as u64);

    while event_handler.key_pressed != Some(Keycode::Escape) {
        let frame_start_time = std::time::Instant::now();

        event_handler.poll(event_pump);
        if !config.boot_sequence_enabled {
            break;
        }

        viewport.canvas.set_draw_color(WHITE);
        viewport.canvas.clear();

        logo_position.y += scroll_speed;

        if logo_position.y > logo_height as i32 / 2 && config.boot_sequence_enabled {
            logo_position.y = logo_height as i32 / 2;
            config.boot_sequence_enabled = false;
            std::thread::sleep(std::time::Duration::from_millis(3000));
        }

        viewport
            .canvas
            .copy(
                &texture,
                None,
                Rect::from_center(logo_position, logo_width, logo_height),
            )
            .unwrap();

        viewport.render_text("booting...", Color::RGB(0, 255, 0));
        viewport.canvas.present();

        let elapsed_time = frame_start_time.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }
}
