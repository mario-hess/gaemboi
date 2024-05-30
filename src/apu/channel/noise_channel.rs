/**
 * @file    apu/channel/noise_channel.rs
 * @brief   Implementation of the noise channel (Channel 4).
 * @author  Mario Hess
 * @date    May 28, 2024
 */
use crate::apu::{
    channel::{core::ChannelCore, length_counter::LengthCounter, volume_envelope::VolumeEnvelope},
    ComponentTick, MemoryAccess, CH4_END, CH4_START, LENGTH_TIMER_MAX,
};

const LENGTH_TIMER: u16 = CH4_START; // NR41
const VOLUME_ENVELOPE: u16 = 0xFF21; // NR42
const FREQUENCY_RANDOMNESS: u16 = 0xFF22; // NR43
const CONTROL: u16 = CH4_END; // NR44

const DIVISORS: [u8; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

pub struct NoiseChannel {
    pub core: ChannelCore,
    pub length_counter: LengthCounter,
    pub volume_envelope: VolumeEnvelope,
    lfsr: u16,
    clock_divider: u8,
    lfsr_width: bool,
    clock_shift: u8,
}

impl MemoryAccess for NoiseChannel {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.volume_envelope.get(),
            FREQUENCY_RANDOMNESS => self.get_frequency_randomness(),
            CONTROL => self.get_control(),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            FREQUENCY_RANDOMNESS => self.set_frequency_randomness(value),
            CONTROL => self.set_control(value),
            _ => unreachable!(),
        }
    }
}

impl ComponentTick for NoiseChannel {
    fn tick(&mut self, m_cycles: u8) {
        if !self.core.enabled || !self.core.dac_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;

        self.core.timer = self.core.timer.saturating_sub(t_cycles as i16);
        if self.core.timer > 0 {
            return;
        }

        let result = ((self.lfsr & 0x01) ^ ((self.lfsr >> 1) & 0x01)) != 0;

        self.lfsr >>= 1;
        self.lfsr |= if result { 0x01 << 14 } else { 0x00 };

        if self.lfsr_width {
            self.lfsr &= 0xBF;
            self.lfsr |= if result { 0x40 } else { 0x00 };
        }

        self.core.output = if result {
            self.volume_envelope.volume
        } else {
            0x00
        };

        self.core.timer += ((DIVISORS[self.clock_divider as usize] as u16) << self.clock_shift) as i16;
    }
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            core: ChannelCore::default(),
            length_counter: LengthCounter::default(),
            volume_envelope: VolumeEnvelope::default(),
            lfsr: 0,
            clock_divider: 0,
            lfsr_width: false,
            clock_shift: 0,
        }
    }

    pub fn trigger(&mut self) {
        if self.core.dac_enabled {
            self.core.enabled = true;
        }

        self.core.timer = ((DIVISORS[self.clock_divider as usize] as u16) << self.clock_shift) as i16;
        self.lfsr = 0x7FF1;
        self.volume_envelope.counter = 0;

        if self.length_counter.timer == 0 {
            self.length_counter.timer = LENGTH_TIMER_MAX;
        }
    }

    fn get_length_timer(&self) -> u8 {
        (self.length_counter.timer & 0x3F) as u8
    }

    fn set_length_timer(&mut self, value: u8) {
        self.length_counter.timer = LENGTH_TIMER_MAX - (value & 0x3F) as u16;
    }

    fn set_volume_envelope(&mut self, value: u8) {
        self.volume_envelope.set(value);

        self.core.dac_enabled = value & 0xF8 != 0x00;
        if !self.core.dac_enabled {
            self.core.enabled = false;
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
        let length_enabled = if self.length_counter.enabled {
            0x40
        } else {
            0x00
        };
        let triggered = if self.core.triggered { 0x80 } else { 0x00 };

        length_enabled | triggered
    }

    fn set_control(&mut self, value: u8) {
        let triggered = value & 0x80 != 0;
        if triggered {
            self.trigger();
        }

        self.length_counter.enabled = value & 0x40 != 0;
    }

    pub fn reset(&mut self) {
        self.core.reset();
        self.length_counter.reset();
        self.volume_envelope.reset();
        self.lfsr = 0;
        self.clock_divider = 0;
        self.lfsr_width = false;
        self.clock_shift = 0;
    }
}

impl Default for NoiseChannel {
    fn default() -> Self {
        Self::new()
    }
}
