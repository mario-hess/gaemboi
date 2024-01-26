pub struct Core {
    enabled: bool,
    envelope_enabled: bool,
    envelope_sequence: u8,
    convert: bool,
    // NRx1
    length_timer: u8,
    wave_duty: u8,
    // NRx2
    pace: u8,
    direction: bool,
    volume: u8,
}

impl Core {
    pub fn new() -> Self {
        Self {
            enabled: false,
            envelope_enabled: false,
            envelope_sequence: 0,
            convert: false,
            length_timer: 0,
            wave_duty: 0,
            pace: 0,
            direction: true,
            volume: 0,
        }
    }

    pub fn get_length_timer(&self) -> u8 {
        (self.wave_duty & 0x03) << 6 | self.length_timer & 0x3F
    }

    pub fn set_length_timer(&mut self, value: u8) {
        self.wave_duty = (value & 0xC0) >> 6;
        self.length_timer = value & 0x3F;
    }

    pub fn get_volume_envelope(&self) -> u8 {
        (self.pace & 0x07) | if self.direction { 0x08 } else { 0x00 } | (self.volume & 0x0F) << 4
    }

    pub fn set_volume_envelope(&mut self, value: u8) {
        self.pace = value & 0x07;
        self.direction = value & 0x08 != 0;
        self.volume = (value & 0xF0) >> 4;
        self.envelope_enabled = self.pace > 0;
        self.envelope_sequence = 0;
        self.convert = value & 0xF8 != 0x00;

        if !self.convert {
            self.enabled = false;
        }
    }
}
