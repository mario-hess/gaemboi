use gaemboi::{InputButton, InputButtons, InputProvider};
use std::{cell::RefCell, rc::Rc};

pub struct Inputs {
    input_buttons: InputButtons,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            input_buttons: InputButtons::new(),
        }
    }

    pub fn set(&mut self, input_button: InputButton) {
        match input_button {
            InputButton::Right => self.input_buttons.set_right(true),
            InputButton::Left => self.input_buttons.set_left(true),
            InputButton::Up => self.input_buttons.set_up(true),
            InputButton::Down => self.input_buttons.set_down(true),
            InputButton::A => self.input_buttons.set_a(true),
            InputButton::B => self.input_buttons.set_b(true),
            InputButton::Select => self.input_buttons.set_select(true),
            InputButton::Start => self.input_buttons.set_start(true),
            InputButton::L | InputButton::R => {}
        }
    }

    pub fn unset(&mut self, input_button: InputButton) {
        match input_button {
            InputButton::Right => self.input_buttons.set_right(false),
            InputButton::Left => self.input_buttons.set_left(false),
            InputButton::Up => self.input_buttons.set_up(false),
            InputButton::Down => self.input_buttons.set_down(false),
            InputButton::A => self.input_buttons.set_a(false),
            InputButton::B => self.input_buttons.set_b(false),
            InputButton::Select => self.input_buttons.set_select(false),
            InputButton::Start => self.input_buttons.set_start(false),
            InputButton::L | InputButton::R => {}
        }
    }
}

impl InputProvider for Inputs {
    fn get_inputs(&self) -> InputButtons {
        self.input_buttons
    }
}

pub struct InputProviderWrapper(pub Rc<RefCell<Inputs>>);
impl InputProvider for InputProviderWrapper {
    fn get_inputs(&self) -> InputButtons {
        self.0.borrow().get_inputs()
    }
}
