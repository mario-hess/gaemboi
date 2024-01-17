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
