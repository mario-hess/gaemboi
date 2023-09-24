/**
 * @file    keyboard.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct EventHandler {
    pub event_key: Option<Keycode>,
    pub event_file: Option<String>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            event_key: None,
            event_file: None,
        }
    }

    pub fn poll(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.event_key = Some(Keycode::Escape),
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.event_key = Some(Keycode::Num1),
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.event_key = None,
                Event::DropFile { filename, .. } => self.event_file = Some(filename),

                _ => {}
            };
        }
    }
}
