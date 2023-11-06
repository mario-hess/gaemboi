/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    November 06, 2023
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
    pub escape_pressed: bool,
    pub a_pressed: bool,
    pub b_pressed: bool,
    pub select_pressed: bool,
    pub start_pressed: bool,
    pub up_pressed: bool,
    pub left_pressed: bool,
    pub down_pressed: bool,
    pub right_pressed: bool,
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
            escape_pressed: false,
            a_pressed: false,
            b_pressed: false,
            select_pressed: false,
            start_pressed: false,
            up_pressed: false,
            left_pressed: false,
            down_pressed: false,
            right_pressed: false,
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
                    self.escape_pressed = true;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.escape_pressed = true,
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
                    Some(Keycode::N) => self.a_pressed = true,
                    Some(Keycode::M) => self.b_pressed = true,
                    Some(Keycode::Backspace) => self.select_pressed = true,
                    Some(Keycode::Return) => self.start_pressed = true,
                    Some(Keycode::W) => self.up_pressed = true,
                    Some(Keycode::A) => self.left_pressed = true,
                    Some(Keycode::S) => self.down_pressed = true,
                    Some(Keycode::D) => self.right_pressed = true,
                    Some(Keycode::Up) => self.increase_scale(),
                    Some(Keycode::Down) => self.decrease_scale(),
                    _ => {}
                },
                Event::ControllerButtonDown { button, .. } => match button {
                    Button::A => self.a_pressed = true,
                    Button::B => self.b_pressed = true,
                    Button::DPadUp => self.up_pressed = true,
                    Button::DPadLeft => self.left_pressed = true,
                    Button::DPadDown => self.down_pressed = true,
                    Button::DPadRight => self.right_pressed = true,
                    Button::Start => self.start_pressed = true,
                    Button::Back => self.select_pressed = true,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::N) => self.a_pressed = false,
                    Some(Keycode::M) => self.b_pressed = false,
                    Some(Keycode::Backspace) => self.select_pressed = false,
                    Some(Keycode::Return) => self.start_pressed = false,
                    Some(Keycode::W) => self.up_pressed = false,
                    Some(Keycode::A) => self.left_pressed = false,
                    Some(Keycode::S) => self.down_pressed = false,
                    Some(Keycode::D) => self.right_pressed = false,
                    _ => {}
                },
                Event::ControllerButtonUp { button, .. } => match button {
                    Button::A => self.a_pressed = false,
                    Button::B => self.b_pressed = false,
                    Button::Back => self.select_pressed = false,
                    Button::Start => self.start_pressed = false,
                    Button::DPadUp => self.up_pressed = false,
                    Button::DPadLeft => self.left_pressed = false,
                    Button::DPadDown => self.down_pressed = false,
                    Button::DPadRight => self.right_pressed = false,
                    _ => {}
                },
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
