/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use sdl2::{
    audio::{AudioDevice, AudioSpecDesired},
    pixels::Color,
};

use crate::{
    apu::audio::{Audio, SAMPLING_FREQUENCY, SAMPLING_RATE},
    clock::Clock,
    cpu::Cpu,
    event_handler::EventHandler,
    sdl::SDL,
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

        let frame_duration_nanos = (1_000_000_000.0 / FPS) as u64;
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

            // Component tick
            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.tick();
                self.cpu.memory_bus.tick(m_cycles, &mut sdl.window.canvas);
                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            let text: &str =
                &format!("VOL: {}% | FPS: {:.2}", event_handler.volume, &self.fps).to_string();
            sdl.window.render_text(text, Color::RGB(0, 255, 0));
            sdl.window.canvas.present();

            // Tick at 59.73 Hz using a spin-lock
            while frame_start_time.elapsed() < frame_duration {
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
