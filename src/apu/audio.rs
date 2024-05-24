use sdl2::audio::AudioCallback;

use std::collections::VecDeque;

pub const SAMPLING_RATE: u16 = 4096;
pub const SAMPLING_FREQUENCY: u16 = 48000;

pub struct Audio<'a> {
    pub audio_buffer: &'a mut VecDeque<u8>,
    left_volume: &'a u8,
    right_volume: &'a u8,
    volume: &'a u8,
}

impl<'a> Audio<'a> {
    pub fn new(
        audio_buffer: &'a mut VecDeque<u8>,
        left_volume: &'a u8,
        right_volume: &'a u8,
        volume: &'a u8,
    ) -> Self {
        Self {
            audio_buffer,
            left_volume,
            right_volume,
            volume,
        }
    }
}

impl AudioCallback for Audio<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for (i, sample) in out.iter_mut().enumerate() {
            if !self.audio_buffer.is_empty() {
                let master_volume = if i % 2 == 0 {
                    self.left_volume
                } else {
                    self.right_volume
                };
                *sample = self.audio_buffer.pop_front().unwrap() as f32
                    * (*self.volume as f32 / 10000.0)
                    * *master_volume as f32;
            }
        }
    }
}
