use std::fs::File;
use std::io::LineWriter;

use crate::clock::Clock;
use crate::cpu::Cpu;

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

    pub fn run(&mut self) {
        let path = "log/lines.txt";
        let file = File::create(path).expect("Could not create File.");
        let mut file = LineWriter::new(file);

        //let frame_duration = std::time::Duration::from_millis((1000.0 / FPS) as u64);

        loop {
            //let frame_start_time = std::time::Instant::now();

            while self.clock.cycles_passed <= self.clock.cycles_per_frame {
                let m_cycles = self.cpu.step(&mut file);
                self.tick(m_cycles);
            }

            self.clock.reset();

            //let elapsed_time = frame_start_time.elapsed();
            //if elapsed_time < frame_duration {
            //    std::thread::sleep(frame_duration - elapsed_time);
            //}
        }
    }

    fn tick(&mut self, m_cycles: u8) {
        self.timer_tick(m_cycles);
        self.clock.tick(m_cycles);
    }

    fn timer_tick(&mut self, m_cycles: u8) {
        self.cpu.memory_bus.io.timer.tick(m_cycles, &mut self.cpu.memory_bus.io.interrupt_flag);
    }
}
