/**
 * @file    joypad.rs
 * @brief   Handles user input.
 * @author  Mario Hess
 * @date    November 06, 2023
 */
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
        self.up = event_handler.up_pressed;
        self.left = event_handler.left_pressed;
        self.down = event_handler.down_pressed;
        self.right = event_handler.right_pressed;
        self.a = event_handler.a_pressed;
        self.b = event_handler.b_pressed;
        self.select = event_handler.select_pressed;
        self.start = event_handler.start_pressed;
    }
}
