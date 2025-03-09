use gaemboi::AudioSamplesObserver;
use ringbuf::{
    SharedRb,
    storage::Heap,
    traits::{Consumer, Observer, Producer},
    wrap::caching::Caching,
};
use std::sync::Arc;
use ui::sdl2::audio::AudioCallback;

pub struct AudioProducer {
    rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>,
}

impl AudioProducer {
    pub fn new(rb_producer: Caching<Arc<SharedRb<Heap<u8>>>, true, false>) -> Self {
        Self { rb_producer }
    }

    fn queue_samples(&mut self, audio_samples: (u8, u8), volumes: (u8, u8)) {
        let (left_sample, right_sample) = audio_samples;
        let (left_volume, right_volume) = volumes;

        if let Ok(()) = self.rb_producer.try_push(left_sample * left_volume) {};
        if let Ok(()) = self.rb_producer.try_push(right_sample * right_volume) {};
    }

    pub fn set_volume(&mut self, volume: u8) {
        // self.volume = volume
    }
}

impl AudioSamplesObserver for AudioProducer {
    fn on_samples_ready(&mut self, audio_samples: (u8, u8), volumes: (u8, u8)) {
        self.queue_samples(audio_samples, volumes);
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

        //println!("{}", self.rb_consumer.occupied_len());
    }
}
