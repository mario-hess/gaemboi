use crate::apu::channel::square_channel::{core::Core, SquareChannelController};

const SWEEP: u16 = 0xFF10; // NR10
const LENGTH_TIMER: u16 = 0xFF11; // NR11
const VOLUME_ENVELOPE: u16 = 0xFF12; // NR12
const PERIOD_LOW: u16 = 0xFF13; // NR13
const PERIOD_HIGH_CONTROL: u16 = 0xFF14; // NR14

pub struct Channel1 {
    sweep_sequence: u8,
    // NR10
    step: u8,
    direction: bool,
    pace: u8,
}

impl Channel1 {
    pub fn new() -> Self {
        Self {
            sweep_sequence: 0,
            step: 0,
            direction: true,
            pace: 0,
        }
    }
}

impl Channel1 {
    fn get_sweep(&self) -> u8 {
        self.step & 0x07 | if self.direction { 0x08 } else { 0x0 } | (self.pace & 0x07) << 4 | 0x80
    }

    fn set_sweep(&mut self, value: u8) {
        self.step = value & 0x07;
        self.direction = value & 0x08 == 0x00;
        self.pace = value & 0x70;
        self.sweep_sequence = 0x00;
    }
}

impl SquareChannelController for Channel1 {
    fn read_byte(&self, core: &Core, address: u16) -> u8 {
        match address {
            SWEEP => return self.get_sweep(),
            LENGTH_TIMER => return core.get_length_timer(),
            VOLUME_ENVELOPE => return core.get_volume_envelope(),
            PERIOD_LOW => {}
            PERIOD_HIGH_CONTROL => {}
            _ => eprintln!("Unknown address: {:#X} Can't read byte.", address),
        }

        0xFF
    }

    fn write_byte(&mut self, core: &mut Core, address: u16, value: u8) {
        match address {
            SWEEP => self.set_sweep(value),
            LENGTH_TIMER => core.set_length_timer(value),
            VOLUME_ENVELOPE => core.set_volume_envelope(value),
            PERIOD_LOW => {}
            PERIOD_HIGH_CONTROL => {}
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}
