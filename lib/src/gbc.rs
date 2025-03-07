use std::error::Error;

use crate::{
    bus::Bus,
    cpu::{
        clock::{Clock, CYCLES_PER_FRAME},
        Cpu,
    },
};

const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;

pub struct GameBoyColor {
    cpu: Cpu,
    bus: Bus,
    clock: Clock,
}

impl GameBoyColor {
    pub fn new(rom_data: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
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

    pub fn step_frame(&mut self) {
        while self.clock.cycles_passed <= CYCLES_PER_FRAME {
            let m_cycles = self.cpu.step(&mut self.bus);
            // TODO: Handle user inputs (self.bus.joypad.handle_input(event_handler));
            self.bus.tick(m_cycles);
            self.clock.tick(m_cycles);
        }

        self.clock.reset();
    }
}
