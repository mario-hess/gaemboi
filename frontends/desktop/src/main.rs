mod audio;
mod event_handler;
mod inputs;
mod screen;
mod sync_audio;
mod utils;

extern crate egui_sdl2_gl as ui;

use gaemboi::{GameBoyFactory, FRAME_DURATION};
use std::{cell::RefCell, error::Error, rc::Rc, time::Instant};
use ui::sdl2::{audio::AudioSpecDesired, pixels::Color};

use ringbuf::{traits::Split, wrap::Wrap, HeapRb};

use crate::{
    audio::{AudioConsumer, AudioProducer},
    event_handler::EventHandler,
    inputs::{InputProviderWrapper, Inputs},
    screen::Screen,
    sync_audio::SyncBridge,
};

const SAMPLING_RATE: u16 = 512;
const RING_BUFFER_MAX_SIZE: u16 = SAMPLING_RATE * 16;
const SAMPLING_FREQUENCY: u16 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = ui::sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;

    let window = video_subsystem
        .window("Gaemboi", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let rom_path = String::from("../../roms/Pokemon Yellow.gb");
    let (gb_type, rom_data) = utils::rom::extract_from_path(&rom_path)?;

    let mut gameboy = GameBoyFactory::build(&gb_type, &rom_data)?;

    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let ring_buffer = HeapRb::<u8>::new(RING_BUFFER_MAX_SIZE.into());
    let (rb_producer, rb_consumer) = ring_buffer.split();
    let ring_buffer_ref = rb_consumer.rb_ref().clone();

    let audio_producer = AudioProducer::new(rb_producer);
    let audio_consumer = AudioConsumer::new(rb_consumer);

    let audio_device = audio_subsystem.open_playback(None, &device, |_spec| audio_consumer)?;

    let screen = Screen::new();
    let inputs = Rc::new(RefCell::new(Inputs::new()));
    let mut event_handler = EventHandler::new(inputs.clone());

    gameboy.set_frame_buffer_observer(Box::new(screen));
    gameboy.set_audio_samples_observer(Box::new(audio_producer));
    gameboy.set_input_provider(Box::new(InputProviderWrapper(inputs.clone())));

    let mut sync_bridge = SyncBridge::new();

    audio_device.resume();

    while !event_handler.quit {
        let frame_start_time = Instant::now();
        event_handler.poll(&mut event_pump);

        gameboy.step_frame();

        //spin(&frame_start_time);
        sync_bridge.sync(&frame_start_time, ring_buffer_ref.clone());
    }

    Ok(())
}

fn spin(frame_start_time: &Instant) {
    while frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() {
        std::hint::spin_loop();
    }
}
