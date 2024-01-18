const CH3_DAC_ENABLE: u16 = 0xFF1A; // NR30
const CH3_LENGTH_TIMER: u16 = 0xFF1B; // NR31
const CH3_OUTPUT_LEVEL: u16 = 0xFF1C; // NR32
const CH3_PERIOD_LOW: u16 = 0xFF1D; // NR33
const CH3_PERIOD_HIGH_CONTROL: u16 = 0xFF1E; // NR34

pub struct WaveChannel {
    wave_ram: [u8; 32]
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            wave_ram: [0; 32],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {

    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8) {

    }
}
