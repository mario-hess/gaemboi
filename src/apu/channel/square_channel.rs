/**
 * @file    apu/channel/square_channel.rs
 * @brief   Implementation of the square channels (Channel 1 & 2).
 * @author  Mario Hess
 * @date    May 27, 2024
 */
use crate::apu::{
    channel::length_counter::LengthCounter, channel::sweep::Sweep,
    channel::volume_envelope::VolumeEnvelope, MemoryAccess, LENGTH_TIMER_MAX,
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
    dac_enabled: bool,
    output: u8,
    timer: i16,
    triggered: bool,
    pub length_counter: LengthCounter,
    pub volume_envelope: VolumeEnvelope,
    pub sweep: Option<Sweep>,
    sequence: u8,
    pub frequency: u16,
    wave_duty: u8,
}

impl MemoryAccess for SquareChannel {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            SWEEP => {
                if let Some(sweep) = &self.sweep {
                    sweep.get()
                } else {
                    0x00
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

impl SquareChannel {
    pub fn new(channel_type: ChannelType) -> Self {
        let sweep_enabled = match channel_type {
            ChannelType::CH1 => true,
            ChannelType::CH2 => false,
        };

        let sweep = if sweep_enabled {
            Some(Sweep::default())
        } else {
            None
        };

        Self {
            enabled: false,
            dac_enabled: false,
            output: 0,
            timer: 0,
            triggered: false,
            length_counter: LengthCounter::default(),
            volume_envelope: VolumeEnvelope::default(),
            sweep,
            sequence: 0,
            frequency: 0,
            wave_duty: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        if !self.enabled || !self.dac_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;

        self.timer = self.timer.saturating_sub(t_cycles as i16);
        if self.timer > 0 {
            return;
        }

        self.output = if DUTY_TABLE[self.wave_duty as usize][self.sequence as usize] == 1 {
            self.volume_envelope.volume
        } else {
            0
        };

        self.timer += ((2048 - self.frequency) * 4) as i16;
        self.sequence = (self.sequence + 1) & 0x07;
    }

    pub fn tick_sweep(&mut self) {
        if let Some(sweep) = &mut self.sweep {
            sweep.tick(&mut self.frequency, &mut self.enabled);
        }
    }

    pub fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        self.timer = ((2048 - self.frequency) * 4) as i16;
        self.volume_envelope.counter = 0;

        if let Some(sweep) = &mut self.sweep {
            sweep.sequence = 0;
        }

        if self.length_counter.timer == 0 {
            self.length_counter.timer = LENGTH_TIMER_MAX;
        }
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled && self.dac_enabled {
            self.output
        } else {
            0
        }
    }

    fn get_length_timer(&self) -> u8 {
        let wave_duty = (self.wave_duty & 0x03) << 6;
        let length_timer = (self.length_counter.timer & 0x3F) as u8;

        wave_duty | length_timer
    }

    fn set_length_timer(&mut self, value: u8) {
        self.wave_duty = (value & 0xC0) >> 6;
        self.length_counter.timer = LENGTH_TIMER_MAX - (value & 0x3F) as u16;
    }

    fn set_volume_envelope(&mut self, value: u8) {
        self.volume_envelope.set(value);

        self.dac_enabled = value & 0xF8 != 0x00;
        if !self.dac_enabled {
            self.enabled = false;
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
        let triggered = if self.triggered { 0x80 } else { 0x00 };

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

    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_counter.reset();
        self.volume_envelope.reset();
        self.output = 0;
        self.timer = 0;
        self.sequence = 0;
        self.dac_enabled = false;
        self.frequency = 0;
        self.wave_duty = 0;
        self.triggered = false;

        if let Some(sweep) = &mut self.sweep {
            sweep.reset();
        }
    }
}
