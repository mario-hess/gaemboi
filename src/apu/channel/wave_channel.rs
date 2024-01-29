use crate::apu::{CH3_END, CH3_START, LENGTH_TIMER_MAX};

const CONVERT_ENABLE: u16 = CH3_START; // NR30
const LENGTH_TIMER: u16 = 0xFF1B; // NR31
const OUTPUT_LEVEL: u16 = 0xFF1C; // NR32
const PERIOD_LOW: u16 = 0xFF1D; // NR33
const PERIOD_HIGH: u16 = CH3_END; // NR34

pub const WAVE_PATTERN_START: u16 = 0xFF30;
pub const WAVE_PATTERN_END: u16 = 0xFF3F;

pub struct WaveChannel {
    pub enabled: bool,
    timer: i16,
    pub output: u8,
    pub volume: u8,
    position: u8,
    period: u16,

    // NR30
    convert: bool,

    // NR31
    length_timer: u16,

    // NR32
    output_level: u8,

    // NR34
    length_enable: bool,
    triggered: bool,

    wave_ram: [u8; 32],
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
            output: 0,
            volume: 0,
            position: 0,
            period: 0,
            convert: false,
            length_timer: 0,
            output_level: 0,
            length_enable: false,
            triggered: false,
            wave_ram: [0; 32],
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;
        self.timer = self.timer.saturating_sub(t_cycles as i16);

        if self.timer > 0 {
            return;
        }

        if self.enabled && self.convert {
            let wave_index = self.position >> 1;
            let mut output = self.wave_ram[wave_index as usize];
            output = if self.position & 0x01 == 0x01 {
                output & 0x0F
            } else {
                (output & 0xF0) >> 4
            };

            if self.output_level > 0 {
                output >>= self.output_level - 1;
            } else {
                output = 0;
            }

            self.output = output;
        } else {
            self.output = 0;
        }

        self.timer += ((2048 - self.period) << 1) as i16;
        self.position = (self.position + 1) & 0x1F;
    }

    pub fn tick_length_timer(&mut self) {
        if !self.length_enable || self.length_timer >= 256 {
            return;
        }

        self.length_timer = self.length_timer.saturating_add(1);
        if self.length_timer >= 256 {
            self.enabled = false;
        }
    }

    pub fn trigger(&mut self, sequencer_tick: &mut u8) {
        self.timer = 3;
        self.position = 0;

        if self.length_timer >= 256 {
            self.length_timer = 0;

            if self.length_enable && *sequencer_tick % 2 == 1 {
                self.tick_length_timer();
            }
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CONVERT_ENABLE => self.get_convert(),
            LENGTH_TIMER => self.length_timer as u8,
            OUTPUT_LEVEL => self.get_output_level(),
            PERIOD_LOW => self.get_period_low(),
            PERIOD_HIGH => self.get_period_high(),
            _ => {
                eprintln!("Unknown address: {:#X} Can't read byte.", address);
                0xFF
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8, sequencer_tick: &mut u8) {
        match address {
            CONVERT_ENABLE => self.set_convert(value),
            LENGTH_TIMER => self.length_timer = value as u16,
            OUTPUT_LEVEL => self.set_output_level(value),
            PERIOD_LOW => self.set_period_low(value),
            PERIOD_HIGH => self.set_period_high(value, sequencer_tick),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn get_convert(&self) -> u8 {
        if self.convert {
            0x80
        } else {
            0x00
        }
    }

    fn set_convert(&mut self, value: u8) {
        self.convert = value & 0x80 != 0;

        // Setting bit 7 of this register to 0 turns the converter off (and thus, the channel as well)
        if !self.convert {
            self.enabled = false;
        }
    }

    fn get_output_level(&self) -> u8 {
        (self.output_level & 0x03) << 5
    }

    fn set_output_level(&mut self, value: u8) {
        let value = (value & 0x60) >> 5;
        self.output_level = value;

        /*
        match value {
            0x00 => self.volume = 0,
            0x01 => self.volume = 100,
            0x02 => self.volume = 50,
            0x03 => self.volume = 25,
            _ => unreachable!(),
        }
        */
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

        if length_enable && self.length_timer >= 256 {
            self.enabled = false;
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

    pub fn reset(&mut self) {
        self.enabled = false;
        self.timer = 0;
        self.output = 0;
        self.volume = 0;
        self.position = 0;
        self.period = 0;
        self.convert = false;
        self.length_timer = 0;
        self.output_level = 0;
        self.length_enable = false;
        self.triggered = false;
        self.wave_ram = [0; 32];
    }
}
