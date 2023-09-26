/**
 * @file    main.rs
 * @brief   Initializes the emulator by loading the ROM and delegating control to the core emulation loop.
 * @author  Mario Hess
 * @date    September 20, 2023
 *
 * Program Dependencies:
 * - SDL2: Required for audio, input, and display handling.
 *      (https://docs.rs/sdl2/latest/sdl2/)
 */
use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use sdl2::keyboard::Keycode;

mod cartridge;
mod clock;
mod config;
mod cpu;
mod event_handler;
mod instruction;
mod interrupt;
mod machine;
mod memory_bus;
mod ppu;
mod registers;
mod timer;
mod windows;

use crate::config::Config;
use crate::event_handler::EventHandler;
use crate::machine::Machine;
use crate::machine::FPS;
use crate::windows::Windows;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    println!("{:?}", args);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut windows = Windows::build(&config, &video_subsystem);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut event_handler = EventHandler::new();

    if let Some(ref config) = config {
        if let Some(ref file_path) = config.file_path {
            event_handler.event_file = Some(file_path.to_string());
        }
    }

    let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);

    while event_handler.event_key != Some(Keycode::Escape) {
        let frame_start_time = std::time::Instant::now();

        event_handler.poll(&mut event_pump);
        Windows::clear(&mut windows);

        if let Some(file_path) = event_handler.event_file {
            event_handler.event_file = None;

            let rom_data = match read_file(file_path.to_owned()) {
                Ok(value) => value,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => panic!("File not found."),
                    ErrorKind::InvalidData => panic!("Invalid file."),
                    _ => panic!("Couldn't read ROM from provided file."),
                },
            };

            let mut machine = Machine::new(rom_data);
            machine.run(&mut event_pump, &mut event_handler, &mut windows);
        }

        Windows::present(&mut windows);

        let elapsed_time = frame_start_time.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }
}

fn read_file(rom_path: String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(rom_path)?;
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)?;

    Ok(rom_data)
}
