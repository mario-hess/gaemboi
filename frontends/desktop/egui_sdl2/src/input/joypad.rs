use gaemboi::{InputEvent, InputProvider, InputQueue};
use std::{cell::RefCell, rc::Rc};

pub struct Joypad {
    queue: InputQueue,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            queue: InputQueue::new(),
        }
    }

    pub fn apply(&mut self, input: InputEvent) {
        self.queue.push(input);
    }
}

impl InputProvider for Joypad {
    fn on_input(&mut self) -> Vec<InputEvent> {
        self.queue.drain()
    }
}

pub struct InputProviderWrapper(pub Rc<RefCell<Joypad>>);
impl InputProvider for InputProviderWrapper {
    fn on_input(&mut self) -> Vec<InputEvent> {
        self.0.borrow_mut().queue.drain()
    }
}
