/*
 * @file    apu/audio.rs
 * @brief   Implementation of the audio callback.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

use egui_sdl2_gl::sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub const SAMPLING_RATE: u16 = 512;
pub const SAMPLING_FREQUENCY: u16 = 41000;

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
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for (i, sample) in out.iter_mut().enumerate() {
            if !self.audio_buffer.lock().unwrap().is_empty() {
                let master_volume = if i % 2 == 0 {
                    self.left_master
                } else {
                    self.right_master
                };

                *sample = self.audio_buffer.lock().unwrap().pop_front().unwrap() as i16
                    * *self.volume as i16
                    * *master_volume as i16 + 1;
            } else {
                *sample = 0
            }
        }
    }
}

pub fn create_audio_device<'a>(
    audio_subsystem: &AudioSubsystem,
    left_volume: &'a u8,
    right_volume: &'a u8,
    volume: &'a u8,
    audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>,
) -> AudioDevice<Audio<'a>> {
    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let audio = Audio::new(audio_buffer, left_volume, right_volume, volume);

    audio_subsystem
        .open_playback(None, &device, |_spec| audio)
        .unwrap()
}
