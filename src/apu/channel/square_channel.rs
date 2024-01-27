const SWEEP: u16 = 0;
const LENGTH_TIMER: u16 = 1;
const VOLUME_ENVELOPE: u16 = 2;
const PERIOD_LOW: u16 = 3;
const PERIOD_HIGH: u16 = 4;

pub enum ChannelType {
    CH1,
    CH2,
}

pub struct SquareChannel {
    pub enabled: bool,
    convert: bool,
    envelope_enabled: bool,
    envelope_sequence: u8,

    // NRx0
    sweep_sequence: Option<u8>,
    sweep_step: Option<u8>,
    sweep_direction: Option<bool>,
    sweep_pace: Option<u8>,

    // NRx1
    length_timer: u8,
    wave_duty: u8,

    // NRx2
    pace: u8,
    direction: bool,
    volume: u8,

    // NRx3
    period_low: u8,

    // NRx4
    period_high: u8,
    length_enable: bool,
    triggered: bool,
}

impl SquareChannel {
    pub fn new(channel_type: ChannelType) -> Self {
        let sweep_enabled = match channel_type {
            ChannelType::CH1 => true,
            ChannelType::CH2 => false,
        };

        let sweep_sequence = if sweep_enabled { Some(0) } else { None };
        let sweep_step = if sweep_enabled { Some(0) } else { None };
        let sweep_direction = if sweep_enabled { Some(true) } else { None };
        let sweep_pace = if sweep_enabled { Some(0) } else { None };

        Self {
            enabled: false,
            convert: false,
            envelope_enabled: false,
            envelope_sequence: 0,
            sweep_sequence,
            sweep_step,
            sweep_direction,
            sweep_pace,
            length_timer: 0,
            wave_duty: 0,
            pace: 0,
            direction: true,
            volume: 0,
            period_low: 0,
            period_high: 0,
            length_enable: false,
            triggered: false,
        }
    }

    pub fn read_byte(&self, base_address: u16, address: u16) -> u8 {
        let address = address - base_address;

        match address {
            SWEEP => self.get_sweep(),
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.get_volume_envelope(),
            PERIOD_LOW => self.period_low,
            PERIOD_HIGH => self.get_period_high(),
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
                0xFF
            }
        }
    }

    pub fn write_byte(&mut self, base_address: u16, address: u16, value: u8) {
        let address = address - base_address;

        match address {
            SWEEP => self.set_sweep(value),
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            PERIOD_LOW => self.period_low = value,
            PERIOD_HIGH => self.set_period_high(value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn get_sweep(&self) -> u8 {
        let step = self.sweep_step.unwrap() & 0x07;
        let direction = if self.sweep_direction.unwrap() {
            0x08
        } else {
            0x0
        };
        let pace = (self.sweep_pace.unwrap() & 0x07) << 4;

        step | direction | pace | 0x80
    }

    fn set_sweep(&mut self, value: u8) {
        self.sweep_step = Some(value & 0x07);
        self.sweep_direction = Some(value & 0x08 == 0x00);
        self.sweep_pace = Some(value & 0x70);
        self.sweep_sequence = Some(0);
    }

    fn get_length_timer(&self) -> u8 {
        let wave_duty = (self.wave_duty & 0x03) << 6;
        let length_timer = self.length_timer & 0x3F;

        wave_duty | length_timer
    }

    fn set_length_timer(&mut self, value: u8) {
        self.wave_duty = (value & 0xC0) >> 6;
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

    fn get_period_high(&self) -> u8 {
        let period_high = self.period_high & 0x07;
        let length_enable = if self.length_enable { 0x40 } else { 0x00 };
        let triggered = if self.triggered { 0x80 } else { 0x00 };

        period_high | length_enable | triggered
    }

    fn set_period_high(&mut self, value: u8) {
        self.period_high = value & 0x07;
        self.length_enable = value & 0x40 != 0;
        self.triggered = value & 0x80 != 0;

        // Triggering a channel causes it to turn on if it wasn’t
        if self.triggered {
            self.enabled = true;
        }
    }
}
