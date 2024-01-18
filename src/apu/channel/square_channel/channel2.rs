use crate::apu::channel::square_channel::{core::Core, SquareChannelController};

const LENGTH_TIMER: u16 = 0xFF16;       // NR21
const VOLUME_ENVELOPE: u16 = 0xFF17;    // NR22
const PERIOD_LOW: u16 = 0xFF18;         // NR23
const PERIOD_HIGH_CONTROL: u16 = 0xFF19;// NR24

pub struct Channel2 {}

impl Channel2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SquareChannelController for Channel2 {
    fn read_byte(&self, core: &Core, address: u16) -> u8 {
        0xFF
    }

    fn write_byte(&mut self, core: &mut Core, address: u16, value: u8) {
        match address {
            LENGTH_TIMER => {}
            VOLUME_ENVELOPE => {}
            PERIOD_LOW => {}
            PERIOD_HIGH_CONTROL => {}
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}
