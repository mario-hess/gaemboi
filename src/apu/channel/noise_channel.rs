const LENGTH_TIMER: u16 = 0xFF20; // NR41
const VOLUME_ENVELOPE: u16 = 0xFF21; // NR42
const FREQUENCY_RANDOMNESS: u16 = 0xFF22; // NR43
const CONTROL: u16 = 0xFF23; // NR44

pub struct NoiseChannel {
    pub enabled: bool,
    convert: bool,
    envelope_enabled: bool,
    envelope_sequence: u8,

    // NR41
    length_timer: u8,

    // NR42
    pace: u8,
    direction: bool,
    volume: u8,

    // NR43
    clock_divider: u8,
    lfsr_width: bool,
    clock_shift: u8,

    // NR44
    length_enable: bool,
    triggered: bool,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            convert: false,
            envelope_enabled: false,
            envelope_sequence: 0,
            length_timer: 0,
            pace: 0,
            direction: true,
            volume: 0,
            clock_divider: 0,
            lfsr_width: false,
            clock_shift: 0,
            length_enable: false,
            triggered: false,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.get_volume_envelope(),
            FREQUENCY_RANDOMNESS => self.get_frequency_randomness(),
            CONTROL => self.get_control(),
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
                0xFF
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            FREQUENCY_RANDOMNESS => self.set_frequency_randomness(value),
            CONTROL => self.set_control(value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn get_length_timer(&self) -> u8 {
        self.length_timer & 0x3F
    }

    fn set_length_timer(&mut self, value: u8) {
        self.length_timer = value & 0x3F;
    }

    fn get_volume_envelope(&self) -> u8 {
        let pace = self.pace & 0x07;
        let direction = if self.direction { 0x08 } else { 0x00 };
        let volume = (self.volume & 0x0F) << 4;

        pace | direction | volume
    }

    fn set_volume_envelope(&mut self, value: u8) {
        self.pace = value & 0x07;
        self.direction = value & 0x08 != 0;
        self.volume = (value & 0xF0) >> 4;
        self.envelope_enabled = self.pace > 0;
        self.envelope_sequence = 0;

        // Setting bits 3-7 of this register all to 0 turns the converter off (and thus, the channel as well)
        self.convert = value & 0b11111000 != 0x00;
        if !self.convert {
            self.enabled = false;
        }
    }

    fn get_frequency_randomness(&self) -> u8 {
        let clock_divider = self.clock_divider & 0x07;
        let lfsr_width = if self.lfsr_width { 0x08 } else { 0x00 };
        let clock_shift = (self.clock_shift & 0x0F) << 4;

        clock_divider | lfsr_width | clock_shift
    }

    fn set_frequency_randomness(&mut self, value: u8) {
        self.clock_divider = value & 0x07;
        self.lfsr_width = value & 0x08 != 0;
        self.clock_shift = (value & 0xF0) >> 4;
    }

    fn get_control(&self) -> u8 {
        let length_enable = if self.length_enable { 0x40 } else { 0x00 };
        let triggered = if self.triggered { 0x80 } else { 0x00 };

        length_enable | triggered
    }

    fn set_control(&mut self, value: u8) {
        self.length_enable = value & 0x40 != 0;
        self.triggered = value & 0x80 != 0;

        // Triggering a channel causes it to turn on if it wasn’t
        if self.triggered {
            self.enabled = true;
        }
    }
}
