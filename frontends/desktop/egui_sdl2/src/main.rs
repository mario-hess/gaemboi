mod audio;
mod events;
mod input;
mod screen;
mod utils;

use crate::{
    audio::{audio_consumer::AudioConsumer, audio_producer::AudioProducer, audio_sync::AudioSync},
    events::event_handler::EventHandler,
    input::joypad::{InputProviderWrapper, Joypad},
    screen::screen_adapter::{FrameBufferObserverWrapper, ScreenAdapter},
    utils::fps_counter::FpsCounter,
};
use gaemboi::{
    ADVANCE_SCREEN_HEIGHT, ADVANCE_SCREEN_WIDTH, CLASSIC_SCREEN_HEIGHT, CLASSIC_SCREEN_WIDTH,
    FRAME_DURATION, GameBoyFactory, GameBoyType,
};

use egui_sdl2_gl::{
    DpiScaling, ShaderVersion,
    egui::{self, CentralPanel, FontFamily, FontId, FullOutput, TextStyle, TopBottomPanel, menu},
    gl, sdl2, with_sdl2,
};
use sdl2::{
    audio::AudioSpecDesired,
    video::{GLProfile, SwapInterval},
};
use std::{cell::RefCell, error::Error, rc::Rc, time::Instant};

use ringbuf::{SharedRb, storage::Heap, traits::Split, wrap::Wrap};

const MENU_HEIGHT: u32 = 26;
const WINDOW_SCALE: u32 = 4;
const SAMPLING_RATE: u16 = 512;
const RING_BUFFER_MAX_SIZE: u16 = SAMPLING_RATE * 12;
const SAMPLING_FREQUENCY: u16 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = egui_sdl2_gl::sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;

    let rom_path = String::from("../../../roms/Pokemon Yellow.gb");
    let (gb_type, rom_data) = utils::rom::extract_from_path(&rom_path)?;
    let mut gameboy = GameBoyFactory::build(&gb_type, &rom_data)?;

    let (min_width, min_height) = match gb_type {
        GameBoyType::Classic | GameBoyType::Color => {
            (CLASSIC_SCREEN_WIDTH as u32, CLASSIC_SCREEN_HEIGHT as u32)
        }
        GameBoyType::Advance => (ADVANCE_SCREEN_WIDTH as u32, ADVANCE_SCREEN_HEIGHT as u32),
    };

    // Video
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);

    let mut window = video_subsystem
        .window(
            "Gaemboi",
            min_width * WINDOW_SCALE,
            min_height * WINDOW_SCALE + MENU_HEIGHT,
        )
        .opengl()
        .resizable()
        .position_centered()
        .build()?;

    window.set_minimum_size(min_width, min_height + MENU_HEIGHT)?;
    let _ctx = window.gl_create_context()?;

    // Init egui
    let (painter, mut egui_state) = with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let painter = Rc::new(RefCell::new(painter));
    let egui_ctx = egui::Context::default();

    let mut style = (*egui_ctx.style()).clone();
    let font_id = FontId::new(16.0, FontFamily::Proportional);
    style.text_styles = [
        (TextStyle::Small, font_id.clone()),
        (TextStyle::Body, font_id.clone()),
        (TextStyle::Button, font_id.clone()),
        (TextStyle::Heading, font_id.clone()),
        (TextStyle::Monospace, font_id.clone()),
    ]
    .into();
    egui_ctx.set_style(style);

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)?;

    let screen_adapter = Rc::new(RefCell::new(ScreenAdapter::new(painter.clone(), &gb_type)));
    gameboy.set_frame_buffer_observer(Box::new(FrameBufferObserverWrapper::new(
        screen_adapter.clone(),
    )));

    // Audio
    let audio_subsystem = sdl_context.audio()?;
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
    gameboy.set_audio_samples_observer(Box::new(audio_producer));

    // Inputs
    let joypad = Rc::new(RefCell::new(Joypad::new()));
    gameboy.set_input_provider(Box::new(InputProviderWrapper(joypad.clone())));

    let mut event_handler = EventHandler::new(joypad.clone());
    let mut fps_counter = FpsCounter::new();
    let mut audio_sync = AudioSync::new();

    let start_time = Instant::now();

    while !event_handler.quit {
        let frame_start_time = Instant::now();
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_pass(egui_state.input.take());
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        event_handler.poll(
            &mut event_pump,
            &mut egui_state,
            &mut window,
            painter.clone(),
        );

        TopBottomPanel::top("top_panel")
            .exact_height(MENU_HEIGHT as f32)
            .show(&egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open").clicked() {
                                ui.close_menu();
                            }
                        });
                    })
                })
            });

        CentralPanel::default()
            .frame(egui_sdl2_gl::egui::Frame::default())
            .show(&egui_ctx, |ui| {
                let available_size = ui.available_size();

                ui.centered_and_justified(|ui| {
                    ui.add(
                        screen_adapter
                            .borrow()
                            .get_image()
                            .maintain_aspect_ratio(true)
                            .fit_to_exact_size(available_size),
                    );
                });
            });

        gameboy.step_frame();

        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_pass();
        egui_state.process_output(&window, &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes, pixels_per_point);
        painter
            .borrow_mut()
            .paint_jobs(None, textures_delta, paint_jobs);

        window.gl_swap_window();

        //audio_sync.sync(&frame_start_time, rb_ref.clone());
        fps_counter.show(&frame_start_time);
    }

    Ok(())
}
