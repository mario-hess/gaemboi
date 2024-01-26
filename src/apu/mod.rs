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
const SOUND_PANNING: u16 = 0xFF25; // NR51
const MASTER_CONTROL: u16 = 0xFF26; // NR52

pub const AUDIO_START: u16 = CH1_START;
pub const AUDIO_END: u16 = WAVE_PATTERN_END;

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    master_volume: u8,
    sound_panning: u8,
    master_control: u8,
    enabled: bool,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(ChannelType::CH1),
            ch2: SquareChannel::new(ChannelType::CH2),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            master_volume: 0,
            sound_panning: 0,
            master_control: 0,
            enabled: false,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {}

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CH1_START..=CH1_END => self.ch1.read_byte(CH1_START, address),
            CH2_START..=CH2_END => self.ch2.read_byte(CH2_START, address),
            CH3_START..=CH3_END => self.ch3.read_byte(address),
            CH4_START..=CH4_END => self.ch4.read_byte(address),
            MASTER_CONTROL => self.master_control,
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
            MASTER_CONTROL => self.set_master_control(value),
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.write_wave_ram(address, value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn set_master_control(&mut self, value: u8) {
        let enabled = value & 0b1000_0000;
        self.master_control |= enabled;

        if enabled != 0 {
            self.enabled = true;
        } else {
            self.enabled = false;
            // TODO: Clear all registers
        }
    }
}
