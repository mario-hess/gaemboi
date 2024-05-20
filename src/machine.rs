/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    May 20, 2024
 */
use sdl2::{
    audio::AudioSpecDesired,
    pixels::Color,
    ttf::Sdl2TtfContext,
    AudioSubsystem, EventPump, VideoSubsystem,
};

use crate::{
    apu::audio::{Audio, SAMPLING_RATE, SAMPLING_FREQUENCY},
    clock::Clock,
    config::Config,
    cpu::Cpu,
    debug_windows::DebugWindows,
    event_handler::EventHandler,
    ppu::{TILEMAP_END_0, TILEMAP_END_1, TILEMAP_START_0, TILEMAP_START_1},
    window::Window,
    MachineState,
};

pub const FPS: f32 = 59.73;

pub struct Machine {
    pub cpu: Cpu,
    clock: Clock,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
            clock: Clock::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &mut self,
        config: &mut Config,
        event_pump: &mut EventPump,
        event_handler: &mut EventHandler,
        video_subsystem: &VideoSubsystem,
        ttf_context: &Sdl2TtfContext,
        viewport: &mut Window,
        audio_subsystem: &mut AudioSubsystem,
    ) {
        // Create audio device
        let device = AudioSpecDesired {
            freq: Some(SAMPLING_FREQUENCY as i32),
            samples: Some(SAMPLING_RATE),
            channels: Some(2),
        };

        let left_volume = &self.cpu.memory_bus.apu.left_volume;
        let right_volume = &self.cpu.memory_bus.apu.right_volume;
        let audio = Audio::new(&mut self.cpu.memory_bus.apu.audio_buffer, left_volume, right_volume);
        let audio_device = audio_subsystem
            .open_playback(None, &device, |_spec| audio)
            .unwrap();

        let frame_duration_nanos = (1_000_000_000.0 / FPS) as u64;
        let frame_duration = std::time::Duration::from_nanos(frame_duration_nanos);

        let mut debug_windows = DebugWindows::build(video_subsystem, ttf_context, config);

        audio_device.resume();

        // Core emulation loop
        while !event_handler.pressed_escape {
            let frame_start_time = std::time::Instant::now();

            event_handler.poll(event_pump);
            event_handler.check_resized(&mut viewport.canvas);
            self.cpu.memory_bus.joypad.handle_input(event_handler);

            // Boot new game on file-drop
            if event_handler.file_path.is_some() {
                event_handler.machine_state = MachineState::Boot;
                break;
            }

            debug_windows.clear();

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles, &mut viewport.canvas);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();
            self.debug_draw(&mut debug_windows);

            viewport.canvas.present();
            debug_windows.present();

            // Tick at 59.73 Hz using a busy-wait loop 
            while frame_start_time.elapsed() < frame_duration {}

            /* This isn't precise enough as thread scheduling is OS-dependent
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
            */
        }

        event_handler.pressed_escape = false;
    }

    fn debug_draw(&mut self, windows: &mut DebugWindows) {
        if let Some(ref mut tiletable) = windows.tiletable {
            self.cpu
                .memory_bus
                .ppu
                .debug_draw_tile_table(&mut tiletable.canvas);
            tiletable.render_text("tile table", Color::RGB(0, 255, 0));
        }

        if let Some(ref mut tilemap_9800) = windows.tilemap_9800 {
            self.cpu.memory_bus.ppu.debug_draw_tile_map(
                &mut tilemap_9800.canvas,
                TILEMAP_START_0,
                TILEMAP_END_0,
            );
            tilemap_9800.render_text("tilemap 9800", Color::RGB(0, 255, 0));
        }

        if let Some(ref mut tilemap_9c00) = windows.tilemap_9c00 {
            self.cpu.memory_bus.ppu.debug_draw_tile_map(
                &mut tilemap_9c00.canvas,
                TILEMAP_START_1,
                TILEMAP_END_1,
            );
            tilemap_9c00.render_text("tilemap 9C00", Color::RGB(0, 255, 0));
        }
    }
}
