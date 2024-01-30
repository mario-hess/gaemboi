mod channel;
mod frame_sequencer;
mod mixer;

use std::collections::VecDeque;

use crate::apu::channel::noise_channel::NoiseChannel;
use crate::apu::channel::square_channel::{ChannelType, SquareChannel};
use crate::apu::channel::wave_channel::{WaveChannel, WAVE_PATTERN_END, WAVE_PATTERN_START};
use crate::apu::frame_sequencer::FrameSequencer;
use crate::apu::mixer::Mixer;
use crate::audio::SAMPLING_FREQUENCY;
use crate::clock::CPU_CLOCK_SPEED;

pub const APU_CLOCK_SPEED: u16 = 512;

pub const LENGTH_TIMER_MAX: u8 = 64;

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

pub enum AudioBuffer {
    Buffer1,
    Buffer2,
}

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    mixer: Mixer,
    right_volume: u8,
    left_volume: u8,
    enabled: bool,
    sequencer_tick: u8,
    output_timer: f32,
    pub audio_buffer1: VecDeque<u8>,
    pub audio_buffer2: VecDeque<u8>,
    pub current_audio_buffer: AudioBuffer,
    pub buffer_full: bool,
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
            sequencer_tick: 0,
            output_timer: 0.0,
            audio_buffer1: VecDeque::new(),
            audio_buffer2: VecDeque::new(),
            current_audio_buffer: AudioBuffer::Buffer1,
            buffer_full: false,
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

        self.output_timer += t_cycles as f32;
        while self.output_timer >= (CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32) {
            let (output_left, output_right) =
                self.mixer.mix(&self.ch1, &self.ch2, &self.ch3, &self.ch4);

            self.audio_buffer1.push_back(output_left);
            self.audio_buffer1.push_back(output_right);
            //println!("{:?}", self.audio_buffer1.len());
            /*
            match self.current_audio_buffer {
                AudioBuffer::Buffer1 => {
                    self.audio_buffer1.push_back(output_left);
                    self.audio_buffer1.push_back(output_right);

                    if self.audio_buffer1.len() >= 8192 {
                        self.audio_buffer2.clear();
                        self.current_audio_buffer = AudioBuffer::Buffer2;
                        println!("B1 Full");
                        self.buffer_full = true;
                    }
                }
                AudioBuffer::Buffer2 => {
                    self.audio_buffer2.push_back(output_left);
                    self.audio_buffer2.push_back(output_right);

                    if self.audio_buffer2.len() >= 8192 {
                        self.audio_buffer1.clear();
                        self.current_audio_buffer = AudioBuffer::Buffer1;
                        println!("B2 Full");
                    }
                }
            }
            */

            self.output_timer -= CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32;
        }
    }

    fn tick_channels(&mut self, m_cycles: u8) {
        self.ch1.tick(m_cycles);
        self.ch2.tick(m_cycles);
        self.ch3.tick(m_cycles);
        self.ch4.tick(m_cycles);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CH1_START..=CH1_END => self.ch1.read_byte(CH1_START, address),
            CH2_START..=CH2_END => self.ch2.read_byte(CH2_START, address),
            CH3_START..=CH3_END => self.ch3.read_byte(address),
            CH4_START..=CH4_END => self.ch4.read_byte(address),
            MASTER_VOLUME => self.get_master_volume(),
            PANNING => u8::from(self.mixer),
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
            CH1_START..=CH1_END => {
                self.ch1
                    .write_byte(CH1_START, address, value, &mut self.sequencer_tick)
            }
            CH2_START..=CH2_END => {
                self.ch2
                    .write_byte(CH2_START, address, value, &mut self.sequencer_tick)
            }
            CH3_START..=CH3_END => self
                .ch3
                .write_byte(address, value, &mut self.sequencer_tick),
            CH4_START..=CH4_END => self
                .ch4
                .write_byte(address, value, &mut self.sequencer_tick),
            MASTER_VOLUME => self.set_master_volume(value),
            PANNING => self.mixer.set_panning(value),
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
        self.sequencer_tick = 0;
        self.output_timer = 0.0;
        self.audio_buffer1.clear();
        self.audio_buffer2.clear();
    }
}
