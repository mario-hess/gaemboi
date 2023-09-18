use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Keyboard {
    pub key: Option<u8>,
    pub escape_pressed: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key: None,
            escape_pressed: false,
        }
    }

    pub fn set_key(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.escape_pressed = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.key = Some(1),
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.key = None,
                _ => {}
            };
        }
    }
}
