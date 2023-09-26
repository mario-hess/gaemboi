/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    September 26, 2023
 */
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::event_handler::EventHandler;
use crate::ppu::{TILE_MAP_END_0, TILE_MAP_END_1, TILE_MAP_START_0, TILE_MAP_START_1};
use crate::windows::Windows;

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
        event_pump: &mut EventPump,
        event_handler: &mut EventHandler,
        windows: &mut Windows,
    ) {
        let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);

        // Core emulation loop
        while event_handler.event_key != Some(Keycode::Escape) {
            if event_handler.event_file.is_some() {
                break;
            }

            let frame_start_time = std::time::Instant::now();

            Windows::clear(windows);
            event_handler.poll(event_pump);

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();
            self.debug_draw(windows);
            Windows::present(windows);

            // Tick at the CPU frequency rate
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
    }

    fn debug_draw(&mut self, windows: &mut Windows) {
        if let Some(ref mut tiletable) = windows.tiletable {
            self.cpu.memory_bus.ppu.debug_draw_tile_table(tiletable);
        }

        if let Some(ref mut tilemap_9800) = windows.tilemap_9800 {
            self.cpu.memory_bus.ppu.debug_draw_tile_map(
                tilemap_9800,
                TILE_MAP_START_0,
                TILE_MAP_END_0,
            );
        }

        if let Some(ref mut tilemap_9c00) = windows.tilemap_9c00 {
            self.cpu.memory_bus.ppu.debug_draw_tile_map(
                tilemap_9c00,
                TILE_MAP_START_1,
                TILE_MAP_END_1,
            );
        }
    }
}
