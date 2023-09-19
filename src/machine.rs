use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::keyboard::Keyboard;
use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::{TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, WHITE};

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

        let mut viewport_canvas = self.create_canvas(
            &video_subsystem,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            SCALE,
            "gemboi",
        );

        let mut tile_table_canvas = self.create_canvas(
            &video_subsystem,
            TILE_TABLE_WIDTH,
            TILE_TABLE_HEIGHT,
            SCALE,
            "tile_table",
        );

        let mut event_pump = sdl_context.event_pump().unwrap();

        while !self.keyboard.escape_pressed {
            self.keyboard.set_key(&mut event_pump);

            viewport_canvas.set_draw_color(WHITE);
            viewport_canvas.clear();

            tile_table_canvas.set_draw_color(WHITE);
            tile_table_canvas.clear();

            let frame_start_time = std::time::Instant::now();

            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.step();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            self.debug_draw(&mut tile_table_canvas);

            viewport_canvas.present();
            tile_table_canvas.present();

            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
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

    fn debug_draw(&mut self, tile_table_canvas: &mut Canvas<Window>) {
        self.cpu
            .memory_bus
            .ppu
            .debug_draw_tile_table(tile_table_canvas);
    }
}
