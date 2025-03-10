use crate::sdl2::audio::AudioCallback;
use ringbuf::{SharedRb, storage::Heap, traits::Consumer, wrap::caching::Caching};
use std::sync::Arc;

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
