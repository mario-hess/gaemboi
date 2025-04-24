use crate::input::joypad::Joypad;

use crate::sdl2::{EventPump, event::Event, keyboard::Keycode, video::Window};
use egui_sdl2_gl::{EguiStateHandler, painter::Painter};
use std::{cell::RefCell, rc::Rc};

use super::input_handler::InputHandler;

pub struct EventHandler {
    pub quit: bool,
    input_handler: InputHandler,
}

impl EventHandler {
    pub fn new(joypad: Rc<RefCell<Joypad>>) -> Self {
        Self {
            quit: false,
            input_handler: InputHandler::new(joypad),
        }
    }

    pub fn poll(
        &mut self,
        event_pump: &mut EventPump,
        egui_state: &mut EguiStateHandler,
        window: &mut Window,
        painter: Rc<RefCell<Painter>>,
    ) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                Event::KeyDown { keycode, .. } => self.input_handler.key_down(&keycode),
                Event::KeyUp { keycode, .. } => self.input_handler.key_up(&keycode),
                _ => egui_state.process_input(window, event, &mut painter.borrow_mut()),
            }
        }
    }
}
