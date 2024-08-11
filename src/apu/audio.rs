/**
 * @file    apu/audio.rs
 * @brief   Implementation of the audio callback.
 * @author  Mario Hess
 * @date    May 25, 2024
 */
use sdl2::audio::AudioCallback;

use std::{collections::VecDeque, sync::{Arc, Mutex}};

pub const SAMPLING_RATE: u16 = 512;
pub const SAMPLING_FREQUENCY: u16 = 48000;

pub struct Audio<'a> {
    pub audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>,
    left_master: &'a u8,
    right_master: &'a u8,
    volume: &'a u8,
}

impl<'a> Audio<'a> {
    pub fn new(
        audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>,
        left_master: &'a u8,
        right_master: &'a u8,
        volume: &'a u8,
    ) -> Self {
        Self {
            audio_buffer,
            left_master,
            right_master,
            volume,
        }
    }
}

impl AudioCallback for Audio<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for (i, sample) in out.iter_mut().enumerate() {
            if !self.audio_buffer.lock().unwrap().is_empty() {
                let master_volume = if i % 2 == 0 {
                    self.left_master
                } else {
                    self.right_master
                };

                *sample = self.audio_buffer.lock().unwrap().pop_front().unwrap() as f32
                    * (*self.volume as f32 / 10000.0)
                    * *master_volume as f32;
            } else {
                println!("EPMTY AT: {:#?}", std::time::Instant::now());
            }
        }
    }
}
