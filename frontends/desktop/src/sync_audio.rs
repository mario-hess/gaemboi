use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use ringbuf::{storage::Heap, traits::Observer, SharedRb};

use crate::{FRAME_DURATION, SAMPLING_RATE};

const THRESHOLD_MIN: usize = SAMPLING_RATE as usize * 8;
const THRESHOLD_MAX: usize = SAMPLING_RATE as usize * 16;

pub struct SyncBridge {
    last_difference_duration: Duration,
}

impl SyncBridge {
    pub fn new() -> Self {
        Self {
            last_difference_duration: Duration::from_micros(0),
        }
    }

    pub fn sync(&mut self, frame_start_time: &Instant, ring_buffer_ref: Arc<SharedRb<Heap<u8>>>) {
        if ring_buffer_ref.occupied_len() > THRESHOLD_MIN {
            self.sleep(frame_start_time);

            balance_buffer(&ring_buffer_ref, || {
                std::thread::sleep(std::time::Duration::from_millis(1))
            });
        }
    }

    fn sleep(&mut self, frame_start_time: &Instant) {
        let elapsed = frame_start_time.elapsed();
        let frame_duration = FRAME_DURATION;

        let base_target_duration = if elapsed < frame_duration {
            frame_duration - elapsed
        } else {
            Duration::from_micros(0)
        };

        let new_target_duration = if base_target_duration > self.last_difference_duration {
            base_target_duration - self.last_difference_duration
        } else {
            base_target_duration
        };

        let target_duration = if new_target_duration.as_micros() > 0 {
            new_target_duration
        } else {
            Duration::from_micros(0)
        };

        let now = Instant::now();
        std::thread::sleep(target_duration);
        let sleep_duration = now.elapsed();

        let difference_duration = if sleep_duration > target_duration {
            sleep_duration - target_duration
        } else {
            Duration::from_micros(0)
        };

        self.last_difference_duration = difference_duration;
    }
}

fn spin(frame_start_time: &Instant, fast_forward: &u8) {
    while frame_start_time.elapsed().as_micros()
        < FRAME_DURATION.as_micros() / *fast_forward as u128
    {
        std::hint::spin_loop();
    }
}

fn balance_buffer<F>(ring_buffer_ref: &Arc<SharedRb<Heap<u8>>>, wait_action: F)
where
    F: Fn(),
{
    if ring_buffer_ref.occupied_len() > THRESHOLD_MAX {
        while ring_buffer_ref.occupied_len() > THRESHOLD_MIN {
            wait_action();
        }
    }
}
