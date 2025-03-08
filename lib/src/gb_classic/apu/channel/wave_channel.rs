use crate::gb_classic::apu::{channel::core::ChannelCore, ChannelType, CH3_END, CH3_START};

const DAC_ENABLE: u16 = CH3_START; // NR30
const LENGTH_TIMER: u16 = 0xFF1B; // NR31
const VOLUME: u16 = 0xFF1C; // NR32
const FREQUENCY_LOW: u16 = 0xFF1D; // NR33
const FREQUENCY_HIGH: u16 = CH3_END; // NR34

pub const WAVE_PATTERN_START: u16 = 0xFF30;
pub const WAVE_PATTERN_END: u16 = 0xFF3F;

const LENGTH_TIMER_MAX: u16 = 256;

pub struct WaveChannel {
    pub core: ChannelCore,
    pub volume: u8,
    pub frequency: u16,
    pub wave_ram: [u8; 32],
    pub wave_ram_position: u8,
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            core: ChannelCore::default(),
            volume: 0,
            frequency: 0,
            wave_ram: [
                0x8, 0x4, 0x4, 0x0, 0x4, 0x3, 0xA, 0xA, 0x2, 0xD, 0x7, 0x8, 0x9, 0x2, 0x3, 0xC,
                0x6, 0x0, 0x5, 0x9, 0x5, 0x9, 0xB, 0x0, 0x3, 0x4, 0xB, 0x8, 0x2, 0xE, 0xD, 0xA,
            ],
            wave_ram_position: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            DAC_ENABLE => self.get_dac_enable(),
            LENGTH_TIMER => 0xFF,
            VOLUME => self.get_output_level(),
            FREQUENCY_LOW => 0xFF,
            FREQUENCY_HIGH => self.get_frequency_high(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            DAC_ENABLE => self.set_dac_enable(value),
            LENGTH_TIMER => self.core.length_counter.timer = LENGTH_TIMER_MAX - (value as u16),
            VOLUME => self.set_output_level(value),
            FREQUENCY_LOW => self.set_frequency_low(value),
            FREQUENCY_HIGH => self.set_frequency_high(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        if !self.core.enabled || !self.core.dac_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;

        self.core.timer = self.core.timer.saturating_sub(t_cycles as i32);
        if self.core.timer > 0 {
            return;
        }

        let wave_index = self.wave_ram_position;
        let output = self.wave_ram[wave_index as usize];

        self.core.output = output >> self.volume_shift();

        self.core.timer += ((2048 - self.frequency) * 2) as i32;
        self.wave_ram_position = (self.wave_ram_position + 1) & 0x1F;
    }

    pub fn trigger(&mut self) {
        if self.core.dac_enabled {
            self.core.enabled = true;
        }

        self.core.timer = ((2048 - self.frequency) * 2) as i32;
        self.wave_ram_position = 1;

        if self.core.length_counter.timer == 0 {
            self.core.length_counter.timer = LENGTH_TIMER_MAX;
        }
    }

    fn get_dac_enable(&self) -> u8 {
        if self.core.dac_enabled {
            0xFF
        } else {
            0x7F
        }
    }

    fn set_dac_enable(&mut self, value: u8) {
        self.core.dac_enabled = value & 0x80 != 0;

        if !self.core.dac_enabled {
            self.core.enabled = false;
        }
    }

    fn get_output_level(&self) -> u8 {
        0x9F | (self.volume & 0x03) << 5
    }

    fn set_output_level(&mut self, value: u8) {
        self.volume = (value & 0x60) >> 5;
    }

    fn volume_shift(&self) -> u8 {
        match self.volume {
            0x00 => 4, // 0%
            0x01 => 0, // 100%
            0x02 => 1, // 50%
            0x03 => 2, // 25%
            _ => unreachable!(),
        }
    }

    fn set_frequency_low(&mut self, value: u8) {
        self.frequency = (self.frequency & 0x0700) | value as u16;
    }

    fn get_frequency_high(&self) -> u8 {
        let frequency_high = ((self.frequency & 0x0700) >> 8) as u8;
        let length_enabled = if self.core.length_counter.enabled {
            0x40
        } else {
            0x00
        };
        let triggered = if self.core.triggered { 0x80 } else { 0x00 };

        0xBF | frequency_high | length_enabled | triggered
    }

    fn set_frequency_high(&mut self, value: u8) {
        let triggered = value & 0x80 != 0;
        if triggered {
            self.trigger();
        }

        self.core.length_counter.enabled = value & 0x40 != 0;
        self.frequency = (self.frequency & 0x00FF) | ((value & 0x07) as u16) << 8;
    }

    pub fn read_wave_ram(&self, address: u16) -> u8 {
        let index = (address - WAVE_PATTERN_START) * 2;
        let upper_nibble = (self.wave_ram[index as usize] & 0xF) << 4;
        let lower_nibble = self.wave_ram[index as usize + 1] & 0xF;

        upper_nibble | lower_nibble
    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8) {
        let index = (address - WAVE_PATTERN_START) * 2;
        self.wave_ram[index as usize] = (value & 0xF0) >> 4;
        self.wave_ram[index as usize + 1] = value & 0xF;
    }

    pub fn reset(&mut self, channel: ChannelType) {
        self.core.reset();
        self.core.length_counter.reset(channel);
        self.volume = 0;
        self.frequency = 0;
    }
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self::new()
    }
}
