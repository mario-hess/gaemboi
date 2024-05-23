/**
 * @file    main.rs
 * @brief   Initializes the emulator by loading the ROM and delegating control to the core emulation loop.
 * @author  Mario Hess
 * @date    May 23, 2024
 *
 * Dependencies:
 * - SDL2: Audio, input, and display handling.
 *      (https://docs.rs/sdl2/latest/sdl2/)
 * - rfd: File dialog
 *      (https://docs.rs/rfd/latest/rfd/)
 */
mod apu;
mod boot_sequence;
mod cartridge;
mod clock;
mod config;
mod cpu;
mod event_handler;
mod instruction;
mod interrupt;
mod joypad;
mod machine;
mod memory_bus;
mod menu;
mod ppu;
mod registers;
mod sdl;
mod timer;

use std::{
    env,
    fs::File,
    io::{Error, Read},
};

use crate::{config::Config, event_handler::EventHandler, machine::Machine, sdl::SDL};

#[derive(Debug)]
pub enum MachineState {
    Menu,
    Boot,
    Play,
}

fn main() -> Result<(), Error> {
    // Build config
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    let mut event_handler = EventHandler::new();

    let mut sdl = SDL::new(&event_handler);

    // Set file_path if passed through args
    if let Some(ref file_path) = config.file_path {
        event_handler.file_path = Some(file_path.to_string());
        event_handler.machine_state = MachineState::Boot;
    }

    while !event_handler.pressed_escape && !event_handler.quit {
        event_handler.poll(&mut sdl.event_pump);
        event_handler.check_resized(&mut sdl.window.canvas);

        match event_handler.machine_state {
            MachineState::Menu => {
                menu::run(&mut event_handler, &mut sdl.event_pump, &mut sdl.window);
            }
            MachineState::Boot => {
                boot_sequence::run(&mut sdl.window, &mut event_handler, &mut sdl.event_pump);
            }
            MachineState::Play => {
                let file_path = event_handler.file_path.clone().unwrap();
                let rom_data = read_file(event_handler.file_path.unwrap())?;

                let mut machine = Machine::new(rom_data);

                // Try to load a save file
                match read_file(file_path.replace(".gb", ".sav")) {
                    Ok(data) => machine.cpu.memory_bus.load_game(data),
                    Err(_) => println!("Couldn't load game progress."),
                }

                event_handler.file_path = None;

                // Delegate control to the core emulation loop
                machine.run(&mut sdl, &mut event_handler);

                // Try to create a save file
                machine
                    .cpu
                    .memory_bus
                    .save_game(&file_path.replace(".gb", ".sav"));

                // Back to menu
                event_handler.machine_state = MachineState::Menu;
            }
        }
    }

    Ok(())
}

fn read_file(file_path: String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}
