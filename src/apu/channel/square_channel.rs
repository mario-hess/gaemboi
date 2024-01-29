use crate::apu::LENGTH_TIMER_MAX;

const SWEEP: u16 = 0;
const LENGTH_TIMER: u16 = 1;
const VOLUME_ENVELOPE: u16 = 2;
const PERIOD_LOW: u16 = 3;
const PERIOD_HIGH: u16 = 4;

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

pub enum ChannelType {
    CH1,
    CH2,
}

pub struct SquareChannel {
    pub enabled: bool,
    timer: i16,
    sequence: u8,
    pub output: u8,
    convert: bool,
    period: u16,
    envelope_enabled: bool,
    envelope_sequence: u8,

    // NRx0
    sweep_sequence: Option<u8>,
    sweep_shift: Option<u8>,
    sweep_direction: Option<bool>,
    sweep_pace: Option<u8>,

    // NRx1
    length_timer: u8,
    wave_duty: u8,

    // NRx2
    pace: u8,
    direction: bool,
    pub volume: u8,

    // NRx4
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
        let sweep_shift = if sweep_enabled { Some(0) } else { None };
        let sweep_direction = if sweep_enabled { Some(true) } else { None };
        let sweep_pace = if sweep_enabled { Some(0) } else { None };

        Self {
            enabled: false,
            timer: 0,
            sequence: 0,
            output: 0,
            convert: false,
            period: 0,
            envelope_enabled: false,
            envelope_sequence: 0,
            sweep_sequence,
            sweep_shift,
            sweep_direction,
            sweep_pace,
            length_timer: 0,
            wave_duty: 0,
            pace: 0,
            direction: true,
            volume: 0,
            length_enable: false,
            triggered: false,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;
        self.timer = self.timer.saturating_sub(t_cycles as i16);

        if self.timer > 0 {
            return;
        }

        if self.enabled {
            self.output = if DUTY_TABLE[self.wave_duty as usize][self.sequence as usize] == 1 {
                self.volume
            } else {
                0
            };
        } else {
            self.output = 0;
        }

        self.timer += ((2048 - self.period) << 2) as i16;
        self.sequence = (self.sequence + 1) & 0x07;
    }

    pub fn tick_sweep(&mut self) {
        if self.sweep_pace == Some(0) {
            return;
        }

        if let Some(value) = self.sweep_sequence {
            self.sweep_sequence = Some(value + 1);
        }

        if self.sweep_sequence.unwrap() >= self.sweep_pace.unwrap() {
            let delta = self.period >> self.sweep_shift.unwrap();

            self.period = if self.sweep_direction.unwrap() {
                self.period.saturating_add(delta)
            } else {
                self.period.saturating_sub(delta)
            };

            // Overflow check
            if self.period > 0x07FF {
                self.enabled = false;
                self.period = 0x07FF;
            }

            self.sweep_sequence = Some(0);
        }
    }

    pub fn tick_length_timer(&mut self) {
        if !self.length_enable || self.length_timer >= LENGTH_TIMER_MAX {
            return;
        }

        self.length_timer = self.length_timer.saturating_add(1);
        if self.length_timer >= LENGTH_TIMER_MAX {
            self.enabled = false;
        }
    }

    pub fn tick_envelope(&mut self) {
        if !self.enabled || !self.envelope_enabled {
            return;
        }

        self.envelope_sequence += 1;

        if self.envelope_sequence >= self.pace {
            self.volume = if self.direction {
                self.volume.saturating_add(1)
            } else {
                self.volume.saturating_sub(1)
            };

            if self.volume == 0 || self.volume == 15 {
                self.envelope_enabled = false;
            }

            self.envelope_sequence = 0;
        }
    }

    pub fn trigger(&mut self, sequencer_tick: &mut u8) {
        self.timer = ((2048 - self.period) << 2) as i16;
        self.envelope_sequence = 0;

        if self.sweep_sequence.is_some() {
            self.sweep_sequence = Some(0);
        }

        if self.length_timer >= LENGTH_TIMER_MAX {
            self.length_timer = 0;
            if self.length_enable && *sequencer_tick % 2 == 1 {
                self.tick_length_timer();
            }
        }
    }

    pub fn read_byte(&self, base_address: u16, address: u16) -> u8 {
        let address = if address < 0xFF16 {address - base_address} else {(address - base_address) + 1};

        match address {
            SWEEP => self.get_sweep(),
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.get_volume_envelope(),
            PERIOD_LOW => self.get_period_low(),
            PERIOD_HIGH => self.get_period_high(),
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
                0xFF
            }
        }
    }

    pub fn write_byte(
        &mut self,
        base_address: u16,
        address: u16,
        value: u8,
        sequencer_tick: &mut u8,
    ) {
        let address = if address < 0xFF16 {address - base_address} else {(address - base_address) + 1};

        match address {
            SWEEP => self.set_sweep(value),
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            PERIOD_LOW => self.set_period_low(value),
            PERIOD_HIGH => self.set_period_high(value, sequencer_tick),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn get_sweep(&self) -> u8 {
        let step = self.sweep_shift.unwrap() & 0x07;
        let direction = if self.sweep_direction.unwrap() {
            0x08
        } else {
            0x0
        };
        let pace = (self.sweep_pace.unwrap() & 0x07) << 4;

        step | direction | pace | 0x80
    }

    fn set_sweep(&mut self, value: u8) {
        self.sweep_shift = Some(value & 0x07);
        self.sweep_direction = Some((value & 0x08) == 0x00);
        self.sweep_pace = Some((value & 0x70) >> 4);
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
        self.convert = value & 0xF8 != 0x00;
        if !self.convert {
            self.enabled = false;
        }
    }

    fn get_period_low(&self) -> u8 {
        self.period as u8
    }

    fn set_period_low(&mut self, value: u8) {
        self.period = (self.period & 0x0700) | value as u16;
    }

    fn get_period_high(&self) -> u8 {
        let period_high = ((self.period & 0x0700) >> 8) as u8;
        let length_enable = if self.length_enable { 0x40 } else { 0x00 };
        let triggered = if self.triggered { 0x80 } else { 0x00 };

        period_high | length_enable | triggered
    }

    fn set_period_high(&mut self, value: u8, sequencer_tick: &mut u8) {
        let length_enable = value & 0x40 != 0;
        let triggered = value & 0x80 != 0;
        let length_edge = length_enable && !self.length_enable;
        self.period = (self.period & 0x00FF) | ((value & 0x07) as u16) << 8;
        self.length_enable = length_enable;
        self.enabled |= triggered;

        if length_edge && *sequencer_tick % 2 == 1 {
            self.tick_length_timer();
        }

        if triggered {
            self.trigger(sequencer_tick);
        }

        if length_enable && self.length_timer >= LENGTH_TIMER_MAX {
            self.enabled = false;
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.timer = 0;
        self.sequence = 0;
        self.output = 0;
        self.convert = false;
        self.period = 0;
        self.envelope_enabled = false;
        self.envelope_sequence = 0;
        self.sweep_sequence = Some(0);
        self.sweep_shift = Some(0);
        self.sweep_direction = Some(true);
        self.sweep_pace = Some(0);
        self.length_timer = 0;
        self.wave_duty = 0;
        self.pace = 0;
        self.direction = true;
        self.volume = 0;
        self.length_enable = false;
        self.triggered = false;
    }
}
