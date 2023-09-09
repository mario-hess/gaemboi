use crate::cpu::Cpu;
use crate::timer::Timer;

pub const FPS: f32 = 60.0;

pub struct Machine {
    cpu: Cpu,
    timer: Timer,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
            timer: Timer::new(),
        }
    }

    pub fn run(&mut self) {
        let frame_duration = std::time::Duration::from_secs_f32(1.0 / FPS);

        loop {
            let frame_start_time = std::time::Instant::now();

            while self.timer.cycles_passed <= self.timer.cycles_per_frame {
                let m_cycles = self.cpu.step();
                self.timer.tick(m_cycles);
            }

            self.timer.reset();

            let elapsed_time = frame_start_time.elapsed();
            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
    }
}
