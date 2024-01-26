const LENGTH_TIMER: u16 = 0xFF20; // NR41
const VOLUME_ENVELOPE: u16 = 0xFF21; // NR42
const FREQUENCY_RANDOMNESS: u16 = 0xFF22; // NR43
const CHANNEL_CONTROL: u16 = 0xFF23; // NR44

pub struct NoiseChannel {}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            LENGTH_TIMER => {}
            VOLUME_ENVELOPE => {}
            FREQUENCY_RANDOMNESS => {}
            CHANNEL_CONTROL => {}
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
            }
        }

        0xFF
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            LENGTH_TIMER => {}
            VOLUME_ENVELOPE => {}
            FREQUENCY_RANDOMNESS => {}
            CHANNEL_CONTROL => {}
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}
