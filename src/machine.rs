use crate::ppu::{BLACK, DARK, LIGHT, WHITE};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::keyboard::Keyboard;
use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::tile::Tile;

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

        let tilemap_width = 256;
        let tilemap_height = 64;
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

            let start_address: u16 = 0x8000;
            let end_address: u16 = 0x8FFF;
            let tile_width = 8;
            let tile_height = 8;

            let mut tilemap = Vec::<u8>::new();

            for i in start_address..=end_address {
                tilemap.push(self.cpu.memory_bus.read_byte(i));
            }

            let mut tiles = Vec::<Tile>::new();

            for chunk in tilemap.chunks(16) {
                if chunk.len() == 16 {
                    let mut tile_bytes = [0; 16];
                    tile_bytes.copy_from_slice(chunk);

                    let tile = Tile::new(tile_bytes);
                    tiles.push(tile);
                }
            }

            for row in 0..tilemap_height {
                for col in 0..tilemap_width {
                    let tile_index = row * 32 + col;

                    if tile_index < tiles.len() {
                        let tile = &tiles[tile_index];

                        let x = col * tile_width;
                        let y = row * tile_height;

                        for (tile_row, row_pixels) in tile.data.iter().enumerate() {
                            for (tile_col, pixel) in row_pixels.iter().enumerate() {
                                let color = match *pixel {
                                    WHITE => WHITE,
                                    LIGHT => LIGHT,
                                    DARK => DARK,
                                    BLACK => BLACK,
                                    _ => unreachable!(),
                                };

                                tilemap_canvas.set_draw_color(color);

                                tilemap_canvas
                                    .draw_point(Point::new(
                                        x as i32 + tile_col as i32,
                                        y as i32 + tile_row as i32,
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                }
            }

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
