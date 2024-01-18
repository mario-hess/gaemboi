const CH4_LENGTH_TIMER: u16 = 0xFF20; // NR41
const CH4_VOLUME_ENVELOPE: u16 = 0xFF21; // NR42
const CH4_FREQUENCY_RANDOMNESS: u16 = 0xFF22; // NR43
const CH4_CHANNEL_CONTROL: u16 = 0xFF23; // NR44

pub struct NoiseChannel {}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {}
}
