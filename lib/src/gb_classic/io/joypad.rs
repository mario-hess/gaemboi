use crate::{
    InputEvent, InputProvider,
    utils::input::{InputAction, InputButton},
};
use std::collections::HashMap;

const A_RIGHT_MASK: u8 = 0x01;
const B_LEFT_MASK: u8 = 0x02;
const SELECT_UP_MASK: u8 = 0x04;
const START_DOWN_MASK: u8 = 0x08;
const SELECT_DPAD_MASK: u8 = 0x10;
const SELECT_BUTTONS_MASK: u8 = 0x20;
const DEFAULT_STATE: u8 = 0xCF;
const BASE_MASK: u8 = 0xC0;

pub struct Joypad {
    buttons: HashMap<InputButton, InputAction>,
    select_dpad: bool,
    select_buttons: bool,
    pub input_provider: Option<Box<dyn InputProvider>>,
}

impl Joypad {
    pub fn new() -> Self {
        let buttons = [
            InputButton::Right,
            InputButton::Left,
            InputButton::Up,
            InputButton::Down,
            InputButton::A,
            InputButton::B,
            InputButton::Select,
            InputButton::Start,
        ]
        .iter()
        .map(|&button| (button, InputAction::Release))
        .collect();

        Self {
            buttons,
            select_dpad: false,
            input_provider: None,
            select_buttons: false,
        }
    }

    pub fn set(&mut self, value: u8) {
        self.select_dpad = (value & SELECT_DPAD_MASK) == 0;
        self.select_buttons = (value & SELECT_BUTTONS_MASK) == 0;
    }

    pub fn get(&self) -> u8 {
        if self.select_dpad {
            return BASE_MASK
                | (if self.is_button_pressed(InputButton::Right) {
                    0
                } else {
                    A_RIGHT_MASK
                })
                | (if self.is_button_pressed(InputButton::Left) {
                    0
                } else {
                    B_LEFT_MASK
                })
                | (if self.is_button_pressed(InputButton::Up) {
                    0
                } else {
                    SELECT_UP_MASK
                })
                | (if self.is_button_pressed(InputButton::Down) {
                    0
                } else {
                    START_DOWN_MASK
                });
        }

        if self.select_buttons {
            return BASE_MASK
                | (if self.is_button_pressed(InputButton::A) {
                    0
                } else {
                    A_RIGHT_MASK
                })
                | (if self.is_button_pressed(InputButton::B) {
                    0
                } else {
                    B_LEFT_MASK
                })
                | (if self.is_button_pressed(InputButton::Select) {
                    0
                } else {
                    SELECT_UP_MASK
                })
                | (if self.is_button_pressed(InputButton::Start) {
                    0
                } else {
                    START_DOWN_MASK
                });
        }

        // Return default state to not boot in multiplayer mode
        DEFAULT_STATE
    }

    fn is_button_pressed(&self, button: InputButton) -> bool {
        matches!(self.buttons.get(&button), Some(InputAction::Press))
    }

    pub fn apply_input(&mut self, input_events: Vec<InputEvent>) {
        for event in input_events {
            let InputEvent::EventAction(button, new_action) = event;
            println!("Button: {:?} {:?}", button, new_action);
            if let Some(action) = self.buttons.get_mut(&button) {
                *action = new_action;
            }
        }
    }

    pub fn set_input_provider(&mut self, provider: Option<Box<dyn InputProvider>>) {
        self.input_provider = provider;
    }
}
