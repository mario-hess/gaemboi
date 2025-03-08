mod audio;
mod event_handler;
mod inputs;
mod screen;
mod utils;

extern crate egui_sdl2_gl as ui;

use gaemboi::GameBoyFactory;
use std::{cell::RefCell, error::Error, rc::Rc};
use ui::sdl2::pixels::Color;

use crate::{
    audio::Audio,
    event_handler::EventHandler,
    inputs::{InputProviderWrapper, Inputs},
    screen::Screen,
};

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = ui::sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gaemboi", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let rom_path = String::from("../../roms/tests/cpu_instrs/cpu_instrs.gb");
    let (gb_type, rom_data) = utils::rom::extract_from_path(&rom_path)?;

    let mut gameboy = GameBoyFactory::build(&gb_type, &rom_data)?;

    let screen = Screen::new();
    let audio_playback = Audio::new();
    let inputs = Rc::new(RefCell::new(Inputs::new()));
    let mut event_handler = EventHandler::new(inputs.clone());

    gameboy.set_frame_buffer_observer(Box::new(screen));
    gameboy.set_audio_samples_observer(Box::new(audio_playback));
    gameboy.set_input_provider(Box::new(InputProviderWrapper(inputs.clone())));

    while !event_handler.quit {
        event_handler.poll(&mut event_pump);
        gameboy.step_frame();
    }

    Ok(())
}
