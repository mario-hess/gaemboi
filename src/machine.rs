use crate::ppu::{BLACK, DARK, LIGHT, WHITE};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::keyboard::Keyboard;
use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

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

        let tilemap_width = 16 * 8;
        let tilemap_height = 32 * 8;
        let mut tilemap_canvas = self.create_canvas(
            &video_subsystem,
            tilemap_width,
            tilemap_height,
            SCALE,
            "tilemap",
        );

        let mut event_pump = sdl_context.event_pump().unwrap();

        while !self.keyboard.escape_pressed {
            self.keyboard.set_key(&mut event_pump);

            viewport_canvas.set_draw_color(WHITE);
            viewport_canvas.clear();

            tilemap_canvas.set_draw_color(WHITE);
            tilemap_canvas.clear();

            let frame_start_time = std::time::Instant::now();

            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.step();
                self.cpu.memory_bus.tick(m_cycles);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            viewport_canvas.present();
            tilemap_canvas.present();

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
}
