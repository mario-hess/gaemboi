const DAC_ENABLE: u16 = 0xFF1A; // NR30
const LENGTH_TIMER: u16 = 0xFF1B; // NR31
const OUTPUT_LEVEL: u16 = 0xFF1C; // NR32
const PERIOD_LOW: u16 = 0xFF1D; // NR33
const PERIOD_HIGH_CONTROL: u16 = 0xFF1E; // NR34

pub const WAVE_PATTERN_START: u16 = 0xFF30;
pub const WAVE_PATTERN_END: u16 = 0xFF3F;

pub struct WaveChannel {
    wave_ram: [u8; 32],
}

impl WaveChannel {
    pub fn new() -> Self {
        Self { wave_ram: [0; 32] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            DAC_ENABLE => {}
            LENGTH_TIMER => {}
            OUTPUT_LEVEL => {}
            PERIOD_LOW => {}
            PERIOD_HIGH_CONTROL => {}
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
            }
        }

        0xFF
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            DAC_ENABLE => {}
            LENGTH_TIMER => {}
            OUTPUT_LEVEL => {}
            PERIOD_LOW => {}
            PERIOD_HIGH_CONTROL => {}
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    pub fn read_wave_ram(&self, address: u16) -> u8 {
        let index = address - WAVE_PATTERN_START;

        self.wave_ram[index as usize]
    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8) {
        let index = address - WAVE_PATTERN_START;
        self.wave_ram[index as usize] = value;
    }
}
