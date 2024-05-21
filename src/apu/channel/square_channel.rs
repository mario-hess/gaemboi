/**
 * @file    apu/channel/square_channel.rs
 * @brief   Square channel.
 * @author  Mario Hess
 * @date    May 21, 2024
 */
use crate::apu::LENGTH_TIMER_MAX;

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
    output: u8,
    timer: i16,
    sequence: u8,
    dac_enabled: bool,
    frequency: u16,
    envelope_enabled: bool,
    envelope_sequence: u8,
    sweep_sequence: Option<u8>,
    sweep_shift: Option<u8>,
    sweep_direction: Option<bool>,
    sweep_pace: Option<u8>,
    length_timer: u8,
    wave_duty: u8,
    pace: u8,
    direction: bool,
    volume: u8,
    length_enabled: bool,
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
            output: 0,
            timer: 0,
            sequence: 0,
            dac_enabled: false,
            frequency: 0,
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
            length_enabled: false,
            triggered: false,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        if !self.enabled {
            return;
        }

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

        self.timer += ((2048 - self.frequency) * 4) as i16;
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
            let delta = self.frequency >> self.sweep_shift.unwrap();

            self.frequency = if self.sweep_direction.unwrap() {
                self.frequency.saturating_add(delta)
            } else {
                self.frequency.saturating_sub(delta)
            };

            // Overflow check
            if self.frequency > 0x07FF {
                self.enabled = false;
                self.frequency = 0x07FF;
            }

            self.sweep_sequence = Some(0);
        }
    }

    pub fn tick_length_timer(&mut self) {
        if !self.length_enabled || self.length_timer == 0 {
            return;
        }

        self.length_timer = self.length_timer.saturating_sub(1);
        if self.length_timer == 0 {
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

    pub fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        self.timer = ((2048 - self.frequency) * 4) as i16;
        self.envelope_sequence = 0;

        if self.sweep_sequence.is_some() {
            self.sweep_sequence = Some(0);
        }

        if self.length_timer == 0 {
            self.length_timer = LENGTH_TIMER_MAX;
        }
    }

    pub fn read_byte(&self, base_address: u16, address: u16) -> u8 {
        let address = if address < 0xFF16 {
            address - base_address
        } else {
            (address - base_address) + 1
        };

        match address {
            SWEEP => self.get_sweep(),
            LENGTH_TIMER => self.get_length_timer(),
            VOLUME_ENVELOPE => self.get_volume_envelope(),
            FREQUENCY_LOW => self.get_frequency_low(),
            FREQUENCY_HIGH => self.get_frequency_high(),
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
    ) {
        let address = if address < 0xFF16 {
            address - base_address
        } else {
            (address - base_address) + 1
        };

        match address {
            SWEEP => self.set_sweep(value),
            LENGTH_TIMER => self.set_length_timer(value),
            VOLUME_ENVELOPE => self.set_volume_envelope(value),
            FREQUENCY_LOW => self.set_frequency_low(value),
            FREQUENCY_HIGH => self.set_frequency_high(value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled {
            self.output
        } else {
            0
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
        self.length_timer = 64 - (value & 0x3F);
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
        let length_enabled = if self.length_enabled { 0x40 } else { 0x00 };
        let triggered = if self.triggered { 0x80 } else { 0x00 };

        frequency_high | length_enabled | triggered
    }

    fn set_frequency_high(&mut self, value: u8) {
        let triggered = value & 0x80 != 0;
        self.enabled |= triggered;
        if triggered {
            self.trigger();
        }

        self.length_enabled = value & 0x40 != 0;
        self.frequency = (self.frequency & 0x00FF) | ((value & 0x07) as u16) << 8;
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.output = 0;
        self.timer = 0;
        self.sequence = 0;
        self.dac_enabled = false;
        self.frequency = 0;
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
        self.length_enabled = false;
        self.triggered = false;
    }
}
