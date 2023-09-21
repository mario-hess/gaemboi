/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    September 21, 2023
 */
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::keyboard::Keyboard;
use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::{TILE_MAP_END_0, TILE_MAP_END_1, TILE_MAP_START_0, TILE_MAP_START_1, WHITE};

const TILE_TABLE_WIDTH: usize = 256;
const TILE_TABLE_HEIGHT: usize = 96;
const TILE_MAP_WIDTH: usize = TILE_TABLE_WIDTH;

pub const FPS: f32 = 60.0;

pub struct Machine {
    cpu: Cpu,
    clock: Clock,
    keyboard: Keyboard,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
            clock: Clock::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn run(&mut self) {
        let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let (
            mut viewport_canvas,
            mut tile_table_canvas,
            mut tile_map_0_canvas,
            mut tile_map_1_canvas,
        ) = self.create_windows(&video_subsystem);

        let mut event_pump = sdl_context.event_pump().unwrap();

        // Core emulation loop
        while !self.keyboard.escape_pressed {
            self.keyboard.set_key(&mut event_pump);

            self.clear_canvases([
                &mut viewport_canvas,
                &mut tile_table_canvas,
                &mut tile_map_0_canvas,
                &mut tile_map_1_canvas,
            ]);

            let frame_start_time = std::time::Instant::now();

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            // Draw debug windows (Tile Data & Tile Maps)
            self.debug_draw(&mut tile_table_canvas, &mut tile_map_0_canvas, &mut tile_map_1_canvas);

            self.present_canvases([
                &mut viewport_canvas,
                &mut tile_table_canvas,
                &mut tile_map_0_canvas,
                &mut tile_map_1_canvas,
            ]);

            // Tick at the CPU frequency rate
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
    }

    fn create_windows(
        &mut self,
        video_subsystem: &VideoSubsystem,
    ) -> (
        Canvas<Window>,
        Canvas<Window>,
        Canvas<Window>,
        Canvas<Window>,
    ) {
        let viewport_canvas = self.create_canvas(
            video_subsystem,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            SCALE,
            "gemboi",
        );

        let tile_table_canvas = self.create_canvas(
            video_subsystem,
            TILE_TABLE_WIDTH,
            TILE_TABLE_HEIGHT,
            SCALE,
            "tile_table",
        );

        let tile_map_0 = self.create_canvas(
            video_subsystem,
            TILE_MAP_WIDTH,
            TILE_MAP_WIDTH,
            SCALE,
            "tile_map_0",
        );

        let tile_map_1 = self.create_canvas(
            video_subsystem,
            TILE_MAP_WIDTH,
            TILE_MAP_WIDTH,
            SCALE,
            "tile_map_1",
        );

        (viewport_canvas, tile_table_canvas, tile_map_0, tile_map_1)
    }

    fn create_canvas(
        &mut self,
        video_subsystem: &VideoSubsystem,
        width: usize,
        height: usize,
        scale: usize,
        title: &str,
    ) -> Canvas<Window> {
        let window = video_subsystem
            .window(title, (width * scale) as u32, (height * scale) as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas
            .set_logical_size(width as u32, height as u32)
            .unwrap();

        canvas
    }

    fn clear_canvases(&mut self, canvases: [&mut Canvas<Window>; 4]) {
        for canvas in canvases {
            canvas.set_draw_color(WHITE);
            canvas.clear();
        }
    }

    fn present_canvases(&mut self, canvases: [&mut Canvas<Window>; 4]) {
        for canvas in canvases {
            canvas.present();
        }
    }

    fn debug_draw(
        &mut self,
        tile_table_canvas: &mut Canvas<Window>,
        tile_map_0_canvas: &mut Canvas<Window>,
        tile_map_1_canvas: &mut Canvas<Window>,
    ) {
        self.cpu
            .memory_bus
            .ppu
            .debug_draw_tile_table(tile_table_canvas);

        self.cpu.memory_bus.ppu.debug_draw_tile_map(
            tile_map_0_canvas,
            TILE_MAP_START_0,
            TILE_MAP_END_0,
        );

        self.cpu.memory_bus.ppu.debug_draw_tile_map(
            tile_map_1_canvas,
            TILE_MAP_START_1,
            TILE_MAP_END_1,
        );
    }
}
