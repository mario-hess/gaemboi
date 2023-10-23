/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    October 23, 2023
 */
use sdl2::{controller::Button, event::Event, keyboard::Keycode, EventPump};

use crate::{
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    window::Window,
    State,
};

pub struct EventHandler {
    pub mode: State,
    pub key_pressed: Option<Keycode>,
    pub button_pressed: Option<Button>,
    pub file_dropped: Option<String>,
    pub window_scale: u32,
    pub window_resized: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            mode: State::Splash,
            key_pressed: None,
            button_pressed: None,
            file_dropped: None,
            window_scale: 4,
            window_resized: false,
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
                    Some(Keycode::N) => self.key_pressed = Some(Keycode::N),
                    Some(Keycode::M) => self.key_pressed = Some(Keycode::M),
                    Some(Keycode::Backspace) => self.key_pressed = Some(Keycode::Backspace),
                    Some(Keycode::Return) => self.key_pressed = Some(Keycode::Return),
                    Some(Keycode::W) => self.key_pressed = Some(Keycode::W),
                    Some(Keycode::A) => self.key_pressed = Some(Keycode::A),
                    Some(Keycode::S) => self.key_pressed = Some(Keycode::S),
                    Some(Keycode::D) => self.key_pressed = Some(Keycode::D),
                    Some(Keycode::Up) => self.increase_scale(),
                    Some(Keycode::Down) => self.decrease_scale(),
                    _ => {}
                },
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
                Event::KeyUp { .. } => self.key_pressed = None,
                Event::ControllerButtonUp { .. } => self.button_pressed = None,
                Event::DropFile { filename, .. } => self.file_dropped = Some(filename),
                _ => {}
            };
        }
    }

    fn increase_scale(&mut self) {
        if self.window_scale < 6 {
            self.window_scale += 1;
        }

        self.window_resized = true;
    }

    fn decrease_scale(&mut self) {
        if self.window_scale > 1 {
            self.window_scale -= 1;
        }

        self.window_resized = true;
    }

    pub fn check_resized(&mut self, viewport: &mut Window) {
        if !self.window_resized {
            return;
        }

        viewport
            .canvas
            .window_mut()
            .set_size(
                (VIEWPORT_WIDTH) as u32 * self.window_scale,
                (VIEWPORT_HEIGHT as u32) * self.window_scale,
            )
            .unwrap();

        self.window_resized = false;
    }
}
