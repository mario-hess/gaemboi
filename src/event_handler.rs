/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    October 20, 2023
 */
use sdl2::{controller::Button, event::Event, keyboard::Keycode, EventPump};

pub struct EventHandler {
    pub key_pressed: Option<Keycode>,
    pub button_pressed: Option<Button>,
    pub file_dropped: Option<String>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            key_pressed: None,
            button_pressed: None,
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
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::F) => self.key_pressed = Some(Keycode::F),
                    Some(Keycode::C) => self.key_pressed = Some(Keycode::C),
                    Some(Keycode::Backspace) => self.key_pressed = Some(Keycode::Backspace),
                    Some(Keycode::Return) => self.key_pressed = Some(Keycode::Return),
                    Some(Keycode::W) => self.key_pressed = Some(Keycode::W),
                    Some(Keycode::A) => self.key_pressed = Some(Keycode::A),
                    Some(Keycode::S) => self.key_pressed = Some(Keycode::S),
                    Some(Keycode::D) => self.key_pressed = Some(Keycode::D),
                    _ => {}
                },
                Event::KeyUp { .. } => self.key_pressed = None,
                Event::ControllerButtonDown { button, .. } => match button {
                    Button::A => self.button_pressed = Some(Button::A),
                    Button::B => self.button_pressed = Some(Button::B),
                    Button::DPadUp => self.button_pressed = Some(Button::DPadUp),
                    Button::DPadLeft => self.button_pressed = Some(Button::DPadLeft),
                    Button::DPadDown => self.button_pressed = Some(Button::DPadDown),
                    Button::DPadRight => self.button_pressed = Some(Button::DPadRight),
                    Button::Start => self.button_pressed = Some(Button::Start),
                    Button::Back => self.button_pressed = Some(Button::Back),
                    _ => {}
                },
                Event::ControllerButtonUp { .. } => self.button_pressed = None,
                Event::DropFile { filename, .. } => self.file_dropped = Some(filename),
                _ => {}
            };
        }
    }
}
