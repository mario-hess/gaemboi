extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode};
use std::{error::Error, io::Read};

use gaemboi::{AudioSamplesObserver, FrameBufferObserver, GameBoyFactory, GameBoyType};

struct Screen;
impl Screen {
    fn new() -> Self {
        Self {}
    }
    fn update(&mut self, frame_buffer: &[u8]) {
        // println!("{:?}", frame_buffer);
    }
    fn render(&self) {}
}

impl FrameBufferObserver for Screen {
    fn on_frame_ready(&mut self, frame_buffer: &[u8]) {
        self.update(frame_buffer);
        self.render();
    }
}

struct AudioPlayback;
impl AudioPlayback {
    fn new() -> Self {
        Self {}
    }

    fn queue_samples(&mut self, audio_samples: &(u8, u8)) {
        let (left, right) = audio_samples;
        // println!("Left: {} | Right: {}", left, right);
    }
}

// Implement only the audio observer for the audio component
impl AudioSamplesObserver for AudioPlayback {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8)) {
        self.queue_samples(audio_samples);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let rom_path = String::from("../../roms/Pokemon Yellow.gb");
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
    let audio_playback = AudioPlayback::new();

    gameboy.set_frame_buffer_observer(Box::new(screen));
    gameboy.set_audio_samples_observer(Box::new(audio_playback));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
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
