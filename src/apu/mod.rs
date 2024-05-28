/**
 * @file    apu/mod.rs
 * @brief   Implementation of the Audio Processing Unit (APU).
 * @author  Mario Hess
 * @date    May 28, 2024
 */
pub mod audio;
mod channel;
mod frame_sequencer;
mod mixer;

use std::collections::VecDeque;

use crate::{
    apu::{
        audio::{SAMPLING_FREQUENCY, SAMPLING_RATE},
        channel::{
            noise_channel::NoiseChannel,
            square_channel::{ChannelType, SquareChannel},
            wave_channel::{WaveChannel, WAVE_PATTERN_END, WAVE_PATTERN_START},
        },
        frame_sequencer::FrameSequencer,
        mixer::Mixer,
    },
    clock::CPU_CLOCK_SPEED,
    memory_bus::MemoryAccess,
};

const CPU_CYCLES_PER_SAMPLE: f32 = CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32;
pub const APU_CLOCK_SPEED: u16 = 512;
pub const LENGTH_TIMER_MAX: u16 = 64;

const AUDIO_BUFFER_SIZE: usize = SAMPLING_RATE as usize * 4;

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

/**
 * https://gbdev.io/pandocs/Audio.html
 * https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware
 * https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html
 */
pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    mixer: Mixer,
    pub right_volume: u8,
    pub left_volume: u8,
    enabled: bool,
    counter: f32,
    pub audio_buffer: VecDeque<u8>,
}

impl MemoryAccess for Apu {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            CH1_START..=CH1_END => {
                let address = calculate_square_address(CH1_START, address);
                self.ch1.read_byte(address)
            }
            CH2_START..=CH2_END => {
                let address = calculate_square_address(CH2_START, address);
                self.ch2.read_byte(address)
            }
            CH3_START..=CH3_END => self.ch3.read_byte(address),
            CH4_START..=CH4_END => self.ch4.read_byte(address),
            MASTER_VOLUME => self.get_master_volume(),
            PANNING => u8::from(self.mixer),
            MASTER_CONTROL => self.get_master_control(),
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.read_wave_ram(address),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        // Even when disabled, MASTER_CONTROL (NR52) is accessible
        if address == MASTER_CONTROL {
            self.set_master_control(value);
        }

        if !self.enabled {
            return;
        }

        match address {
            CH1_START..=CH1_END => {
                let address = calculate_square_address(CH1_START, address);
                self.ch1.write_byte(address, value);
            }
            CH2_START..=CH2_END => {
                let address = calculate_square_address(CH2_START, address);
                self.ch2.write_byte(address, value);
            }
            CH3_START..=CH3_END => self.ch3.write_byte(address, value),
            CH4_START..=CH4_END => self.ch4.write_byte(address, value),
            MASTER_VOLUME => self.set_master_volume(value),
            PANNING => {
                self.mixer.set_panning(value);
            }
            MASTER_CONTROL => {}
            WAVE_PATTERN_START..=WAVE_PATTERN_END => self.ch3.write_wave_ram(address, value),
            _ => unreachable!(),
        }
    }
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(ChannelType::CH1),
            ch2: SquareChannel::new(ChannelType::CH2),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            frame_sequencer: FrameSequencer::new(),
            mixer: Mixer::default(),
            right_volume: 0,
            left_volume: 0,
            enabled: false,
            counter: 0.0,
            audio_buffer: VecDeque::with_capacity(AUDIO_BUFFER_SIZE),
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        if !self.enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;

        self.frame_sequencer.tick(
            t_cycles,
            &mut self.ch1,
            &mut self.ch2,
            &mut self.ch3,
            &mut self.ch4,
        );

        self.tick_channels(m_cycles);

        self.counter += t_cycles as f32;

        while self.counter >= CPU_CYCLES_PER_SAMPLE {
            let (output_left, output_right) =
                self.mixer.mix(&self.ch1, &self.ch2, &self.ch3, &self.ch4);

            self.audio_buffer.push_back(output_left);
            self.audio_buffer.push_back(output_right);

            self.counter -= CPU_CYCLES_PER_SAMPLE;
        }
    }

    fn tick_channels(&mut self, m_cycles: u8) {
        self.ch1.tick(m_cycles);
        self.ch2.tick(m_cycles);
        self.ch3.tick(m_cycles);
        self.ch4.tick(m_cycles);
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
            self.reset();
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

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.frame_sequencer.reset();
        self.mixer.reset();

        self.left_volume = 0;
        self.right_volume = 0;
        self.counter = 0.0;
        self.audio_buffer.clear();
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
