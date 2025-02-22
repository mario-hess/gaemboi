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
mod emulation;
mod event_handler;
mod interrupt;
mod io;
mod memory_bus;
mod ppu;
mod sync_bridge;
mod ui;

use egui_sdl2_gl::{
    egui::{Context, FontFamily, FontId, TextStyle},
    sdl2::{controller::GameController, video::GLProfile, GameControllerSubsystem},
    DpiScaling, ShaderVersion,
};

use {
    apu::ogg_player::create_audio_theme,
    emulation::{ComponentTick, Emulation, MemoryAccess},
    ppu::{colors::Colors, VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    ui::UIManager,
};

use std::{cell::RefCell, error::Error, io::Read, rc::Rc};

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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = config::Config::build(&args);

    // Initialize SDL2
    let sdl_context = egui_sdl2_gl::sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);
    let audio_subsystem = sdl_context.audio()?;
    let controller_subsystem = sdl_context.game_controller()?;
    let _gamepad = initialize_gamepad(controller_subsystem);

    // Initialze EventSystem
    let mut event_handler = event_handler::EventHandler::new();
    let mut event_pump: egui_sdl2_gl::sdl2::EventPump = sdl_context.event_pump()?;

    // Global colors
    let colors = Rc::new(RefCell::new(Colors::new()));
    let frame_paths = generate_frame_paths(2, 14);

    // Build window
    let mut window = video_subsystem
        .window(
            "gaemboi",
            VIEWPORT_WIDTH as u32 * event_handler.window_scale,
            VIEWPORT_HEIGHT as u32 * event_handler.window_scale + 20.0 as u32,
        )
        .opengl()
        .build()?;

    // Setup egui config
    let _ctx = window.gl_create_context()?;
    let (mut painter, mut egui_state) =
        egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = Context::default();
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

    // Check if path is passed through environment variable
    if let Some(path) = config.file_path {
        event_handler.file_path = Some(path);
        event_handler.state = State::Play;
    }

    // Setup UI
    let mut ui_manager = UIManager::new(&mut painter, colors.clone(), &frame_paths);

    let start_time = std::time::Instant::now();

    // State Logic
    while !event_handler.quit {
        match event_handler.state {
            State::Splash => {
                let audio_device = create_audio_theme(&audio_subsystem, &event_handler.volume, "./audio/splash.ogg")?;
                audio_device.resume();

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

                let rom_data = match read_file(&file_path) {
                    Ok(rom_data) => rom_data,
                    Err(error) => {
                        println!("{}", error);
                        event_handler.state = State::Splash;
                        continue;
                    }
                };

                let mut emulation = match Emulation::new(
                    rom_data,
                    colors.clone(),
                    event_handler.fast_forward.clone(),
                ) {
                    Ok(emulation) => emulation,
                    Err(error) => {
                        println!("{}", error);
                        event_handler.state = State::Splash;
                        continue;
                    }
                };

                match read_file(&file_path.replace(".gb", ".sav")) {
                    Ok(data) => emulation.cpu.memory_bus.load_game(data),
                    Err(_) => println!("Couldn't load game progress."),
                }

                emulation.run(
                    &mut event_handler,
                    start_time,
                    &mut egui_state,
                    &egui_ctx,
                    &mut event_pump,
                    &mut window,
                    &mut painter,
                    &mut ui_manager,
                    colors.clone(),
                    &audio_subsystem,
                );

                emulation
                    .cpu
                    .memory_bus
                    .save_game(&file_path.replace(".gb", ".sav"));

                event_handler.state = State::Splash;
                event_handler.quit = false;
            }
        }
    }

    Ok(())
}

fn initialize_gamepad(controller_subsystem: GameControllerSubsystem) -> Option<GameController> {
    let available = controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))
        .unwrap();
    (0..available).find_map(|id| {
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
    })
}

fn generate_frame_paths(start: usize, end: usize) -> Vec<String> {
    (start..=end)
        .map(|i| format!("./images/splash/gaemboi{}.png", i))
        .collect()
}

fn read_file(file_path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}
