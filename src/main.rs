/**
 * @file    main.rs
 * @brief   Initializes the emulator by loading the ROM and delegating control to the core emulation loop.
 * @author  Mario Hess
 * @date    November 11, 2023
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
mod debug_windows;
mod event_handler;
mod instruction;
mod interrupt;
mod joypad;
mod machine;
mod memory_bus;
mod menu;
mod ppu;
mod registers;
mod timer;
mod window;

use std::{
    env,
    fs::File,
    io::{Error, Read},
};

use sdl2::ttf::init;

use crate::{
    config::Config,
    event_handler::EventHandler,
    machine::Machine,
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    window::Window,
};

#[derive(Debug)]
pub enum MachineState {
    Menu,
    Boot,
    Play,
}

fn main() -> Result<(), Error> {
    // Build config
    let args: Vec<String> = env::args().collect();
    let mut config = Config::build(&args);

    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();

    // Initialize gamepad
    let available = controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))
        .unwrap();

    let _gamepad = (0..available).find_map(|id| {
        if !controller_subsystem.is_game_controller(id) {
            println!("{} is not a gamepad", id);
            return None;
        }

        println!("Attempting to open gamepad {}", id);

        match controller_subsystem.open(id) {
            Ok(gamepad) => {
                println!("Success: opened \"{}\"", gamepad.name());
                Some(gamepad)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    });

    let ttf_context = init().map_err(|e| e.to_string()).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut event_handler = EventHandler::new();

    let mut viewport = Window::build(
        &video_subsystem,
        &ttf_context,
        "Viewport",
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        event_handler.window_scale as usize,
    );

    // Set file_path if passed through args
    if let Some(ref file_path) = config.file_path {
        event_handler.file_path = Some(file_path.to_string());
        event_handler.machine_state = MachineState::Boot;
    }

    while !event_handler.pressed_escape && !event_handler.quit {
        event_handler.poll(&mut event_pump);
        event_handler.check_resized(&mut viewport.canvas);

        match event_handler.machine_state {
            MachineState::Menu => {
                menu::run(&mut event_handler, &mut event_pump, &mut viewport);
            }
            MachineState::Boot => {
                boot_sequence::run(&mut viewport, &mut event_handler, &mut event_pump);
            }
            MachineState::Play => {
                let file_path = event_handler.file_path.clone().unwrap();
                let rom_data = read_file(event_handler.file_path.unwrap())?;

                let mut machine = Machine::new(rom_data);

                match read_file(file_path.replace(".gb", ".sav")) {
                    Ok(data) => machine.cpu.memory_bus.load_game(data),
                    Err(_) => println!("Couldn't load game progress."),
                }

                event_handler.file_path = None;

                machine.run(
                    &mut config,
                    &mut event_pump,
                    &mut event_handler,
                    &video_subsystem,
                    &ttf_context,
                    &mut viewport,
                );

                machine
                    .cpu
                    .memory_bus
                    .save_game(&file_path.replace(".gb", ".sav"));

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
