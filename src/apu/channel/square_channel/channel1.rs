use crate::apu::channel::square_channel::{core::Core, SquareChannelController};

const SWEEP: u16 = 0xFF10;              // NR10
const LENGTH_TIMER: u16 = 0xFF11;       // NR11
const VOLUME_ENVELOPE: u16 = 0xFF12;    // NR12
const PERIOD_LOW: u16 = 0xFF13;         // NR13
const PERIOD_HIGH_CONTROL: u16 = 0xFF14;// NR14

pub struct Channel1 {}

impl Channel1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SquareChannelController for Channel1 {
    fn read_byte(&self, core: &Core, address: u16) -> u8 {
        0xFF
    }

    fn write_byte(&mut self, core: &mut Core, address: u16, value: u8) {
        match address {
            SWEEP => {},
            LENGTH_TIMER => {},
            VOLUME_ENVELOPE => {},
            PERIOD_LOW => {},
            PERIOD_HIGH_CONTROL => {},
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}
