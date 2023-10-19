/**
 * @file    joypad.rs
 * @brief   Handles user input.
 * @author  Mario Hess
 * @date    October 19, 2023
 */
use sdl2::keyboard::Keycode;

const A_RIGHT_POS: u8 = 0;
const B_LEFT_BOS: u8 = 1;
const SELECT_UP_POS: u8 = 2;
const START_DOWN_POS: u8 = 3;
const SELECT_DPAD_POS: u8 = 4;
const SELECT_BUTTONS_POS: u8 = 5;

#[derive(Default)]
pub struct Joypad {
    a: bool,
    b: bool,
    select: bool,
    start: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    select_dpad: bool,
    select_buttons: bool,
}

impl Joypad {
    pub fn set(&mut self, value: u8) {
        self.select_dpad = (value >> SELECT_DPAD_POS) & 0b1 == 0;
        self.select_buttons = (value >> SELECT_BUTTONS_POS) & 0b1 == 0;
    }

    pub fn get(&self) -> u8 {
        let mut input: u8 = 0;

        if self.select_buttons {
            input = (if self.a { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.b { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.select { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.start { 0 } else { 1 }) << START_DOWN_POS
        }

        if self.select_dpad {
            input = (if self.right { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.left { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.up { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.down { 0 } else { 1 }) << START_DOWN_POS
        }

        input
    }

    pub fn handle_input(&mut self, key: &Option<Keycode>) {
        match key {
            Some(Keycode::F) => {
                self.a = true;
                self.select_buttons = true;
            }
            Some(Keycode::C) => {
                self.b = true;
                self.select_buttons = true;
            }
            Some(Keycode::Num9) => {
                self.select = true;
                self.select_buttons = true;
            }
            Some(Keycode::Num0) => {
                self.start = true;
                self.select_buttons = true;
            }
            Some(Keycode::W) => {
                self.up = true;
                self.select_dpad = true;
            }
            Some(Keycode::S) => {
                self.down = true;
                self.select_dpad = true;
            }
            Some(Keycode::A) => {
                self.left = true;
                self.select_dpad = true;
            }
            Some(Keycode::D) => {
                self.right = true;
                self.select_dpad = true;
            }
            None => {
                self.a = false;
                self.b = false;
                self.select = false;
                self.start = false;
                self.up = false;
                self.down = false;
                self.left = false;
                self.right = false;
            }
            _ => {}
        }
    }
}
