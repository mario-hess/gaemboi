/**
 * @file    boot_sequence.rs
 * @brief   Custom boot sequence.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    EventPump,
};

use crate::{
    event_handler::EventHandler,
    sdl::window::{clear_canvas, Window},
    MachineState,
};

pub fn run(window: &mut Window, event_handler: &mut EventHandler, event_pump: &mut EventPump) {
    let frame_duration = std::time::Duration::from_millis((1000.0 / 30.0) as u64);

    // Include logo in binaries.
    let bytes = include_bytes!("../images/logo.png");

    let texture = window.texture_creator.load_texture_bytes(bytes).unwrap();

    let logo_width = texture.query().width;
    let logo_height = texture.query().height;
    let scroll_speed = 1;

    let mut logo_position = Point::new(logo_width as i32 / 2, 0);

    let start_time = std::time::Instant::now();

    while !event_handler.pressed_escape {
        event_handler.poll(event_pump);
        event_handler.check_resized(&mut window.canvas);

        match event_handler.machine_state {
            MachineState::Boot => {
                let frame_start_time = std::time::Instant::now();

                clear_canvas(&mut window.canvas);

                logo_position.y += scroll_speed;

                if logo_position.y > logo_height as i32 / 2 {
                    logo_position.y = logo_height as i32 / 2;

                    if start_time.elapsed() >= core::time::Duration::from_millis(5000) {
                        event_handler.machine_state = MachineState::Play;
                        break;
                    }
                }

                window
                    .canvas
                    .copy(
                        &texture,
                        None,
                        Rect::from_center(logo_position, logo_width, logo_height),
                    )
                    .unwrap();

                window.canvas.present();

                let elapsed_time = frame_start_time.elapsed();
                if elapsed_time < frame_duration {
                    std::thread::sleep(frame_duration - elapsed_time);
                }
            }
            _ => {
                break;
            }
        }
    }
}
