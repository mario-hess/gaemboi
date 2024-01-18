mod channel;

use crate::apu::channel::noise_channel::NoiseChannel;
use crate::apu::channel::square_channel::{ChannelType, SquareChannel};
use crate::apu::channel::wave_channel::WaveChannel;

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

const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;

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
            ch1: SquareChannel::new(ChannelType::Channel1),
            ch2: SquareChannel::new(ChannelType::Channel2),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            master_volume: 0,
            sound_panning: 0,
            master_control: 0,
            enabled: false,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        0xFF
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Even when disabled, MASTER_CONTROL (NR52) is accessible
        if !self.enabled && address != MASTER_CONTROL {
            return;
        }

        match address {
            CH1_START..=CH1_END => self.ch1.write_byte(address, value),
            CH2_START..=CH2_END => self.ch2.write_byte(address, value),
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

    // NR52
    // |       7      | 6 | 5 | 4 |    3    |    2    |    1    |    0    |
    // | Audio on/off |   |   |   | CH4 on? | CH3 on? | CH2 on? | CH1 on? |
    // |  Read/Write  |   |   |   |  Read   |  Read   |  Read   |  Read   |

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
