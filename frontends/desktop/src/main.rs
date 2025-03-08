extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};
use std::cell::RefCell;
use std::rc::Rc;
use std::{error::Error, io::Read};

use gaemboi::{
    AudioSamplesObserver, FrameBufferObserver, GameBoyFactory, GameBoyType, InputButton,
    InputButtons, InputProvider,
};

struct Screen;
impl Screen {
    fn new() -> Self {
        Self {}
    }
    fn update(&mut self, frame_buffer: &[u8]) {
        //println!("{:?}", frame_buffer);
    }
    fn render(&self) {}
}
impl FrameBufferObserver for Screen {
    fn on_frame_ready(&mut self, frame_buffer: &[u8]) {
        self.update(frame_buffer);
        self.render();
    }
}

struct Audio;
impl Audio {
    fn new() -> Self {
        Self {}
    }

    fn queue_samples(&mut self, audio_samples: &(u8, u8)) {
        let (left, right) = audio_samples;
        //println!("Left: {} | Right: {}", left, right);
    }
}
impl AudioSamplesObserver for Audio {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8)) {
        self.queue_samples(audio_samples);
    }
}

struct Inputs {
    input_buttons: InputButtons,
}
impl Inputs {
    fn new() -> Self {
        Self {
            input_buttons: InputButtons::new(GameBoyType::GameBoyClassic),
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

pub struct InputProviderWrapper(Rc<RefCell<Inputs>>);
impl InputProvider for InputProviderWrapper {
    fn get_inputs(&self) -> InputButtons {
        self.0.borrow().get_inputs()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let rom_path = String::from("../../roms/tests/cpu_instrs/cpu_instrs.gb");
    let rom_data = match read_file(&rom_path) {
        Ok(rom_data) => rom_data,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };

    let mut gameboy = match GameBoyFactory::build(GameBoyType::GameBoyClassic, &rom_data) {
        Ok(gameboy) => gameboy,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };

    let screen = Screen::new();
    let audio_playback = Audio::new();
    let inputs = Rc::new(RefCell::new(Inputs::new()));

    gameboy.set_frame_buffer_observer(Box::new(screen));
    gameboy.set_audio_samples_observer(Box::new(audio_playback));
    gameboy.set_input_provider(Box::new(InputProviderWrapper(inputs.clone())));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::D) => inputs.borrow_mut().set(InputButton::Right),
                    Some(Keycode::A) => inputs.borrow_mut().set(InputButton::Left),
                    Some(Keycode::W) => inputs.borrow_mut().set(InputButton::Up),
                    Some(Keycode::S) => inputs.borrow_mut().set(InputButton::Down),
                    Some(Keycode::N) => inputs.borrow_mut().set(InputButton::A),
                    Some(Keycode::M) => inputs.borrow_mut().set(InputButton::B),
                    Some(Keycode::Backspace) => inputs.borrow_mut().set(InputButton::Select),
                    Some(Keycode::Return) => inputs.borrow_mut().set(InputButton::Start),

                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::D) => inputs.borrow_mut().unset(InputButton::Right),
                    Some(Keycode::A) => inputs.borrow_mut().unset(InputButton::Left),
                    Some(Keycode::W) => inputs.borrow_mut().unset(InputButton::Up),
                    Some(Keycode::S) => inputs.borrow_mut().unset(InputButton::Down),
                    Some(Keycode::N) => inputs.borrow_mut().unset(InputButton::A),
                    Some(Keycode::M) => inputs.borrow_mut().unset(InputButton::B),
                    Some(Keycode::Backspace) => inputs.borrow_mut().unset(InputButton::Select),
                    Some(Keycode::Return) => inputs.borrow_mut().unset(InputButton::Start),

                    _ => {}
                },
                _ => {}
            }
        }

        gameboy.step_frame();
    }

    Ok(())
}

fn read_file(file_path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}
