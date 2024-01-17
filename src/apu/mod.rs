mod channels;

use crate::apu::channels::noise_channel::NoiseChannel;
use crate::apu::channels::pulse_channel::PulseChannel;
use crate::apu::channels::wave_channel::WaveChannel;

const CH1_SWEEP: u16 = 0xFF10; // NR10
const CH1_LENGTH_TIMER: u16 = 0xFF11; // NR11
const CH1_VOLUME_ENVELOPE: u16 = 0xFF12; // NR12
const CH1_PERIOD_LOW: u16 = 0xFF13; // NR13
const CH1_PERIOD_HIGH_CONTROL: u16 = 0xFF14; // NR14
pub const AUDIO_START: u16 = CH1_SWEEP;
pub const CH1_START: u16 = AUDIO_START;
pub const CH1_END: u16 = CH1_PERIOD_HIGH_CONTROL;

const CH2_LENGTH_TIMER: u16 = 0xFF16; // NR21
const CH2_VOLUME_ENVELOPE: u16 = 0xFF17; // NR22
const CH2_PERIOD_LOW: u16 = 0xFF18; // NR23
const CH2_PERIOD_HIGH_CONTROL: u16 = 0xFF19; // NR24
pub const CH2_START: u16 = CH2_LENGTH_TIMER;
pub const CH2_END: u16 = CH2_PERIOD_HIGH_CONTROL;

const CH3_DAC_ENABLE: u16 = 0xFF1A; // NR30
const CH3_LENGTH_TIMER: u16 = 0xFF1B; // NR31
const CH3_OUTPUT_LEVEL: u16 = 0xFF1C; // NR32
const CH3_PERIOD_LOW: u16 = 0xFF1D; // NR33
const CH3_PERIOD_HIGH_CONTROL: u16 = 0xFF1E; // NR34
pub const CH3_START: u16 = CH3_DAC_ENABLE;
pub const CH3_END: u16 = CH3_PERIOD_HIGH_CONTROL;

const CH4_LENGTH_TIMER: u16 = 0xFF20; // NR41
const CH4_VOLUME_ENVELOPE: u16 = 0xFF21; // NR42
const CH4_FREQUENCY_RANDOMNESS: u16 = 0xFF22; // NR43
const CH4_CHANNEL_CONTROL: u16 = 0xFF23; // NR44
pub const CH4_START: u16 = CH4_LENGTH_TIMER;
pub const CH4_END: u16 = CH4_CHANNEL_CONTROL;

const MASTER_VOLUME: u16 = 0xFF24; // NR50
const SOUND_PANNING: u16 = 0xFF25; // NR51
const MASTER_CONTROL: u16 = 0xFF26; // NR52

const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;
pub const AUDIO_END: u16 = WAVE_PATTERN_END;

pub struct Apu {
    ch1: PulseChannel,
    ch2: PulseChannel,
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
            ch1: PulseChannel::new(),
            ch2: PulseChannel::new(),
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
        // Only MASTER_CONTROL (NR52) is accessible at all times
        if address == MASTER_CONTROL {
            self.set_master_control(value);
            return;
        }

        // If MASTER_CONTROL (NR52) bit 7 is unset, all other registers are read-only
        if !self.enabled {
            return;
        }

        match address {
            CH1_START..=CH1_END => self.ch1.write_byte(address, value),
            CH2_START..=CH2_END => self.ch2.write_byte(address, value),
            CH3_START..=CH3_END => self.ch3.write_byte(address, value),
            CH4_START..=CH4_END => self.ch4.write_byte(address, value),
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
