/**
 * @file    main.rs
 * @brief   Initializes the emulator by loading the ROM and delegating control to the core emulation loop.
 * @author  Mario Hess
 * @date    October 04, 2023
 *
 * Program Dependencies:
 * - SDL2: Required for audio, input, and display handling.
 *      (https://docs.rs/sdl2/latest/sdl2/)
 */
mod boot_sequence;
mod cartridge;
mod clock;
mod config;
mod cpu;
mod debug_windows;
mod event_handler;
mod instruction;
mod interrupt;
mod machine;
mod memory_bus;
mod ppu;
mod registers;
mod timer;
mod window;

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::ttf::init;

use crate::config::Config;
use crate::event_handler::EventHandler;
use crate::machine::Machine;
use crate::ppu::{SCALE, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE};
use crate::window::Window;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::build(&args);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = init().map_err(|e| e.to_string()).unwrap();

    let mut viewport = Window::build(
        &video_subsystem,
        &ttf_context,
        "Viewport",
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        SCALE,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut event_handler = EventHandler::new();

    if let Some(ref file_path) = config.file_path {
        event_handler.file_dropped = Some(file_path.to_string());
    }

    while event_handler.key_pressed != Some(Keycode::Escape) {
        event_handler.poll(&mut event_pump);

        viewport.canvas.set_draw_color(WHITE);
        viewport.canvas.clear();

        if config.boot_sequence_enabled {
            boot_sequence::run(
                &mut viewport,
                &mut event_handler,
                &mut event_pump,
                &mut config,
            );
        }

        if let Some(file_path) = event_handler.file_dropped {
            event_handler.file_dropped = None;

            let rom_data = match read_file(file_path.to_owned()) {
                Ok(value) => value,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => panic!("File not found."),
                    ErrorKind::InvalidData => panic!("Invalid file."),
                    _ => panic!("Couldn't read ROM from provided file."),
                },
            };

            let mut machine = Machine::new(rom_data);
            machine.run(
                &mut config,
                &mut event_pump,
                &mut event_handler,
                &video_subsystem,
                &ttf_context,
                &mut viewport,
            );
        }

        viewport.canvas.present();
    }
}

fn read_file(rom_path: String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(rom_path)?;
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)?;

    Ok(rom_data)
}
