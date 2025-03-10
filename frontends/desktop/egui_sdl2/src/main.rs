mod audio;
mod input;
mod screen;
mod utils;

use crate::{
    audio::{audio_consumer::AudioConsumer, audio_producer::AudioProducer, audio_sync::AudioSync},
    input::{
        input_handler::InputHandler,
        joypad::{InputProviderWrapper, Joypad},
    },
    screen::screen_adapter::ScreenAdapter,
    utils::fps_counter::FpsCounter,
};
use gaemboi::{FRAME_DURATION, GameBoyFactory};

use egui_sdl2_gl::sdl2;
use sdl2::{audio::AudioSpecDesired, pixels::Color};
use std::{cell::RefCell, error::Error, rc::Rc, time::Instant};

use ringbuf::{SharedRb, storage::Heap, traits::Split, wrap::Wrap};

const SAMPLING_RATE: u16 = 512;
const RING_BUFFER_MAX_SIZE: u16 = SAMPLING_RATE * 12;
const SAMPLING_FREQUENCY: u16 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
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

    let rom_path = String::from("../../../roms/Pokemon Yellow.gb");
    let (gb_type, rom_data) = utils::rom::extract_from_path(&rom_path)?;

    let mut gameboy = GameBoyFactory::build(&gb_type, &rom_data)?;

    // Audio
    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };
    let ring_buffer = SharedRb::<Heap<u8>>::new(RING_BUFFER_MAX_SIZE.into());
    let (rb_producer, rb_consumer) = ring_buffer.split();
    let rb_ref = rb_producer.rb_ref().clone();
    let audio_consumer = AudioConsumer::new(rb_consumer);
    let audio_device = audio_subsystem.open_playback(None, &device, |_spec| audio_consumer)?;
    let audio_producer = AudioProducer::new(rb_producer, audio_device);
    gameboy.set_audio_samples_listener(Box::new(audio_producer));

    // Screen Adapter
    let screen_adapter = ScreenAdapter::new();
    gameboy.set_frame_buffer_listener(Box::new(screen_adapter));    

    // Inputs
    let joypad = Rc::new(RefCell::new(Joypad::new()));
    gameboy.set_input_provider(Box::new(InputProviderWrapper(joypad.clone())));
    let mut input_handler = InputHandler::new(joypad.clone());

    let mut fps_counter = FpsCounter::new();
    let mut audio_sync = AudioSync::new();

    canvas.present();
    while !input_handler.quit {
        let frame_start_time = Instant::now();
        canvas.clear();
        input_handler.poll(&mut event_pump);

        gameboy.step_frame();
        canvas.present();

        audio_sync.sync(&frame_start_time, rb_ref.clone());
        fps_counter.show(&frame_start_time);
    }

    Ok(())
}
