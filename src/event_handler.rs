/*
 * @file    event_handler.rs
 * @brief   Manages keyboard input and key states.
 * @author  Mario Hess
 * @date    November 11, 2023
 */

use std::{cell::RefCell, rc::Rc};

use egui_sdl2_gl::{
    painter::Painter,
    sdl2::{controller::Button, event::Event, keyboard::Keycode, video::Window, EventPump},
    EguiStateHandler,
};

use crate::State;

pub struct EventHandler {
    pub file_path: Option<String>,
    pub state: State,
    pub a: Option<Keycode>,
    pub pressed_a: bool,
    pub b: Option<Keycode>,
    pub pressed_b: bool,
    pub select: Option<Keycode>,
    pub pressed_select: bool,
    pub start: Option<Keycode>,
    pub pressed_start: bool,
    pub up: Option<Keycode>,
    pub pressed_up: bool,
    pub left: Option<Keycode>,
    pub pressed_left: bool,
    pub down: Option<Keycode>,
    pub pressed_down: bool,
    pub right: Option<Keycode>,
    pub pressed_right: bool,
    pub window_scale: u32,
    pub previous_scale: u32,
    pub window_resized: bool,
    pub volume: u8,
    pub last_volume: u8,
    pub volume_slider: bool,
    pub fast_forward: Rc<RefCell<u32>>,
    pub performance_mode: bool,
    pub show_waveform: bool,
    pub show_square_waves: bool,
    pub cpu_status_opened: bool,
    pub keybindings_opened: bool,
    pub color_scheme_opened: bool,
    pub about_opened: bool,
    pub bug_report_opened: bool,
    pub quit: bool,
    pub rebinding_key: Option<&'static str>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            file_path: None,
            state: State::Splash,
            a: Some(Keycode::N),
            pressed_a: false,
            b: Some(Keycode::M),
            pressed_b: false,
            select: Some(Keycode::Backspace),
            pressed_select: false,
            start: Some(Keycode::Return),
            pressed_start: false,
            up: Some(Keycode::W),
            pressed_up: false,
            left: Some(Keycode::A),
            pressed_left: false,
            down: Some(Keycode::S),
            pressed_down: false,
            right: Some(Keycode::D),
            pressed_right: false,
            window_scale: 4,
            previous_scale: 4,
            window_resized: false,
            volume: 50,
            last_volume: 50,
            volume_slider: true,
            fast_forward: Rc::new(RefCell::new(1)),
            performance_mode: true,
            show_waveform: false,
            show_square_waves: false,
            cpu_status_opened: false,
            keybindings_opened: false,
            color_scheme_opened: false,
            about_opened: false,
            bug_report_opened: false,
            quit: false,
            rebinding_key: None,
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

                Event::KeyDown { keycode, .. } => {
                    // Handle key rebinding
                    if let Some(key_to_rebind) = self.rebinding_key {
                        if let Some(new_key) = keycode {
                            self.rebind_key(key_to_rebind, new_key);
                            self.rebinding_key = None; // Finish rebinding
                        }
                    } else {
                        // Normal key handling
                        match keycode {
                            Some(key) if Some(key) == self.a => self.pressed_a = true,
                            Some(key) if Some(key) == self.b => self.pressed_b = true,
                            Some(key) if Some(key) == self.select => self.pressed_select = true,
                            Some(key) if Some(key) == self.start => self.pressed_start = true,
                            Some(key) if Some(key) == self.up => self.pressed_up = true,
                            Some(key) if Some(key) == self.left => self.pressed_left = true,
                            Some(key) if Some(key) == self.down => self.pressed_down = true,
                            Some(key) if Some(key) == self.right => self.pressed_right = true,
                            Some(Keycode::Up) => self.increase_scale(),
                            Some(Keycode::Down) => self.decrease_scale(),
                            Some(Keycode::Left) => self.decrease_volume(),
                            Some(Keycode::Right) => self.increase_volume(),
                            _ => {}
                        }
                    }
                }
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
                    Some(key) if Some(key) == self.a => self.pressed_a = false,
                    Some(key) if Some(key) == self.b => self.pressed_b = false,
                    Some(key) if Some(key) == self.select => self.pressed_select = false,
                    Some(key) if Some(key) == self.start => self.pressed_start = false,
                    Some(key) if Some(key) == self.up => self.pressed_up = false,
                    Some(key) if Some(key) == self.left => self.pressed_left = false,
                    Some(key) if Some(key) == self.down => self.pressed_down = false,
                    Some(key) if Some(key) == self.right => self.pressed_right = false,
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

    pub fn rebind_key(&mut self, key: &'static str, new_keycode: Keycode) {
        let new_keycode = Some(new_keycode);

        for binding in [
            &mut self.a,
            &mut self.b,
            &mut self.select,
            &mut self.start,
            &mut self.up,
            &mut self.down,
            &mut self.left,
            &mut self.right,
        ]
        .iter_mut()
        {
            if **binding == new_keycode {
                **binding = None;
            }
        }

        match key {
            "A" => self.a = new_keycode,
            "B" => self.b = new_keycode,
            "Select" => self.select = new_keycode,
            "Start" => self.start = new_keycode,
            "Up" => self.up = new_keycode,
            "Down" => self.down = new_keycode,
            "Left" => self.left = new_keycode,
            "Right" => self.right = new_keycode,
            _ => {}
        }
    }
}
