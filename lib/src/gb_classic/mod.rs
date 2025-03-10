mod apu;
mod cartridge;
mod cpu;
mod interrupt;
mod io;
mod memory_bus;
mod ppu;
pub mod utils;

use std::error::Error;

use crate::{
    AudioSamplesListener, Emulator, FrameBufferListener, InputProvider,
    gb_classic::cpu::{
        Cpu,
        clock::{CYCLES_PER_FRAME, Clock},
    },
    utils::gb_factory::GameBoyType,
};

pub struct GameBoyClassic {
    cpu: Cpu,
    clock: Clock,
}

impl Emulator for GameBoyClassic {
    fn build(gb_type: &GameBoyType, rom_data: &[u8]) -> Result<Self, Box<dyn Error>> {
        let cpu = Cpu::new(rom_data)?;

        Ok(Self {
            cpu,
            clock: Clock::new(),
        })
    }

    fn step_frame(&mut self) {
        while self.clock.cycles_passed <= CYCLES_PER_FRAME {
            let m_cycles = self.cpu.step();
            if let Some(provider) = &mut self.cpu.memory_bus.joypad.input_provider {
                let button_state = provider.on_input();
                self.cpu.memory_bus.joypad.apply_input(button_state);
            }
            self.cpu.memory_bus.tick(m_cycles);
            self.clock.tick(m_cycles);
        }

        self.clock.reset();
    }

    fn set_frame_buffer_listener(&mut self, listener: Box<dyn FrameBufferListener>) {
        self.cpu
            .memory_bus
            .ppu
            .set_frame_buffer_listener(Some(listener));
    }

    fn set_audio_samples_listener(&mut self, listener: Box<dyn AudioSamplesListener>) {
        self.cpu
            .memory_bus
            .apu
            .set_audio_samples_listener(Some(listener));
    }

    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {
        self.cpu
            .memory_bus
            .joypad
            .set_input_provider(Some(provider));
    }
}
