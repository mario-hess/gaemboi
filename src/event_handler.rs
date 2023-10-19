/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    October 19, 2023
 */
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct EventHandler {
    pub key_pressed: Option<Keycode>,
    pub file_dropped: Option<String>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            key_pressed: None,
            file_dropped: None,
        }
    }

    pub fn poll(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.key_pressed = Some(Keycode::Escape),
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.key_pressed = Some(Keycode::Num1),
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => self.key_pressed = Some(Keycode::F),
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => self.key_pressed = Some(Keycode::C),
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => self.key_pressed = Some(Keycode::Num9),
                Event::KeyUp {
                    keycode: Some(Keycode::Num9),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => self.key_pressed = Some(Keycode::Num0),
                Event::KeyUp {
                    keycode: Some(Keycode::Num0),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.key_pressed = Some(Keycode::W),
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.key_pressed = Some(Keycode::S),
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.key_pressed = Some(Keycode::A),
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => self.key_pressed = None,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.key_pressed = Some(Keycode::D),
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => self.key_pressed = None,
                Event::DropFile { filename, .. } => self.file_dropped = Some(filename),

                _ => {}
            };
        }
    }
}
