mod channel;
mod frame_sequencer;
mod master_volume;
mod mixer;

use crate::{
    apu::{
        channel::{
            noise_channel::NoiseChannel,
            square_channel::SquareChannel,
            wave_channel::{WaveChannel, WAVE_PATTERN_END, WAVE_PATTERN_START},
        },
        frame_sequencer::FrameSequencer,
        master_volume::MasterVolume,
        mixer::Mixer,
    },
    cpu::clock::CPU_CLOCK_SPEED,
};

#[derive(PartialEq)]
pub enum ChannelType {
    CH1,
    CH2,
    CH3,
    CH4,
}

const SAMPLING_FREQUENCY: u16 = 44100;
pub const APU_CLOCK_SPEED: u16 = 512;
pub const LENGTH_TIMER_MAX: u16 = 64;

const CH1_START: u16 = 0xFF10;
const CH1_END: u16 = 0xFF14;

const CH2_START: u16 = 0xFF16;
const CH2_END: u16 = 0xFF19;

pub const CH3_START: u16 = 0xFF1A;
pub const CH3_END: u16 = 0xFF1E;

pub const CH4_START: u16 = 0xFF20;
pub const CH4_END: u16 = 0xFF23;

const MASTER_VOLUME: u16 = 0xFF24; // NR50
const PANNING: u16 = 0xFF25; // NR51
const MASTER_CONTROL: u16 = 0xFF26; // NR52

pub const AUDIO_START: u16 = CH1_START;
pub const AUDIO_END: u16 = WAVE_PATTERN_END;

/*
 * https://gbdev.io/pandocs/Audio.html
 * https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware
 * https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html
 */
pub struct Apu {
    pub ch1: SquareChannel,
    pub ch2: SquareChannel,
    pub ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    pub master_volume: MasterVolume,
    mixer: Mixer,
    pub enabled: bool,
    counter: f64,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(ChannelType::CH1),
            ch2: SquareChannel::new(ChannelType::CH2),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            frame_sequencer: FrameSequencer::new(),
            master_volume: MasterVolume::new(),
            mixer: Mixer::default(),
            enabled: true,
            counter: 0.0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CH1_START..=CH1_END => {
                let address = calculate_square_address(CH1_START, address);
                self.ch1.read_byte(address)
            }
            0xFF15 => 0xFF,
            CH2_START..=CH2_END => {
                let address = calculate_square_address(CH2_START, address);
                self.ch2.read_byte(address)
            }
            CH3_START..=CH3_END => self.ch3.read_byte(address),
            0xFF1F => 0xFF,
            CH4_START..=CH4_END => self.ch4.read_byte(address),
            MASTER_VOLUME => self.master_volume.get_master_volume(),
            PANNING => (&self.mixer).into(),
            MASTER_CONTROL => self.get_master_control(),
            0xFF27..=0xFF2F => 0xFF,
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.read_wave_ram(address),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if (WAVE_PATTERN_START..=WAVE_PATTERN_END).contains(&address) {
            self.ch3.write_wave_ram(address, value);
            return;
        }

        // NR52 (Master Control) is accessible even if the APU is turned off
        if address == MASTER_CONTROL {
            self.set_master_control(value);
            return;
        }

        if !self.enabled {
            return;
        }

        match address {
            CH1_START..=CH1_END => {
                let address = calculate_square_address(CH1_START, address);
                self.ch1.write_byte(address, value);
            }
            0xFF15 => {}
            CH2_START..=CH2_END => {
                let address = calculate_square_address(CH2_START, address);
                self.ch2.write_byte(address, value);
            }
            CH3_START..=CH3_END => self.ch3.write_byte(address, value),
            0xFF1F => {}
            CH4_START..=CH4_END => self.ch4.write_byte(address, value),
            MASTER_VOLUME => self.master_volume.set_master_volume(value),
            PANNING => self.mixer = value.into(),
            MASTER_CONTROL => self.set_master_control(value),
            0xFF27..=0xFF2F => {}
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = (m_cycles * 4) as u16;

        self.frame_sequencer.tick(
            t_cycles,
            &mut self.ch1,
            &mut self.ch2,
            &mut self.ch3,
            &mut self.ch4,
        );

        if !self.enabled {
            return;
        }

        self.tick_channels(m_cycles);

        self.counter += t_cycles as f64;

        let cpu_cycles_per_sample = CPU_CLOCK_SPEED as f64 / (SAMPLING_FREQUENCY as f64);
        // TODO multiply by fast_foward value 

        while self.counter >= cpu_cycles_per_sample {
            let (output_left, output_right) = self.mixer.mix([
                &self.ch1.core,
                &self.ch2.core,
                &self.ch3.core,
                &self.ch4.core,
            ]);

            // TODO: Deliver samples to frontend
            // if let Ok(()) = self.prod.try_push(output_left) {};
            // if let Ok(()) = self.prod.try_push(output_right) {};

            self.counter -= cpu_cycles_per_sample;
        }
    }

    fn tick_channels(&mut self, m_cycles: u8) {
        self.ch1.tick(m_cycles);
        self.ch2.tick(m_cycles);
        self.ch3.tick(m_cycles);
        self.ch4.tick(m_cycles);
    }

    /*
     * 0xFF26 — NR52 (Audio master control)
     * Bit 7: Audio on/off (Read/Write): This controls whether the APU is
     * powered on at all. Turning the APU off clears all APU registers and
     * makes them read-only until turned back on, except NR521.
     * Bit 0-3: CHn on? (Read-only): Each of these four bits allows checking
     * whether channels are active. Writing to those does not enable or
     * disable the channels, despite many emulators behaving as if.
     */
    fn get_master_control(&self) -> u8 {
        let ch1_enabled = if self.ch1.core.enabled { 0x01 } else { 0x00 };
        let ch2_enabled = if self.ch2.core.enabled { 0x02 } else { 0x00 };
        let ch3_enabled = if self.ch3.core.enabled { 0x04 } else { 0x00 };
        let ch4_enabled = if self.ch4.core.enabled { 0x08 } else { 0x00 };
        let enabled = if self.enabled { 0x80 } else { 0x00 };

        0x70 | ch1_enabled | ch2_enabled | ch3_enabled | ch4_enabled | enabled
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = value & 0x80 != 0;

        if !self.enabled {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.ch1.reset(ChannelType::CH1);
        self.ch2.reset(ChannelType::CH2);
        self.ch3.reset(ChannelType::CH3);
        self.ch4.reset(ChannelType::CH4);
        self.frame_sequencer.reset();
        self.mixer.reset();
        self.master_volume.reset();
        self.counter = 0.0;
    }
}

fn calculate_square_address(base_address: u16, address: u16) -> u16 {
    let offset = address - base_address;

    if address < CH2_START {
        offset
    } else {
        offset + 1
    }
}
