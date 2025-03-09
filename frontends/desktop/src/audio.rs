use gaemboi::AudioSamplesObserver;
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Observer, Producer},
    wrap::caching::Caching,
    SharedRb,
};
use sdl2::audio::AudioCallback;
use std::sync::Arc;

pub struct AudioProducer {
    rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>,
}

impl AudioProducer {
    pub fn new(rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>) -> Self {
        Self { rb_producer }
    }

    fn queue_samples(&mut self, audio_samples: (u8, u8)) {
        let (left_sample, right_sample) = audio_samples;

        if let Ok(()) = self.rb_producer.try_push(left_sample) {}
        if let Ok(()) = self.rb_producer.try_push(right_sample) {}
    }

    pub fn set_volume(&mut self, volume: u8) {
        // self.volume = volume
    }
}

impl AudioSamplesObserver for AudioProducer {
    fn on_samples_ready(&mut self, audio_samples: (u8, u8)) {
        self.queue_samples(audio_samples);
    }
}

pub struct AudioConsumer {
    rb_consumer: Caching<Arc<SharedRb<Heap<u8>>>, false, true>,
}

impl AudioConsumer {
    pub fn new(rb_consumer: Caching<Arc<SharedRb<Heap<u8>>>, false, true>) -> Self {
        Self { rb_consumer }
    }
}

impl AudioCallback for AudioConsumer {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for sample in out.iter_mut() {
            if let Some(s) = self.rb_consumer.try_pop() {
                *sample = s as i16 * 50 as i16;
            } else {
                *sample = 0;
            }
        }
    }
}
