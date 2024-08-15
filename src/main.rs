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
mod fps_counter;
mod interrupt;
mod machine;
mod memory_bus;
mod ppu;
mod sdl;

use apu::audio::{Audio, SAMPLING_FREQUENCY, SAMPLING_RATE};
use apu::AUDIO_BUFFER_THRESHOLD;
use egui_sdl2_gl::egui::load::SizedTexture;
use egui_sdl2_gl::egui::{
    menu, Color32, Context, FullOutput, Image, Label, Slider, TopBottomPanel, Vec2,
};
use egui_sdl2_gl::sdl2::audio::{AudioDevice, AudioSpecDesired};
use egui_sdl2_gl::sdl2::pixels::Color;
use egui_sdl2_gl::sdl2::video::GLProfile;
use egui_sdl2_gl::sdl2::AudioSubsystem;
use egui_sdl2_gl::{gl, DpiScaling, ShaderVersion};
use ppu::{TILETABLE_HEIGHT, TILETABLE_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE};
use std::collections::VecDeque;
use std::io::{Error, Read};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const FRAME_DURATION_MS: f64 = 16.7433;
const FRAME_DURATION_MICROS: u64 = (FRAME_DURATION_MS * 1_000.0) as u64;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(FRAME_DURATION_MICROS);

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Game,
    Tiletable,
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let config = config::Config::build(&args);

    // INITIALIZE STUFF
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
    // ------------------------------------------------------

    let menu_bar_height = 20.0;
    let mut event_handler = event_handler::EventHandler::new();

    let mut window = video_subsystem
        .window(
            "SDL2 + Egui Example",
            VIEWPORT_WIDTH as u32 * event_handler.window_scale,
            VIEWPORT_HEIGHT as u32 * event_handler.window_scale + menu_bar_height as u32,
        )
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    let (mut painter, mut egui_state) =
        egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = Context::default();
    let mut event_pump: egui_sdl2_gl::sdl2::EventPump = sdl_context.event_pump().unwrap();

    let mut frame_times = Vec::new();
    let mut frame_count = 0;
    let mut last_second = std::time::Instant::now();
    let mut fps = 0.0;

    unsafe {
        egui_sdl2_gl::sdl2::sys::SDL_GL_SetSwapInterval(0);
    }
    let game_texture_id = painter.new_user_texture(
        (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        &vec![Color32::BLACK; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
        false,
    );

    let tiletable_texture_id = painter.new_user_texture(
        (TILETABLE_WIDTH, TILETABLE_HEIGHT),
        &vec![Color32::BLACK; TILETABLE_WIDTH * TILETABLE_HEIGHT],
        false,
    );
    let mut current_tab = Tab::Game;
    let mut previous_tab = Tab::Game;

    let start_time = std::time::Instant::now();

    let rom_data = read_file(config.file_path.unwrap())?;
    let mut machine = machine::Machine::new(rom_data);

    let audio_device = create_audio_device(
        &audio_subsystem,
        &machine.cpu.memory_bus.apu.left_volume,
        &machine.cpu.memory_bus.apu.right_volume,
        &event_handler.volume,
        &mut machine.cpu.memory_bus.apu.audio_buffer,
    );
    audio_device.resume();

    // ---------------- LOOP ------------------------
    while !event_handler.quit {
        let frame_start_time = std::time::Instant::now();
        let time = start_time.elapsed().as_secs_f64();
        egui_state.input.time = Some(time);
        egui_ctx.begin_frame(egui_state.input.take());

        event_handler.poll(&mut event_pump, &mut egui_state, &window, &mut painter);
        machine.cpu.memory_bus.joypad.handle_input(&event_handler);
        // Clear the background
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        machine.run();

        TopBottomPanel::top("top_panel")
            .exact_height(menu_bar_height)
            .show(&egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    // Menu on the left
                    menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open").clicked() {
                                // Handle Open action
                            }
                        });

                        ui.menu_button("Settings", |ui| {
                            if ui.button("Keybindings").clicked() {
                                // Handle Open action
                            }
                            if ui.button("Color Scheme").clicked() {
                                // Handle Open action
                            }
                        });

                        ui.menu_button("Help", |ui| {
                            if ui.button("About").clicked() {
                                // Handle About action
                            }
                        });

                        ui.separator();
                        ui.label(format!("VOL: {}%", event_handler.volume));

                        ui.separator();
                        ui.label(format!("FPS: {:.2}", fps));
                    });

                    // Tabs on the right
                    ui.with_layout(
                        egui_sdl2_gl::egui::Layout::right_to_left(
                            egui_sdl2_gl::egui::Align::Center,
                        ),
                        |ui| {
                            ui.add_space(2.0);
                            if ui.button("Quit").clicked() {
                                event_handler.quit = true;
                            }
                            ui.separator();
                            ui.selectable_value(&mut current_tab, Tab::Tiletable, "Tiletable");
                            ui.selectable_value(&mut current_tab, Tab::Game, "Viewport");
                        },
                    );
                });
            });

        if current_tab != previous_tab {
            match current_tab {
                Tab::Game => {
                    window
                        .set_size(
                            VIEWPORT_WIDTH as u32 * event_handler.window_scale,
                            VIEWPORT_HEIGHT as u32 * event_handler.window_scale
                                + menu_bar_height as u32,
                        )
                        .unwrap();
                }
                Tab::Tiletable => {
                    window
                        .set_size(
                            TILETABLE_WIDTH as u32 * event_handler.window_scale,
                            TILETABLE_HEIGHT as u32 * event_handler.window_scale
                                + menu_bar_height as u32,
                        )
                        .unwrap();
                }
            }
            previous_tab = current_tab;
            window.set_position(
                egui_sdl2_gl::sdl2::video::WindowPos::Centered,
                egui_sdl2_gl::sdl2::video::WindowPos::Centered,
            );
        }

        egui_sdl2_gl::egui::CentralPanel::default()
            .frame(egui_sdl2_gl::egui::Frame::none())
            .show(&egui_ctx, |ui| match current_tab {
                Tab::Game => {
                    let game_background: Vec<Color32> = machine
                        .cpu
                        .memory_bus
                        .ppu
                        .viewport_buffer
                        .iter()
                        .map(|color| Color32::from_rgb(color.r, color.g, color.b))
                        .collect();
                    painter.update_user_texture_data(game_texture_id, &game_background);
                    let game_image = Image::new(SizedTexture::new(
                        game_texture_id,
                        Vec2::new(
                            VIEWPORT_WIDTH as f32 * event_handler.window_scale as f32,
                            VIEWPORT_HEIGHT as f32 * event_handler.window_scale as f32,
                        ),
                    ))
                    .maintain_aspect_ratio(true);
                    ui.add(game_image);
                    machine.cpu.memory_bus.ppu.clear_screen();
                    machine.cpu.memory_bus.ppu.should_draw = false;
                }
                Tab::Tiletable => {
                    let tiletable_background: Vec<Color32> = machine
                        .cpu
                        .memory_bus
                        .ppu
                        .tile_table()
                        .iter()
                        .map(|color| Color32::from_rgb(color.r, color.g, color.b))
                        .collect();
                    painter.update_user_texture_data(tiletable_texture_id, &tiletable_background);
                    let tiletable_image = Image::new(SizedTexture::new(
                        tiletable_texture_id,
                        Vec2::new(
                            TILETABLE_WIDTH as f32 * event_handler.window_scale as f32,
                            TILETABLE_HEIGHT as f32 * event_handler.window_scale as f32,
                        ),
                    ))
                    .maintain_aspect_ratio(true);
                    ui.add(tiletable_image);
                }
            });

        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_frame();

        // Process output
        egui_state.process_output(&window, &platform_output);

        // Paint jobs
        let paint_jobs = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_jobs(None, textures_delta, paint_jobs);

        // Swap the window
        window.gl_swap_window();

        if should_delay(frame_start_time, &machine.cpu.memory_bus.apu.audio_buffer) {
            std::thread::sleep(std::time::Duration::from_micros(
                (FRAME_DURATION.as_micros() - frame_start_time.elapsed().as_micros()) as u64,
            ));
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
    }

    Ok(())
}

/*
fn main() -> Result<(), Error> {
    // Build config
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    let mut event_handler = EventHandler::new();

    let mut sdl = SDL::new(&event_handler, &config);

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
                let path = event_handler.file_path.clone().unwrap();
                let rom_data = read_file(path.clone())?;

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

*/

fn read_file(file_path: String) -> Result<Vec<u8>, Error> {
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
) -> bool {
    frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros()
        && audio_buffer.lock().unwrap().len() > AUDIO_BUFFER_THRESHOLD
}
