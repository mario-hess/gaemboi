#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InputButton {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
    L,
    R,
}

#[derive(PartialEq, Debug)]
pub enum InputAction {
    Press,
    Release,
}

pub enum InputEvent {
    EventAction(InputButton, InputAction),
}

pub struct InputQueue {
    events: Vec<InputEvent>,
}

impl InputQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::default(),
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<InputEvent> {
        std::mem::take(&mut self.events)
    }
}
