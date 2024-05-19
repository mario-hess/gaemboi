use sdl2::audio::AudioCallback;

use std::collections::VecDeque;

pub const SAMPLING_RATE: u16 = 4096;
pub const SAMPLING_FREQUENCY: u16 = 44100;
const VOLUME: f32 = 50.0;

pub struct Audio<'a> {
    pub audio_buffer: &'a mut VecDeque<u8>,
}

impl<'a> Audio<'a> {
    pub fn new(audio_buffer: &'a mut VecDeque<u8>) -> Self {
        Self { audio_buffer }
    }
}

impl AudioCallback for Audio<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if !self.audio_buffer.is_empty() {
                *x = (self.audio_buffer.pop_front().unwrap() as f32) / VOLUME;
            }
        }
    }
}
