use crate::AudioConsumer;
use crate::sdl2::audio::AudioDevice;
use gaemboi::AudioSamplesObserver;
use ringbuf::{SharedRb, storage::Heap, traits::Producer, wrap::caching::Caching};
use std::sync::Arc;

pub struct AudioProducer {
    rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>,
    audio_device: AudioDevice<AudioConsumer>,
    playback_started: bool,
}

impl AudioProducer {
    pub fn new(
        rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>,
        audio_device: AudioDevice<AudioConsumer>,
    ) -> Self {
        Self {
            rb_producer,
            audio_device,
            playback_started: false,
        }
    }

    fn queue_samples(&mut self, audio_samples: &(u8, u8)) {
        let (left_sample, right_sample) = *audio_samples;

        if let Ok(()) = self.rb_producer.try_push(left_sample) {}
        if let Ok(()) = self.rb_producer.try_push(right_sample) {}

        if !self.playback_started {
            self.audio_device.resume();
            self.playback_started = true;
        }
    }
}

impl AudioSamplesObserver for AudioProducer {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8)) {
        self.queue_samples(audio_samples);
    }
}
