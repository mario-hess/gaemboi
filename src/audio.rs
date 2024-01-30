use sdl2::audio::AudioCallback;

use std::collections::VecDeque;

use crate::apu::AudioBuffer;

pub const SAMPLING_RATE: u16 = 4096;
pub const SAMPLING_FREQUENCY: u16 = 44100;
const VOLUME: f32 = 50.0;

pub struct Audio<'a> {
    current_audio_buffer: &'a AudioBuffer,
    pub audio_buffer1: &'a mut VecDeque<u8>,
    pub audio_buffer2: &'a mut VecDeque<u8>,
}

impl<'a> Audio<'a> {
    pub fn new(
        current_audio_buffer: &'a AudioBuffer,
        audio_buffer1: &'a mut VecDeque<u8>,
        audio_buffer2: &'a mut VecDeque<u8>,
    ) -> Self {
        Self {
            current_audio_buffer,
            audio_buffer1,
            audio_buffer2,
        }
    }
}

impl AudioCallback for Audio<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if !self.audio_buffer1.is_empty() {
                *x = (self.audio_buffer1.pop_front().unwrap() as f32) / VOLUME;
            }
        }
        //println!("{:?}", self.audio_buffer1.len());
        //self.audio_buffer1.clear();
        while self.audio_buffer1.len() >= 4096 {
            self.audio_buffer1.pop_front();
        }

        /*
        match self.current_audio_buffer {
            AudioBuffer::Buffer2 => {
                println!("Pulling from B1");
                for x in out.iter_mut() {
                    if !self.audio_buffer1.is_empty() {
                        *x = (self.audio_buffer1.pop_front().unwrap() as f32) / VOLUME;
                    }
                }
            }
            AudioBuffer::Buffer1 => {
                println!("Pulling from B2");
                for x in out.iter_mut() {
                    if !self.audio_buffer2.is_empty() {
                        *x = (self.audio_buffer2.pop_front().unwrap() as f32) / VOLUME;
                    }
                }
            }
        }
        */
    }
}
