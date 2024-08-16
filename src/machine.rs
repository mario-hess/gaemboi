use std::time::Duration;

/**
 * @file    machine.rs
 * @brief   Orchestrates the emulation loop, utilizing SDL2 for rendering and input handling.
 * @author  Mario Hess
 * @date    May 23, 2024
 */
use crate::{
    clock::{Clock, CYCLES_PER_FRAME},
    cpu::Cpu,
    memory_bus::ComponentTick,
};

const FRAME_DURATION_MS: f64 = 16.7433;
const FRAME_DURATION_MICROS: u64 = (FRAME_DURATION_MS * 1_000.0) as u64;
const FRAME_DURATION: Duration = std::time::Duration::from_micros(FRAME_DURATION_MICROS);
pub const FPS: f32 = 59.7275;

pub struct Machine {
    pub cpu: Cpu,
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
        while self.clock.cycles_passed <= CYCLES_PER_FRAME {
            let m_cycles = self.cpu.step();
            self.cpu.memory_bus.tick(m_cycles);
            self.clock.tick(m_cycles);
        }

        self.clock.reset();
    }
}
