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

use crate::State;

pub struct EventHandler {
    pub file_path: Option<String>,
    pub state: State,
    pub pressed_a: bool,
    pub pressed_b: bool,
    pub pressed_select: bool,
    pub pressed_start: bool,
    pub pressed_up: bool,
    pub pressed_left: bool,
    pub pressed_down: bool,
    pub pressed_right: bool,
    pub window_scale: u32,
    pub previous_scale: u32,
    pub window_resized: bool,
    pub volume: u8,
    pub last_volume: u8,
    pub volume_slider: bool,
    pub fast_forward: u8,
    pub last_speed: u8,
    pub performance_mode: bool,
    pub show_waveform: bool,
    pub show_square_waves: bool,
    pub cpu_status_opened: bool,
    pub color_scheme_opened: bool,
    pub quit: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            file_path: None,
            state: State::Splash,
            pressed_a: false,
            pressed_b: false,
            pressed_select: false,
            pressed_start: false,
            pressed_up: false,
            pressed_left: false,
            pressed_down: false,
            pressed_right: false,
            window_scale: 4,
            previous_scale: 4,
            window_resized: false,
            volume: 50,
            last_volume: 50,
            volume_slider: true,
            fast_forward: 1,
            last_speed: 1,
            performance_mode: true,
            show_waveform: false,
            show_square_waves: false,
            cpu_status_opened: false,
            color_scheme_opened: false,
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
                Event::DropFile { filename, .. } => {
                    self.file_path = Some(filename);
                }
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
