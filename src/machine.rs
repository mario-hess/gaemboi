/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use sdl2::{
    audio::{AudioDevice, AudioSpecDesired},
    rect::Point,
};

use crate::{
    apu::{
        audio::{Audio, SAMPLING_FREQUENCY, SAMPLING_RATE},
        AUDIO_BUFFER_THRESHOLD,
    },
    clock::{Clock, CYCLES_PER_FRAME},
    cpu::Cpu,
    event_handler::EventHandler,
    memory_bus::ComponentTick,
    ppu::{BLACK, VIEWPORT_WIDTH, WHITE},
    sdl::{window::Window, SDL},
    MachineState,
};

const FRAME_DURATION_MS: f64 = 16.67;
pub const FPS: f32 = 59.7275;
pub const STATUSBAR_OFFSET: u8 = 8;

pub struct Machine {
    pub cpu: Cpu,
    clock: Clock,
    fps: f32,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
            clock: Clock::new(),
            fps: 0.0,
        }
    }

    pub fn run(&mut self, sdl: &mut SDL, event_handler: &mut EventHandler, game_title: String) {
        let audio_device = self.create_audio_device(sdl, &event_handler.volume);
        audio_device.resume();

        // Use a frame duration of 16.7ms instead of 16.74ms.
        // This is needed to synchronize the audio frequency with the cpu frequency.
        let frame_duration_nanos = (FRAME_DURATION_MS * 1_000_000.0) as u64;
        let frame_duration = std::time::Duration::from_nanos(frame_duration_nanos);

        // Core emulation loop
        while !event_handler.pressed_escape {
            let frame_start_time = std::time::Instant::now();

            event_handler.poll(&mut sdl.event_pump);
            event_handler.check_resized(&mut sdl.window.canvas);
            self.cpu.memory_bus.joypad.handle_input(event_handler);

            // Boot new game on file-drop
            if event_handler.file_path.is_some() {
                event_handler.machine_state = MachineState::Boot;
                break;
            }

            sdl.window.canvas.set_draw_color(WHITE);
            sdl.window.canvas.clear();

            // Component tick
            while self.clock.cycles_passed <= CYCLES_PER_FRAME {
                let m_cycles = self.cpu.step();
                self.cpu.memory_bus.tick(m_cycles);

                if self.cpu.memory_bus.ppu.should_draw {
                    self.draw_statusbar(
                        &mut sdl.window,
                        &event_handler.volume,
                        &mut game_title.clone(),
                    );
                    self.draw_viewport(&mut sdl.window);
                    sdl.window.canvas.present();
                }

                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            if event_handler.potato_mode {
                if self.cpu.memory_bus.apu.audio_buffer.len() > AUDIO_BUFFER_THRESHOLD
                    && frame_start_time.elapsed() < frame_duration
                {
                    std::thread::sleep(frame_duration - frame_start_time.elapsed());
                }
            } else {
                while frame_start_time.elapsed() < frame_duration {
                    std::hint::spin_loop();
                }

                while self.cpu.memory_bus.apu.audio_buffer.len() > AUDIO_BUFFER_THRESHOLD {
                    std::hint::spin_loop();
                }
            }

            self.fps = 1.0 / frame_start_time.elapsed().as_secs_f32();
        }

        event_handler.pressed_escape = false;
    }

    fn draw_viewport(&mut self, window: &mut Window) {
        for (index, pixel) in self.cpu.memory_bus.ppu.viewport_buffer.iter().enumerate() {
            let x_coord = (index % VIEWPORT_WIDTH) as i32;
            let y_coord = (index / VIEWPORT_WIDTH) as i32;

            window.canvas.set_draw_color(*pixel);
            window
                .canvas
                .draw_point(Point::new(x_coord, y_coord + STATUSBAR_OFFSET as i32))
                .unwrap();
        }

        self.cpu.memory_bus.ppu.should_draw = false;
        self.cpu.memory_bus.ppu.clear_screen();
    }

    fn draw_statusbar(&mut self, window: &mut Window, volume: &u8, game_title: &mut String) {
        let fps_str = format!("VOL: {}% | FPS: {:.2}", volume, self.fps);
        let fps_str_position = Point::new(106, 0);

        if game_title.len() > 20 {
            game_title.truncate(20);
            *game_title = game_title.to_owned() + "...";
        }

        let title_str = game_title.clone();
        let title_position = Point::new(1, 0);

        // Render text
        window.render_text(&fps_str, BLACK, fps_str_position);
        window.render_text(&title_str, BLACK, title_position);

        // Status bar bottom border
        for i in 0..160 {
            window.canvas.set_draw_color(BLACK);
            window
                .canvas
                .draw_point(Point::new(i, STATUSBAR_OFFSET as i32 - 1))
                .unwrap();
        }
    }

    fn create_audio_device<'a, 'b: 'a>(
        &'a mut self,
        sdl: &SDL,
        volume: &'b u8,
    ) -> AudioDevice<Audio<'_>> {
        let device = AudioSpecDesired {
            freq: Some(SAMPLING_FREQUENCY as i32),
            samples: Some(SAMPLING_RATE),
            channels: Some(2),
        };

        let left_volume = &self.cpu.memory_bus.apu.left_volume;
        let right_volume = &self.cpu.memory_bus.apu.right_volume;
        let audio = Audio::new(
            &mut self.cpu.memory_bus.apu.audio_buffer,
            left_volume,
            right_volume,
            volume,
        );

        sdl.audio_subsystem
            .open_playback(None, &device, |_spec| audio)
            .unwrap()
    }
}
