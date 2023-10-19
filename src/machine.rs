/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    October 19, 2023
 */
use sdl2::{keyboard::Keycode, pixels::Color, ttf::Sdl2TtfContext, EventPump, VideoSubsystem};

use crate::clock::Clock;
use crate::config::Config;
use crate::cpu::Cpu;
use crate::debug_windows::DebugWindows;
use crate::event_handler::EventHandler;
use crate::ppu::{TILEMAP_END_0, TILEMAP_END_1, TILEMAP_START_0, TILEMAP_START_1};
use crate::window::{clear_canvas, Window};
use crate::Mode;

pub const FPS: f32 = 60.0;

pub struct Machine {
    cpu: Cpu,
    clock: Clock,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
            clock: Clock::new(),
        }
    }

    pub fn run(
        &mut self,
        config: &mut Config,
        event_pump: &mut EventPump,
        event_handler: &mut EventHandler,
        video_subsystem: &VideoSubsystem,
        ttf_context: &Sdl2TtfContext,
        viewport: &mut Window,
    ) {
        let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);
        let mut debug_windows = DebugWindows::build(video_subsystem, ttf_context, config);

        // Core emulation loop
        while event_handler.key_pressed != Some(Keycode::Escape) {
            event_handler.poll(event_pump);
            self.cpu
                .memory_bus
                .joypad
                .handle_input(&event_handler.key_pressed);

            if event_handler.file_dropped.is_some() {
                config.mode = Mode::Boot;
                break;
            }

            let frame_start_time = std::time::Instant::now();

            clear_canvas(&mut viewport.canvas);
            debug_windows.clear();

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();
            self.debug_draw(&mut debug_windows);
            self.cpu.memory_bus.ppu.draw_viewport(&mut viewport.canvas);

            viewport.canvas.present();
            debug_windows.present();

            // Tick at the CPU frequency rate
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
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
