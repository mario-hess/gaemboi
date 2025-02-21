/*
 * @file    apu/channel/square_channel.rs
 * @brief   Implementation of the square channels (Channel 1 & 2).
 * @author  Mario Hess
 * @date    May 27, 2024
 */

use crate::apu::{
    channel::{
        core::ChannelCore, sweep::Sweep,
        volume_envelope::VolumeEnvelope,
    },
    ComponentTick, MemoryAccess, LENGTH_TIMER_MAX,
};

const SWEEP: u16 = 0;
const LENGTH_TIMER: u16 = 1;
const VOLUME_ENVELOPE: u16 = 2;
const FREQUENCY_LOW: u16 = 3;
const FREQUENCY_HIGH: u16 = 4;

/* https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware
Duty   Waveform    Ratio
-------------------------
0      00000001    12.5%
1      10000001    25%
2      10000111    50%
3      01111110    75%
*/
pub const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(PartialEq)]
pub enum ChannelType {
    CH1,
    CH2,
    CH3,
    CH4,
}

pub struct SquareChannel {
    pub core: ChannelCore,
    pub volume_envelope: VolumeEnvelope,
    pub sweep: Option<Sweep>,
    pub sequence: u8,
    pub frequency: u16,
    pub wave_duty: u8,
}

impl MemoryAccess for SquareChannel {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            SWEEP => {
                if let Some(sweep) = &self.sweep {
                    sweep.get()
                } else {
                    0xFF
                }
            }
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.volume_envelope.get(),
            FREQUENCY_LOW => self.get_frequency_low(),
            FREQUENCY_HIGH => self.get_frequency_high(),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            SWEEP => {
                if let Some(sweep) = &mut self.sweep {
                    sweep.set(value);
                }
            }
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            FREQUENCY_LOW => self.set_frequency_low(value),
            FREQUENCY_HIGH => self.set_frequency_high(value),
            _ => unreachable!(),
        }
    }
}

impl ComponentTick for SquareChannel {
    fn tick(&mut self, m_cycles: u8) {
        if !self.core.enabled || !self.core.dac_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;

        self.core.timer = self.core.timer.saturating_sub(t_cycles as i32);
        if self.core.timer > 0 {
            return;
        }

        self.core.output = if DUTY_TABLE[self.wave_duty as usize][self.sequence as usize] == 1 {
            self.volume_envelope.volume
        } else {
            0
        };

        self.core.timer += ((2048 - self.frequency) * 4) as i32;
        self.sequence = (self.sequence + 1) & 0x07;
    }
}

impl SquareChannel {
    pub fn new(channel_type: ChannelType) -> Self {
        let sweep_enabled = match channel_type {
            ChannelType::CH1 => true,
            ChannelType::CH2 => false,
            _ => false,
        };

        let sweep = if sweep_enabled {
            Some(Sweep::default())
        } else {
            None
        };

        let wave_duty = if sweep_enabled { 0x02 } else { 0x0 };
        let volume_envelope = if sweep_enabled { 0xF3 } else { 0x00 };

        Self {
            core: ChannelCore::new(sweep_enabled),
            volume_envelope: VolumeEnvelope::new(volume_envelope),
            sweep,
            sequence: 0,
            frequency: 0,
            wave_duty,
        }
    }

    pub fn tick_sweep(&mut self) {
        if let Some(sweep) = &mut self.sweep {
            sweep.tick(&mut self.frequency, &mut self.core.enabled);
        }
    }

    pub fn trigger(&mut self) {
        if self.core.dac_enabled {
            self.core.enabled = true;
        }

        self.core.timer = ((2048 - self.frequency) * 4) as i32;
        self.volume_envelope.counter = 0;

        if let Some(sweep) = &mut self.sweep {
            sweep.sequence = 0;
        }

        if self.core.length_counter.timer == 0 {
            self.core.length_counter.timer = LENGTH_TIMER_MAX;
        }
    }

    fn get_length_timer(&self) -> u8 {
        let wave_duty = (self.wave_duty & 0x03) << 6;
        let length_timer = (self.core.length_counter.timer & 0x3F) as u8;

        0x3F | wave_duty | length_timer
    }

    fn set_length_timer(&mut self, value: u8) {
        self.wave_duty = (value & 0xC0) >> 6;
        self.core.length_counter.timer = LENGTH_TIMER_MAX - (value & 0x3F) as u16;
    }

    fn set_volume_envelope(&mut self, value: u8) {
        self.volume_envelope.set(value);

        // https://gbdev.io/pandocs/Audio_details.html#dacs
        // Channel x’s DAC is enabled if and only if [NRx2] & 0xF8 != 0;
        self.core.dac_enabled = value & 0xF8 != 0x00;
        if !self.core.dac_enabled {
            self.core.enabled = false;
        }

        // Writes to this register while the channel is on
        // require retriggering it afterwards. If the write
        // turns the channel off, retriggering is not necessary
        if self.core.enabled {
            self.trigger();
        }
    }

    fn get_frequency_low(&self) -> u8 {
        0xFF
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

    pub fn reset(&mut self, channel: ChannelType) {
        self.core.reset();
        self.core.length_counter.reset(channel);
        self.volume_envelope.reset();
        self.sequence = 0;
        self.frequency = 0;
        self.wave_duty = 0;

        if let Some(sweep) = &mut self.sweep {
            sweep.reset();
        }
    }
}

#[cfg(test)]
mod ch1_length_timer_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch1 = SquareChannel::new(ChannelType::CH1);

        assert_eq!(ch1.get_length_timer(), 0xBF);
    }

    #[test]
    fn empty_fill() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x00;
        ch1.set_length_timer(value);

        assert_eq!(ch1.get_length_timer(), 0x3F);
    }

    #[test]
    fn saturate_all() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0xFF;
        ch1.set_length_timer(value);

        assert_eq!(ch1.get_length_timer(), 0xFF)
    }

    #[test]
    fn saturate_length_timer() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x3F;
        ch1.set_length_timer(value);

        assert_eq!(ch1.get_length_timer(), 0x3F);
    }

    #[test]
    fn saturate_duty_cycle() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0xC0;
        ch1.set_length_timer(value);

        assert_eq!(ch1.get_length_timer(), 0xFF);
    }
}

#[cfg(test)]
mod ch2_length_timer_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch2 = SquareChannel::new(ChannelType::CH2);

        assert_eq!(ch2.get_length_timer(), 0x3F);
    }

    #[test]
    fn empty_fill() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x00;
        ch2.set_length_timer(value);

        assert_eq!(ch2.get_length_timer(), 0x3F);
    }

    #[test]
    fn saturate_all() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0xFF;
        ch2.set_length_timer(value);

        assert_eq!(ch2.get_length_timer(), 0xFF)
    }

    #[test]
    fn saturate_length_timer() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x3F;
        ch2.set_length_timer(value);

        assert_eq!(ch2.get_length_timer(), 0x3F);
    }

    #[test]
    fn saturate_duty_cycle() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0xC0;
        ch2.set_length_timer(value);

        assert_eq!(ch2.get_length_timer(), 0xFF);
    }
}

#[cfg(test)]
mod ch1_volume_envelope_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch1 = SquareChannel::new(ChannelType::CH1);

        assert_eq!(ch1.volume_envelope.get(), 0xF3);
    }
}

#[cfg(test)]
mod ch2_volume_envelope_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch1 = SquareChannel::new(ChannelType::CH2);

        assert_eq!(ch1.volume_envelope.get(), 0x00);
    }
}

#[cfg(test)]
mod ch1_frequency_low_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch1 = SquareChannel::new(ChannelType::CH1);

        assert_eq!(ch1.get_frequency_low(), 0xFF);
    }

    #[test]
    fn empty_fill() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x00;
        ch1.set_frequency_low(value);

        assert_eq!(ch1.get_frequency_low(), 0xFF);
    }

    #[test]
    fn saturate_all() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0xFF;
        ch1.set_frequency_low(value);

        assert_eq!(ch1.get_frequency_low(), 0xFF);
    }
}

#[cfg(test)]
mod ch2_frequency_low_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch2 = SquareChannel::new(ChannelType::CH2);

        assert_eq!(ch2.get_frequency_low(), 0xFF);
    }

    #[test]
    fn empty_fill() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x00;
        ch2.set_frequency_low(value);

        assert_eq!(ch2.get_frequency_low(), 0xFF);
    }

    #[test]
    fn saturate_all() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0xFF;
        ch2.set_frequency_low(value);

        assert_eq!(ch2.get_frequency_low(), 0xFF);
    }
}

#[cfg(test)]
mod ch1_frequency_high_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch1 = SquareChannel::new(ChannelType::CH1);

        assert_eq!(ch1.get_frequency_high(), 0xBF);
    }

    #[test]
    fn empty_fill() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x00;
        ch1.set_frequency_high(value);

        assert_eq!(ch1.get_frequency_high(), 0xBF);
    }

    #[test]
    fn saturate_all() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0xFF;
        ch1.set_frequency_high(value);

        assert_eq!(ch1.get_frequency_high(), 0xFF);
    }

    #[test]
    fn saturate_frequency_high() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x07;
        ch1.set_frequency_high(value);

        assert_eq!(ch1.get_frequency_high(), 0xBF);
    }

    #[test]
    fn saturate_length_enabled() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x40;
        ch1.set_frequency_high(value);

        assert_eq!(ch1.get_frequency_high(), 0xFF);
    }

    #[test]
    fn saturate_trigger() {
        let mut ch1 = SquareChannel::new(ChannelType::CH1);

        let value = 0x80;
        ch1.set_frequency_high(value);

        assert_eq!(ch1.get_frequency_high(), 0xBF);
    }
}

#[cfg(test)]
mod ch2_frequency_high_tests {
    use super::*;

    #[test]
    fn default_values() {
        let ch2 = SquareChannel::new(ChannelType::CH2);

        assert_eq!(ch2.get_frequency_high(), 0xBF);
    }

    #[test]
    fn empty_fill() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x00;
        ch2.set_frequency_high(value);

        assert_eq!(ch2.get_frequency_high(), 0xBF);
    }

    #[test]
    fn saturate_all() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0xFF;
        ch2.set_frequency_high(value);

        assert_eq!(ch2.get_frequency_high(), 0xFF);
    }

    #[test]
    fn saturate_frequency_high() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x07;
        ch2.set_frequency_high(value);

        assert_eq!(ch2.get_frequency_high(), 0xBF);
    }

    #[test]
    fn saturate_length_enabled() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x40;
        ch2.set_frequency_high(value);

        assert_eq!(ch2.get_frequency_high(), 0xFF);
    }

    #[test]
    fn saturate_trigger() {
        let mut ch2 = SquareChannel::new(ChannelType::CH2);

        let value = 0x80;
        ch2.set_frequency_high(value);

        assert_eq!(ch2.get_frequency_high(), 0xBF);
    }
}
