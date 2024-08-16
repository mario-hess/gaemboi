/**
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    November 11, 2023
 */
use egui_sdl2_gl::{
    painter::Painter,
    sdl2::{controller::Button, event::Event, keyboard::Keycode, video::Window, EventPump},
    EguiStateHandler,
};

pub struct EventHandler {
    pub pressed_escape: bool,
    pub pressed_a: bool,
    pub pressed_b: bool,
    pub pressed_select: bool,
    pub pressed_start: bool,
    pub pressed_up: bool,
    pub pressed_left: bool,
    pub pressed_down: bool,
    pub pressed_right: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_btn_down: bool,
    pub mouse_btn_up: bool,
    pub window_scale: u32,
    pub previous_scale: u32,
    pub fast_forward: u8,
    pub window_resized: bool,
    pub volume: u8,
    pub quit: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            pressed_escape: false,
            pressed_a: false,
            pressed_b: false,
            pressed_select: false,
            pressed_start: false,
            pressed_up: false,
            pressed_left: false,
            pressed_down: false,
            pressed_right: false,
            mouse_x: 0,
            mouse_y: 0,
            mouse_btn_down: false,
            mouse_btn_up: true,
            window_scale: 4,
            previous_scale: 4,
            fast_forward: 1,
            window_resized: false,
            volume: 50,
            quit: false,
        }
    }

    pub fn poll(
        &mut self,
        event_pump: &mut EventPump,
        egui_state: &mut EguiStateHandler,
        window: &Window,
        painter: &mut Painter,
    ) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::N) => self.pressed_a = true,
                    Some(Keycode::M) => self.pressed_b = true,
                    Some(Keycode::Backspace) => self.pressed_select = true,
                    Some(Keycode::Return) => self.pressed_start = true,
                    Some(Keycode::W) => self.pressed_up = true,
                    Some(Keycode::A) => self.pressed_left = true,
                    Some(Keycode::S) => self.pressed_down = true,
                    Some(Keycode::D) => self.pressed_right = true,
                    Some(Keycode::Up) => self.increase_scale(),
                    Some(Keycode::Down) => self.decrease_scale(),
                    Some(Keycode::Left) => self.decrease_volume(),
                    Some(Keycode::Right) => self.increase_volume(),
                    _ => {}
                },
                Event::ControllerButtonDown { button, .. } => match button {
                    Button::A => self.pressed_a = true,
                    Button::B => self.pressed_b = true,
                    Button::DPadUp => self.pressed_up = true,
                    Button::DPadLeft => self.pressed_left = true,
                    Button::DPadDown => self.pressed_down = true,
                    Button::DPadRight => self.pressed_right = true,
                    Button::Start => self.pressed_start = true,
                    Button::Back => self.pressed_select = true,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::N) => self.pressed_a = false,
                    Some(Keycode::M) => self.pressed_b = false,
                    Some(Keycode::Backspace) => self.pressed_select = false,
                    Some(Keycode::Return) => self.pressed_start = false,
                    Some(Keycode::W) => self.pressed_up = false,
                    Some(Keycode::A) => self.pressed_left = false,
                    Some(Keycode::S) => self.pressed_down = false,
                    Some(Keycode::D) => self.pressed_right = false,
                    _ => {}
                },
                Event::ControllerButtonUp { button, .. } => match button {
                    Button::A => self.pressed_a = false,
                    Button::B => self.pressed_b = false,
                    Button::Back => self.pressed_select = false,
                    Button::Start => self.pressed_start = false,
                    Button::DPadUp => self.pressed_up = false,
                    Button::DPadLeft => self.pressed_left = false,
                    Button::DPadDown => self.pressed_down = false,
                    Button::DPadRight => self.pressed_right = false,
                    _ => {}
                },
                Event::DropFile { filename, .. } => {}
                _ => egui_state.process_input(window, event, painter),
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

    fn increase_volume(&mut self) {
        if self.volume > 95 {
            return;
        }

        self.volume += 5;
    }

    fn decrease_volume(&mut self) {
        if self.volume < 5 {
            return;
        }

        self.volume -= 5;
    }
}
