/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    September 23, 2023
 */
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::clock::Clock;
use crate::config::Config;
use crate::cpu::Cpu;
use crate::event_handler::EventHandler;
use crate::ppu::{TILE_MAP_END_0, TILE_MAP_END_1, TILE_MAP_START_0, TILE_MAP_START_1};
use crate::window;

pub const FPS: f32 = 60.0;

pub struct Machine<'a> {
    cpu: Cpu,
    clock: Clock,
    config: &'a Option<Config>,
}

impl<'a> Machine<'a> {
    pub fn new(config: &'a Option<Config>, rom_data: Vec<u8>) -> Self {
        Self {
            config,
            cpu: Cpu::new(rom_data),
            clock: Clock::new(),
        }
    }

    pub fn run(
        &mut self,
        event_pump: &mut EventPump,
        event_handler: &mut EventHandler,
        windows: &mut Vec<Canvas<Window>>,
    ) {
        let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);

        // Core emulation loop
        while event_handler.event_key != Some(Keycode::Escape) {
            if event_handler.event_file.is_some() {
                break;
            }

            let frame_start_time = std::time::Instant::now();

            event_handler.poll(event_pump);
            window::clear_canvases(windows);

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();
            self.debug_draw(windows);
            window::present_canvases(windows);

            // Tick at the CPU frequency rate
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
    }

    fn debug_draw(&mut self, windows: &mut [Canvas<Window>]) {
        let mut counter = 1;

        if let Some(config) = self.config {
            if config.tiletable_enable {
                self.cpu
                    .memory_bus
                    .ppu
                    .debug_draw_tile_table(&mut windows[counter]);
                counter += 1;
            }

            if config.tilemaps_enable {
                self.cpu.memory_bus.ppu.debug_draw_tile_map(
                    &mut windows[counter],
                    TILE_MAP_START_0,
                    TILE_MAP_END_0,
                );
                counter += 1;

                self.cpu.memory_bus.ppu.debug_draw_tile_map(
                    &mut windows[counter],
                    TILE_MAP_START_1,
                    TILE_MAP_END_1,
                );
            }
        }
    }
}
