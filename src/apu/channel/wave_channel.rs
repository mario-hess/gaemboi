/*
 * @file    apu/channel/wave_channel.rs
 * @brief   Implementation of the wave channel (Channel 3).
 * @author  Mario Hess
 * @date    May 28, 2024
 */

use crate::apu::{
    channel::{core::ChannelCore, length_counter::LengthCounter},
    MemoryAccess, CH3_END, CH3_START,
};

use super::square_channel::ChannelType;

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
    pub length_counter: LengthCounter,
    pub volume: u8,
    pub frequency: u16,
    pub wave_ram: [u8; 32],
    pub wave_ram_position: u8,
}

impl MemoryAccess for WaveChannel {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            DAC_ENABLE => self.get_dac_enable(),
            LENGTH_TIMER => self.length_counter.timer as u8,
            VOLUME => self.get_output_level(),
            FREQUENCY_LOW => self.get_frequency_low(),
            FREQUENCY_HIGH => self.get_frequency_high(),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            DAC_ENABLE => self.set_dac_enable(value),
            LENGTH_TIMER => self.length_counter.timer = LENGTH_TIMER_MAX - (value as u16),
            VOLUME => self.set_output_level(value),
            FREQUENCY_LOW => self.set_frequency_low(value),
            FREQUENCY_HIGH => self.set_frequency_high(value),
            _ => unreachable!(),
        }
    }
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            core: ChannelCore::default(),
            length_counter: LengthCounter::default(),
            volume: 0,
            frequency: 0,
            wave_ram: [0; 32],
            wave_ram_position: 0,
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

        let wave_index = self.wave_ram_position / 2;
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
        self.wave_ram_position = 0;

        if self.length_counter.timer == 0 {
            self.length_counter.timer = LENGTH_TIMER_MAX;
        }
    }

    fn get_dac_enable(&self) -> u8 {
        if self.core.dac_enabled {
            0x80
        } else {
            0x00
        }
    }

    fn set_dac_enable(&mut self, value: u8) {
        self.core.dac_enabled = value & 0x80 != 0;

        if !self.core.dac_enabled {
            self.core.enabled = false;
        }
    }

    fn get_output_level(&self) -> u8 {
        (self.volume & 0x03) << 5
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

    fn get_frequency_low(&self) -> u8 {
        self.frequency as u8
    }

    fn set_frequency_low(&mut self, value: u8) {
        self.frequency = (self.frequency & 0x0700) | value as u16;
    }

    fn get_frequency_high(&self) -> u8 {
        let frequency_high = ((self.frequency & 0x0700) >> 8) as u8;
        let length_enabled = if self.length_counter.enabled {
            0x40
        } else {
            0x00
        };
        let triggered = if self.core.triggered { 0x80 } else { 0x00 };

        frequency_high | length_enabled | triggered
    }

    fn set_frequency_high(&mut self, value: u8) {
        let triggered = value & 0x80 != 0;
        if triggered {
            self.trigger();
        }

        self.length_counter.enabled = value & 0x40 != 0;
        self.frequency = (self.frequency & 0x00FF) | ((value & 0x07) as u16) << 8;
    }

    pub fn read_wave_ram(&self, address: u16) -> u8 {
        let index = address - WAVE_PATTERN_START;
        self.wave_ram[index as usize]
    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8) {
        let index = address - WAVE_PATTERN_START;
        self.wave_ram[index as usize] = (value & 0xF0) >> 4;
        self.wave_ram[index as usize + 1] = value & 0xF;
    }

    pub fn reset(&mut self, channel: ChannelType) {
        self.core.reset();
        self.length_counter.reset(channel);
        self.volume = 0;
        self.wave_ram_position = 0;
        self.frequency = 0;
        self.wave_ram = [0; 32];
    }
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self::new()
    }
}
