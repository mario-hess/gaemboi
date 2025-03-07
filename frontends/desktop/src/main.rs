extern crate sdl2;

use std::{error::Error, io::Read};
use sdl2::{event::Event, keyboard::Keycode};

// TODO: Implement full frontend
fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let rom_path = String::from("../../roms/tests/cpu_instrs/cpu_instrs.gb");
    let rom_data = match read_file(&rom_path) {
        Ok(rom_data) => rom_data,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };

    let mut gameboy = match gaemboi::build(&rom_data) {
        Ok(gameboy) => gameboy,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };

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
