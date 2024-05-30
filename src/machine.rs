/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use sdl2::{
    audio::{AudioDevice, AudioSpecDesired},
    pixels::Color,
    rect::Point,
};

use crate::{
    apu::{audio::{Audio, SAMPLING_FREQUENCY, SAMPLING_RATE}, AUDIO_BUFFER_THRESHOLD},
    clock::Clock,
    cpu::Cpu,
    event_handler::EventHandler,
    memory_bus::ComponentTick,
    ppu::{BUFFER_SIZE, OVERLAP_MAP_SIZE, VIEWPORT_WIDTH, WHITE},
    sdl::{window::Window, SDL},
    MachineState,
};

pub const FPS: f32 = 59.7275;

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

    pub fn run(&mut self, sdl: &mut SDL, event_handler: &mut EventHandler) {
        let audio_device = self.create_audio_device(sdl, &event_handler.volume);
        audio_device.resume();

        // Using an unsigned integer for the frame duration effectively lets the
        // core emulation loop run at 62.5 FPS (16ms frame duration) instead of
        // 59.7275 FPS (16.74ms frame duration). This is needed to synchronize
        // the audio frequency with the cpu frequency.
        let frame_duration = std::time::Duration::from_millis((1_000.0 / FPS) as u64);

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

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.step();
                self.cpu.memory_bus.tick(m_cycles);

                if self.cpu.memory_bus.ppu.should_draw {
                    self.draw_viewport(&mut sdl.window, &event_handler.volume);
                }

                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            // Tick at 62.5 Hz using a spin-lock
            while frame_start_time.elapsed() < frame_duration {
                std::hint::spin_loop();
            }

            // Synchronize the audio frequency with the cpu frequency,
            // effectively clocking the system at ~59.73 Hz
            while self.cpu.memory_bus.apu.audio_buffer.len() > AUDIO_BUFFER_THRESHOLD {
                std::hint::spin_loop();
            }

            self.fps = 1.0 / frame_start_time.elapsed().as_secs_f32();

            /* This isn't precise enough as thread scheduling is OS-dependent
            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
            */
        }

        event_handler.pressed_escape = false;
    }

    fn draw_viewport(&mut self, window: &mut Window, volume: &u8) {
        for (index, pixel) in self.cpu.memory_bus.ppu.screen_buffer.iter().enumerate() {
            let x_coord = (index % VIEWPORT_WIDTH) as i32;
            let y_coord = (index / VIEWPORT_WIDTH) as i32;

            window.canvas.set_draw_color(*pixel);
            window
                .canvas
                .draw_point(Point::new(x_coord, y_coord))
                .unwrap();
        }

        let text: &str = &format!("VOL: {}% | FPS: {:.2}", volume, &self.fps).to_string();
        window.render_text(text, Color::RGB(0, 255, 0));
        window.canvas.present();
        self.cpu.memory_bus.ppu.should_draw = false;

        self.clear_screen();
    }

    fn clear_screen(&mut self) {
        for i in 0..OVERLAP_MAP_SIZE {
            if i < BUFFER_SIZE {
                self.cpu.memory_bus.ppu.screen_buffer[i] = WHITE;
            }
            self.cpu.memory_bus.ppu.overlap_map[i] = false;
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
