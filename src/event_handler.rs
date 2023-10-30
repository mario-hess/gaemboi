/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    October 30, 2023
 */
use sdl2::{
    controller::Button, event::Event, keyboard::Keycode, render::Canvas,
    video::Window as SDL_Window, EventPump,
};

use crate::{
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    MachineState,
};

pub struct EventHandler {
    pub machine_state: MachineState,
    pub key_pressed: Option<Keycode>,
    pub button_pressed: Option<Button>,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_btn_down: bool,
    pub mouse_btn_up: bool,
    pub file_path: Option<String>,
    pub window_scale: u32,
    pub window_resized: bool,
    pub quit: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            machine_state: MachineState::Menu,
            key_pressed: None,
            button_pressed: None,
            mouse_x: 0,
            mouse_y: 0,
            mouse_btn_down: false,
            mouse_btn_up: true,
            file_path: None,
            window_scale: 4,
            window_resized: false,
            quit: false,
        }
    }

    pub fn poll(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit = true;
                    self.key_pressed = Some(Keycode::Escape);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.key_pressed = Some(Keycode::Escape),
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                Event::MouseButtonDown { .. } => {
                    self.mouse_btn_down = true;
                    self.mouse_btn_up = false;
                }
                Event::MouseButtonUp { .. } => {
                    self.mouse_btn_up = true;
                }
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
                Event::DropFile { filename, .. } => self.file_path = Some(filename),
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

    pub fn check_resized(&mut self, canvas: &mut Canvas<SDL_Window>) {
        if !self.window_resized {
            return;
        }

        canvas
            .window_mut()
            .set_size(
                (VIEWPORT_WIDTH) as u32 * self.window_scale,
                (VIEWPORT_HEIGHT as u32) * self.window_scale,
            )
            .unwrap();

        self.window_resized = false;
    }

    pub fn reset_mouse_buttons(&mut self) {
        self.mouse_btn_up = true;
        self.mouse_btn_down = false;
    }
}
