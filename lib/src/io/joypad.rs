const A_RIGHT_POS: u8 = 0x00;
const B_LEFT_BOS: u8 = 0x01;
const SELECT_UP_POS: u8 = 0x02;
const START_DOWN_POS: u8 = 0x03;
const SELECT_DPAD_POS: u8 = 0x04;
const SELECT_BUTTONS_POS: u8 = 0x05;

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
        self.select_dpad = (value >> SELECT_DPAD_POS) & 0x01 == 0;
        self.select_buttons = (value >> SELECT_BUTTONS_POS) & 0x01 == 0;
    }

    pub fn get(&self) -> u8 {
        if self.select_dpad {
            return 0xC0
                | (if self.right { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.left { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.up { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.down { 0 } else { 1 }) << START_DOWN_POS;
        }

        if self.select_buttons {
            return 0xC0 
                | (if self.a { 0 } else { 1 }) << A_RIGHT_POS
                | (if self.b { 0 } else { 1 }) << B_LEFT_BOS
                | (if self.select { 0 } else { 1 }) << SELECT_UP_POS
                | (if self.start { 0 } else { 1 }) << START_DOWN_POS;
        }

        // Return default value to not boot in multiplayer mode
        0xCF
    }

    /*
    pub fn handle_input(&mut self, event_handler: &EventHandler) {
        self.up = event_handler.pressed_up;
        self.left = event_handler.pressed_left;
        self.down = event_handler.pressed_down;
        self.right = event_handler.pressed_right;
        self.a = event_handler.pressed_a;
        self.b = event_handler.pressed_b;
        self.select = event_handler.pressed_select;
        self.start = event_handler.pressed_start;
    }
    */
}
