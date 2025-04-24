use egui_sdl2_gl::sdl2::keyboard::Keycode;
use gaemboi::{InputAction, InputButton, InputEvent};
use std::{cell::RefCell, rc::Rc};

use crate::Joypad;

pub struct InputHandler {
    joypad: Rc<RefCell<Joypad>>,
}

impl InputHandler {
    pub fn new(joypad: Rc<RefCell<Joypad>>) -> Self {
        Self { joypad }
    }

    pub fn key_down(&self, keycode: &Option<Keycode>) {
        let mut joypad = self.joypad.borrow_mut();

        match keycode {
            Some(Keycode::D) => joypad.apply(InputEvent::EventAction(
                InputButton::Right,
                InputAction::Press,
            )),
            Some(Keycode::A) => joypad.apply(InputEvent::EventAction(
                InputButton::Left,
                InputAction::Press,
            )),
            Some(Keycode::W) => {
                joypad.apply(InputEvent::EventAction(InputButton::Up, InputAction::Press))
            }
            Some(Keycode::S) => joypad.apply(InputEvent::EventAction(
                InputButton::Down,
                InputAction::Press,
            )),
            Some(Keycode::N) => {
                joypad.apply(InputEvent::EventAction(InputButton::A, InputAction::Press))
            }
            Some(Keycode::M) => {
                joypad.apply(InputEvent::EventAction(InputButton::B, InputAction::Press))
            }
            Some(Keycode::Backspace) => joypad.apply(InputEvent::EventAction(
                InputButton::Select,
                InputAction::Press,
            )),
            Some(Keycode::Return) => joypad.apply(InputEvent::EventAction(
                InputButton::Start,
                InputAction::Press,
            )),
            _ => {}
        }
    }

    pub fn key_up(&self, keycode: &Option<Keycode>) {
        let mut joypad = self.joypad.borrow_mut();

        match keycode {
            Some(Keycode::D) => joypad.apply(InputEvent::EventAction(
                InputButton::Right,
                InputAction::Release,
            )),
            Some(Keycode::A) => joypad.apply(InputEvent::EventAction(
                InputButton::Left,
                InputAction::Release,
            )),
            Some(Keycode::W) => joypad.apply(InputEvent::EventAction(
                InputButton::Up,
                InputAction::Release,
            )),
            Some(Keycode::S) => joypad.apply(InputEvent::EventAction(
                InputButton::Down,
                InputAction::Release,
            )),
            Some(Keycode::N) => joypad.apply(InputEvent::EventAction(
                InputButton::A,
                InputAction::Release,
            )),
            Some(Keycode::M) => joypad.apply(InputEvent::EventAction(
                InputButton::B,
                InputAction::Release,
            )),
            Some(Keycode::Backspace) => joypad.apply(InputEvent::EventAction(
                InputButton::Select,
                InputAction::Release,
            )),
            Some(Keycode::Return) => joypad.apply(InputEvent::EventAction(
                InputButton::Start,
                InputAction::Release,
            )),
            _ => {}
        }
    }
}
