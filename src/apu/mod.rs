mod channel;

use crate::apu::channel::noise_channel::NoiseChannel;
use crate::apu::channel::square_channel::{ChannelType, SquareChannel};
use crate::apu::channel::wave_channel::{WaveChannel, WAVE_PATTERN_END, WAVE_PATTERN_START};

pub const CH1_START: u16 = 0xFF10;
pub const CH1_END: u16 = 0xFF14;

pub const CH2_START: u16 = 0xFF16;
pub const CH2_END: u16 = 0xFF19;

pub const CH3_START: u16 = 0xFF1A;
pub const CH3_END: u16 = 0xFF1E;

pub const CH4_START: u16 = 0xFF20;
pub const CH4_END: u16 = 0xFF23;

const MASTER_VOLUME: u16 = 0xFF24; // NR50
const PANNING: u16 = 0xFF25; // NR51
const MASTER_CONTROL: u16 = 0xFF26; // NR52

pub const AUDIO_START: u16 = CH1_START;
pub const AUDIO_END: u16 = WAVE_PATTERN_END;

pub struct Apu {
    ch1: SquareChannel,
    ch1_right: bool,
    ch1_left: bool,
    ch2: SquareChannel,
    ch2_right: bool,
    ch2_left: bool,
    ch3: WaveChannel,
    ch3_right: bool,
    ch3_left: bool,
    ch4: NoiseChannel,
    ch4_right: bool,
    ch4_left: bool,
    right_volume: u8,
    left_volume: u8,
    enabled: bool,
    counter: u16,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(ChannelType::CH1),
            ch1_right: false,
            ch1_left: false,
            ch2: SquareChannel::new(ChannelType::CH2),
            ch2_right: false,
            ch2_left: false,
            ch3: WaveChannel::new(),
            ch3_right: false,
            ch3_left: false,
            ch4: NoiseChannel::new(),
            ch4_right: false,
            ch4_left: false,
            right_volume: 0,
            left_volume: 0,
            enabled: false,
            counter: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;
        // self.counter += t_cycles;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CH1_START..=CH1_END => self.ch1.read_byte(CH1_START, address),
            CH2_START..=CH2_END => self.ch2.read_byte(CH2_START, address),
            CH3_START..=CH3_END => self.ch3.read_byte(address),
            CH4_START..=CH4_END => self.ch4.read_byte(address),
            MASTER_VOLUME => self.get_master_volume(),
            PANNING => self.get_panning(),
            MASTER_CONTROL => self.get_master_control(),
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.read_wave_ram(address),
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Even when disabled, MASTER_CONTROL (NR52) is accessible
        if !self.enabled && address != MASTER_CONTROL {
            return;
        }

        match address {
            CH1_START..=CH1_END => self.ch1.write_byte(CH1_START, address, value),
            CH2_START..=CH2_END => self.ch2.write_byte(CH2_START, address, value),
            CH3_START..=CH3_END => self.ch3.write_byte(address, value),
            CH4_START..=CH4_END => self.ch4.write_byte(address, value),
            MASTER_VOLUME => self.set_master_volume(value),
            PANNING => self.set_panning(value),
            MASTER_CONTROL => self.set_master_control(value),
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.write_wave_ram(address, value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn get_master_control(&self) -> u8 {
        let ch1_enabled = if self.ch1.enabled { 0x01 } else { 0x00 };
        let ch2_enabled = if self.ch2.enabled { 0x02 } else { 0x00 };
        let ch3_enabled = if self.ch3.enabled { 0x04 } else { 0x00 };
        let ch4_enabled = if self.ch4.enabled { 0x08 } else { 0x00 };
        let enabled = if self.enabled { 0x80 } else { 0x00 };

        ch1_enabled | ch2_enabled | ch3_enabled | ch4_enabled | enabled
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = value & 0x80 != 0;

        if !self.enabled {
            // TODO: Clear all registers
        }
    }

    fn get_master_volume(&self) -> u8 {
        let right_volume = self.right_volume - 1;
        let left_volume = (self.left_volume - 1) << 4;

        right_volume | left_volume
    }

    fn set_master_volume(&mut self, value: u8) {
        self.right_volume = (value & 0x07) + 1;
        self.left_volume = ((value & 0x70) >> 4) + 1;
    }

    fn get_panning(&self) -> u8 {
        let ch1_right = if self.ch1_right { 0x01 } else { 0x00 };
        let ch2_right = if self.ch2_right { 0x02 } else { 0x00 };
        let ch3_right = if self.ch3_right { 0x04 } else { 0x00 };
        let ch4_right = if self.ch4_right { 0x08 } else { 0x00 };
        let ch1_left = if self.ch1_left { 0x10 } else { 0x00 };
        let ch2_left = if self.ch2_left { 0x20 } else { 0x00 };
        let ch3_left = if self.ch3_left { 0x40 } else { 0x00 };
        let ch4_left = if self.ch4_left { 0x80 } else { 0x00 };

        ch1_right | ch2_right | ch3_right | ch4_right | ch1_left | ch2_left | ch3_left | ch4_left
    }

    fn set_panning(&mut self, value: u8) {
        self.ch1_right = value & 0x01 != 0;
        self.ch2_right = value & 0x02 != 0;
        self.ch3_right = value & 0x04 != 0;
        self.ch4_right = value & 0x08 != 0;
        self.ch1_left = value & 0x10 != 0;
        self.ch2_left = value & 0x20 != 0;
        self.ch3_left = value & 0x40 != 0;
        self.ch4_left = value & 0x80 != 0;
    }
}
