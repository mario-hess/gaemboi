/**
 * @file    joypad.rs
 * @brief   Handles user input.
 * @author  Mario Hess
 * @date    October 22, 2023
 */
use sdl2::{controller::Button, keyboard::Keycode};

use crate::event_handler::EventHandler;

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
        if self.select_buttons {
            return (if self.a { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.b { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.select { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.start { 0 } else { 1 }) << START_DOWN_POS;
        }

        if self.select_dpad {
            return (if self.right { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.left { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.up { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.down { 0 } else { 1 }) << START_DOWN_POS;
        }

        0x00
    }

    pub fn handle_input(&mut self, event_handler: &EventHandler) {
        // Keyboard input
        if let Some(keycode) = event_handler.key_pressed {
            match keycode {
                Keycode::W | Keycode::S | Keycode::A | Keycode::D => {
                    self.up = keycode == Keycode::W;
                    self.down = keycode == Keycode::S;
                    self.left = keycode == Keycode::A;
                    self.right = keycode == Keycode::D;
                    self.select_dpad = true;
                }
                Keycode::N | Keycode::M | Keycode::Backspace | Keycode::Return => {
                    self.a = keycode == Keycode::N;
                    self.b = keycode == Keycode::M;
                    self.select = keycode == Keycode::Backspace;
                    self.start = keycode == Keycode::Return;
                    self.select_buttons = true;
                }
                _ => {}
            }
        }

        // Controller input
        if let Some(button) = event_handler.button_pressed {
            match button {
                Button::DPadUp | Button::DPadLeft | Button::DPadDown | Button::DPadRight => {
                    self.up = button == Button::DPadUp;
                    self.down = button == Button::DPadDown;
                    self.left = button == Button::DPadLeft;
                    self.right = button == Button::DPadRight;
                    self.select_dpad = true;
                }
                Button::A | Button::B | Button::Start | Button::Back => {
                    self.a = button == Button::A;
                    self.b = button == Button::B;
                    self.select = button == Button::Back;
                    self.start = button == Button::Start;
                    self.select_buttons = true;
                }
                _ => {}
            }
        }

        // Reset if no input is detected
        if event_handler.key_pressed.is_none() && event_handler.button_pressed.is_none() {
            self.a = false;
            self.b = false;
            self.select = false;
            self.start = false;
            self.up = false;
            self.down = false;
            self.left = false;
            self.right = false;
        }
    }
}
