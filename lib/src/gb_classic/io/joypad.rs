#![cfg_attr(rustfmt, rustfmt::skip)]
use crate::{gb_factory::GameBoyType, InputProvider, input_buttons::InputButtons};

const A_RIGHT_MASK: u8 = 0x01;
const B_LEFT_MASK: u8 = 0x02;
const SELECT_UP_MASK: u8 = 0x04;
const START_DOWN_MASK: u8 = 0x08;
const SELECT_DPAD_MASK: u8 = 0x10;
const SELECT_BUTTONS_MASK: u8 = 0x20;

pub struct Joypad {
    input_buttons: InputButtons,
    select_dpad: bool,
    select_buttons: bool,
    pub input_provider: Option<Box<dyn InputProvider>>,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            input_buttons: InputButtons::new(GameBoyType::GameBoyClassic),
            select_dpad: false,
            select_buttons: false,
            input_provider: None,
        }
    }
}

impl Joypad {
    pub fn set(&mut self, value: u8) {
        self.select_dpad = (value & SELECT_DPAD_MASK) == 0;
        self.select_buttons = (value & SELECT_BUTTONS_MASK) == 0;
    }

    pub fn get(&self) -> u8 {
        if self.select_dpad {
            return 0xC0
                | (if self.input_buttons.get_right() { 0 } else { A_RIGHT_MASK })
                | (if self.input_buttons.get_left() { 0 } else { B_LEFT_MASK })
                | (if self.input_buttons.get_up() { 0 } else { SELECT_UP_MASK })
                | (if self.input_buttons.get_down() { 0 } else { START_DOWN_MASK });
        }

        if self.select_buttons {
            return 0xC0
                | (if self.input_buttons.get_a() { 0 } else { A_RIGHT_MASK })
                | (if self.input_buttons.get_b() { 0 } else { B_LEFT_MASK })
                | (if self.input_buttons.get_select() { 0 } else { SELECT_UP_MASK })
                | (if self.input_buttons.get_start() { 0 } else { START_DOWN_MASK });
        }

        // Return default value to not boot in multiplayer mode
        0xCF
    }

    pub fn set_inputs(&mut self, input_buttons: InputButtons) {
        self.input_buttons = input_buttons;

        //println!("{:?}", self.input_buttons);
    }
}
