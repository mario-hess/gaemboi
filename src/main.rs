/*
 * @file    main.rs
 * @brief   Initializes the emulator by loading the ROM and delegating control to the core emulation loop.
 * @author  Mario Hess
 * @date    May 23, 2024
 *
 * Dependencies:
 * - egui_sdl2_gl: UI, Rendering, Audio and Input handling.
 *      (https://docs.rs/egui_sdl2_gl/latest/egui_sdl2_gl/)
 * - rfd: File dialog
 *      (https://docs.rs/rfd/latest/rfd/)
 * - image: Image decoding
 *      (https://docs.rs/image/latest/image/)
 */

mod apu;
mod cartridge;
mod config;
mod cpu;
mod event_handler;
mod interrupt;
mod io;
mod memory_bus;
mod ppu;
mod ui;

use apu::audio::{Audio, SAMPLING_FREQUENCY, SAMPLING_RATE};
use apu::AUDIO_BUFFER_THRESHOLD;
use cpu::clock::{Clock, CYCLES_PER_FRAME};
use egui_sdl2_gl::egui::{Context, FontFamily, FontId, TextStyle};
use memory_bus::ComponentTick;

use egui_sdl2_gl::sdl2::audio::{AudioDevice, AudioSpecDesired};
use egui_sdl2_gl::sdl2::video::GLProfile;
use egui_sdl2_gl::sdl2::AudioSubsystem;
use egui_sdl2_gl::{DpiScaling, ShaderVersion};
use ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use std::collections::VecDeque;
use std::io::{Error, Read};
use std::sync::{Arc, Mutex};
use ui::UIManager;

const FRAME_DURATION_MS: f64 = 16.7433;
const FRAME_DURATION_MICROS: u64 = (FRAME_DURATION_MS * 1_000.0) as u64;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(FRAME_DURATION_MICROS);
pub const FPS: f32 = 59.7275;

#[derive(PartialEq, Clone, Copy)]
pub enum View {
    Viewport,
    Tiletable,
    Tilemap0,
    Tilemap1,
}

pub enum State {
    Splash,
    Play,
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let config = config::Config::build(&args);

    let sdl_context = egui_sdl2_gl::sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);
    let audio_subsystem = sdl_context.audio().unwrap();
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

    let mut event_handler = event_handler::EventHandler::new();

    let mut window = video_subsystem
        .window(
            "gaemboi",
            VIEWPORT_WIDTH as u32 * event_handler.window_scale,
            VIEWPORT_HEIGHT as u32 * event_handler.window_scale + 20.0 as u32,
        )
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    let (mut painter, mut egui_state) =
        egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = Context::default();
    let mut event_pump: egui_sdl2_gl::sdl2::EventPump = sdl_context.event_pump().unwrap();

    let mut style = (*egui_ctx.style()).clone();
    let font_id = FontId::new(14.0, FontFamily::Proportional);

    style.text_styles = [
        (TextStyle::Small, font_id.clone()),
        (TextStyle::Body, font_id.clone()),
        (TextStyle::Button, font_id.clone()),
        (TextStyle::Heading, font_id.clone()),
        (TextStyle::Monospace, font_id.clone()),
    ]
    .into();

    egui_ctx.set_style(style);

    unsafe {
        egui_sdl2_gl::sdl2::sys::SDL_GL_SetSwapInterval(0);
    }

    if let Some(path) = config.file_path {
        event_handler.file_path = Some(path);
        event_handler.state = State::Play;
    }

    let mut frame_times = Vec::new();
    let mut frame_count = 0;
    let mut last_second = std::time::Instant::now();
    let mut fps = 0.0;

    let mut ui_manager = UIManager::new(&mut painter);
    let start_time = std::time::Instant::now();

    while !event_handler.quit {
        match event_handler.state {
            State::Splash => {
                while !event_handler.quit {
                    let frame_start_time = std::time::Instant::now();
                    let time = start_time.elapsed().as_secs_f64();
                    egui_state.input.time = Some(time);
                    egui_ctx.begin_frame(egui_state.input.take());

                    match ui_manager.current_view {
                        View::Viewport => {}
                        _ => {
                            ui_manager.update_window_size(&mut window, &mut event_handler);
                            ui_manager.current_view = View::Viewport;
                        }
                    }

                    event_handler.poll(&mut event_pump, &mut egui_state, &window, &mut painter);

                    unsafe {
                        egui_sdl2_gl::gl::Clear(egui_sdl2_gl::gl::COLOR_BUFFER_BIT);
                    }

                    ui_manager.draw_splash(
                        &egui_ctx,
                        &mut egui_state,
                        &mut painter,
                        &mut window,
                        &mut event_handler,
                    );

                    if event_handler.file_path.is_some() {
                        event_handler.state = State::Play;
                        break;
                    }
                    if frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() {
                        std::thread::sleep(std::time::Duration::from_micros(
                            FRAME_DURATION_MICROS - frame_start_time.elapsed().as_micros() as u64,
                        ));
                    }
                }
            }
            State::Play => {
                let file_path = event_handler.file_path.clone().unwrap();
                event_handler.file_path = None;

                let rom_data = read_file(&file_path)?;
                let mut cpu = cpu::Cpu::new(rom_data);

                match read_file(&file_path.replace(".gb", ".sav")) {
                    Ok(data) => cpu.memory_bus.load_game(data),
                    Err(_) => println!("Couldn't load game progress."),
                }

                let audio_device = create_audio_device(
                    &audio_subsystem,
                    &cpu.memory_bus.apu.left_volume,
                    &cpu.memory_bus.apu.right_volume,
                    &event_handler.volume,
                    &mut cpu.memory_bus.apu.audio_buffer,
                );
                audio_device.resume();

                let mut clock = Clock::new();

                // ---------------- LOOP ------------------------
                while !event_handler.quit {
                    let frame_start_time = std::time::Instant::now();
                    let time = start_time.elapsed().as_secs_f64();
                    egui_state.input.time = Some(time);
                    egui_ctx.begin_frame(egui_state.input.take());

                    event_handler.poll(&mut event_pump, &mut egui_state, &window, &mut painter);
                    cpu.memory_bus.joypad.handle_input(&event_handler);

                    while clock.cycles_passed <= CYCLES_PER_FRAME {
                        let m_cycles = cpu.step();
                        cpu.memory_bus.tick(m_cycles);

                        if cpu.memory_bus.ppu.should_draw {
                            ui_manager.draw(
                                &egui_ctx,
                                &mut egui_state,
                                &mut painter,
                                &mut window,
                                &mut event_handler,
                                &mut cpu,
                                &fps,
                            );

                            cpu.memory_bus.ppu.should_draw = false;
                            cpu.memory_bus.ppu.clear_screen();
                        }

                        clock.tick(m_cycles);
                    }

                    clock.reset();

                    if event_handler.performance_mode {
                        if should_delay(
                            frame_start_time,
                            &cpu.memory_bus.apu.audio_buffer,
                            &event_handler.fast_forward,
                        ) {
                            std::thread::sleep(std::time::Duration::from_micros(
                                FRAME_DURATION_MICROS / event_handler.fast_forward as u64
                                    - frame_start_time.elapsed().as_micros() as u64,
                            ));
                        }
                    } else {
                        while should_delay(
                            frame_start_time,
                            &cpu.memory_bus.apu.audio_buffer,
                            &event_handler.fast_forward,
                        ) {
                            std::hint::spin_loop();
                        }
                    }

                    let frame_time = frame_start_time.elapsed().as_secs_f32();
                    frame_times.push(frame_time);
                    frame_count += 1;

                    if last_second.elapsed().as_secs() >= 1 {
                        fps = frame_count as f32 / frame_times.iter().sum::<f32>();
                        frame_times.clear();
                        frame_count = 0;
                        last_second = std::time::Instant::now();
                    }

                    if event_handler.file_path.is_some() {
                        break;
                    }
                }

                cpu.memory_bus.save_game(&file_path.replace(".gb", ".sav"));

                event_handler.state = State::Splash;
                event_handler.quit = false;
            }
        }
    }

    Ok(())
}

fn read_file(file_path: &String) -> Result<Vec<u8>, Error> {
    let mut file = std::fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

fn create_audio_device<'a>(
    audio_subsystem: &AudioSubsystem,
    left_volume: &'a u8,
    right_volume: &'a u8,
    volume: &'a u8,
    audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>,
) -> AudioDevice<Audio<'a>> {
    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let audio = Audio::new(audio_buffer, left_volume, right_volume, volume);

    audio_subsystem
        .open_playback(None, &device, |_spec| audio)
        .unwrap()
}

fn should_delay(
    frame_start_time: std::time::Instant,
    audio_buffer: &Arc<Mutex<VecDeque<u8>>>,
    fast_forward: &u8,
) -> bool {
    frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() / *fast_forward as u128
        && audio_buffer.lock().unwrap().len() > AUDIO_BUFFER_THRESHOLD
}
