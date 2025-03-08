use gaemboi::InputButton;
use std::{cell::RefCell, rc::Rc};
use ui::sdl2::{EventPump, event::Event, keyboard::Keycode};

use crate::inputs::Inputs;

pub struct EventHandler {
    inputs: Rc<RefCell<Inputs>>,
    pub quit: bool,
}

impl EventHandler {
    pub fn new(inputs: Rc<RefCell<Inputs>>) -> Self {
        Self {
            inputs,
            quit: false,
        }
    }

    pub fn poll(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::D) => self.inputs.borrow_mut().set(InputButton::Right),
                    Some(Keycode::A) => self.inputs.borrow_mut().set(InputButton::Left),
                    Some(Keycode::W) => self.inputs.borrow_mut().set(InputButton::Up),
                    Some(Keycode::S) => self.inputs.borrow_mut().set(InputButton::Down),
                    Some(Keycode::N) => self.inputs.borrow_mut().set(InputButton::A),
                    Some(Keycode::M) => self.inputs.borrow_mut().set(InputButton::B),
                    Some(Keycode::Backspace) => self.inputs.borrow_mut().set(InputButton::Select),
                    Some(Keycode::Return) => self.inputs.borrow_mut().set(InputButton::Start),

                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::D) => self.inputs.borrow_mut().unset(InputButton::Right),
                    Some(Keycode::A) => self.inputs.borrow_mut().unset(InputButton::Left),
                    Some(Keycode::W) => self.inputs.borrow_mut().unset(InputButton::Up),
                    Some(Keycode::S) => self.inputs.borrow_mut().unset(InputButton::Down),
                    Some(Keycode::N) => self.inputs.borrow_mut().unset(InputButton::A),
                    Some(Keycode::M) => self.inputs.borrow_mut().unset(InputButton::B),
                    Some(Keycode::Backspace) => self.inputs.borrow_mut().unset(InputButton::Select),
                    Some(Keycode::Return) => self.inputs.borrow_mut().unset(InputButton::Start),

                    _ => {}
                },
                _ => {}
            }
        }
    }
}
