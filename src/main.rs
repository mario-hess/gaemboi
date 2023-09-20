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
mod cartridge;
mod clock;
mod cpu;
mod instruction;
mod interrupt;
mod keyboard;
mod machine;
mod memory_bus;
mod ppu;
mod registers;
mod timer;
use std::env;

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use crate::machine::Machine;

const ROM_FOLDER: &str = "roms/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = parse_config(&args);

    let rom_data = match create_rom(rom_path) {
        Ok(value) => value,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found."),
            ErrorKind::InvalidData => panic!("Invalid file."),
            _ => panic!("Couldn't create ROM."),
        },
    };

    let mut machine = Machine::new(rom_data);
    machine.run();
}

fn parse_config(args: &[String]) -> String {
    if args.len() < 2 {
        panic!("Error: No file path provided.");
    }

    ROM_FOLDER.to_owned() + &args[1]
}

fn create_rom(rom_path: String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(rom_path)?;
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)?;

    Ok(rom_data)
}
