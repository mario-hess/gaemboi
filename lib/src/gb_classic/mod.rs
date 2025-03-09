mod apu;
mod bus;
mod cartridge;
mod cpu;
mod io;
mod ppu;
pub mod utils;

use std::error::Error;

use crate::{
    gb_classic::{
        bus::Bus,
        cpu::{
            clock::{Clock, CYCLES_PER_FRAME},
            Cpu,
        },
    },
    utils::gb_factory::{Emulator, GameBoyType},
    AudioSamplesObserver, FrameBufferObserver, InputProvider,
};

const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;

pub struct GameBoyClassic {
    cpu: Cpu,
    bus: Bus,
    clock: Clock,
}

impl Emulator for GameBoyClassic {
    fn build(gb_type: &GameBoyType, rom_data: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        // If the header checksum is 0x00, then the carry and
        // half-carry flags are clear; otherwise, they are both set
        let flags_enabled = rom_data[HEADER_CHECKSUM_ADDRESS] != 0x00;

        let cpu = Cpu::new(flags_enabled)?;
        let bus = Bus::new(&rom_data)?;

        Ok(Self {
            cpu,
            bus,
            clock: Clock::new(),
        })
    }

    fn step_frame(&mut self) {
        while self.clock.cycles_passed <= CYCLES_PER_FRAME {
            let m_cycles = self.cpu.step(&mut self.bus);

            if let Some(provider) = &mut self.bus.joypad.input_provider {
                let button_state = provider.get_inputs();
                self.bus.joypad.set_inputs(button_state);
            }

            self.bus.tick(m_cycles);
            self.clock.tick(m_cycles);
        }

        self.clock.reset();
    }

    fn set_frame_buffer_observer(&mut self, observer: Box<dyn FrameBufferObserver>) {
        self.bus.ppu.frame_observer = Some(observer);
    }

    fn set_audio_samples_observer(&mut self, observer: Box<dyn AudioSamplesObserver>) {
        self.bus.apu.samples_observer = Some(observer);
    }

    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {
        self.bus.joypad.input_provider = Some(provider);
    }
}
