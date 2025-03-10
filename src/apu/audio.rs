/*
 * @file    apu/audio.rs
 * @brief   Implementation of the audio callback.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

use std::sync::Arc;

use egui_sdl2_gl::sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

use ringbuf::{
    storage::Heap,
    traits::Consumer,
    wrap::caching::Caching,
    SharedRb,
};

pub const SAMPLING_RATE: u16 = 512;
pub const SAMPLING_FREQUENCY: u16 = 44100;

pub struct Audio<'a> {
    left_master: &'a u8,
    right_master: &'a u8,
    volume: &'a u8,
    pub cons: Caching<Arc<SharedRb<Heap<u8>>>, false, true>,
}

impl<'a> Audio<'a> {
    pub fn new(
        left_master: &'a u8,
        right_master: &'a u8,
        volume: &'a u8,
        cons: Caching<Arc<SharedRb<Heap<u8>>>, false, true>,
    ) -> Self {
        Self {
            left_master,
            right_master,
            volume,
            cons,
        }
    }
}

impl AudioCallback for Audio<'_> {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for (i, sample) in out.iter_mut().enumerate() {
            let master_volume = if i % 2 == 0 {
                self.left_master
            } else {
                self.right_master
            };

            if let Some(s) = self.cons.try_pop() {
                *sample = s as i16 * *self.volume as i16 * *master_volume as i16 + 1;
            } else {
                *sample = 0;
            }
        }
    }
}

pub fn create_audio_device<'a>(
    audio_subsystem: &AudioSubsystem,
    left_volume: &'a u8,
    right_volume: &'a u8,
    volume: &'a u8,
    cons: Caching<Arc<SharedRb<Heap<u8>>>, false, true>,
) -> AudioDevice<Audio<'a>> {
    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let audio = Audio::new(left_volume, right_volume, volume, cons);

    audio_subsystem
        .open_playback(None, &device, |_spec| audio)
        .unwrap()
}
