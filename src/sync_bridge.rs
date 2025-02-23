use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use ringbuf::{storage::Heap, traits::Observer, SharedRb};

use crate::{apu::audio::SAMPLING_RATE, FRAME_DURATION, FRAME_DURATION_MICROS};

const THRESHOLD_MIN: usize = SAMPLING_RATE as usize * 4;
const THRESHOLD_MAX: usize = SAMPLING_RATE as usize * 8;

pub struct SyncBridge {
    last_difference_duration: Duration,
}

impl SyncBridge {
    pub fn new() -> Self {
        Self {
            last_difference_duration: Duration::from_micros(0),
        }
    }

    pub fn sync(
        &mut self,
        frame_start_time: &Instant,
        fast_forward: &u8,
        performance_mode: bool,
        apu_enabled: bool,
        ring_buffer_ref: Arc<SharedRb<Heap<u8>>>,
    ) {
        if apu_enabled {
            if ring_buffer_ref.occupied_len() > THRESHOLD_MIN {
                if performance_mode {
                    self.sleep(frame_start_time, fast_forward);

                    if ring_buffer_ref.occupied_len() > THRESHOLD_MAX {
                        while ring_buffer_ref.occupied_len() > THRESHOLD_MIN {
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                    }
                } else {
                    self.spin(frame_start_time, fast_forward);
                }
            }
        } else if performance_mode {
            self.sleep(frame_start_time, fast_forward);
        } else {
            self.spin(frame_start_time, fast_forward);
        }
    }

    fn sleep(&mut self, frame_start_time: &Instant, fast_forward: &u8) {
        let elapsed = frame_start_time.elapsed();
        let frame_duration = Duration::from_micros(FRAME_DURATION_MICROS / *fast_forward as u64);

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

    fn spin(&self, frame_start_time: &Instant, fast_forward: &u8) {
        while frame_start_time.elapsed().as_micros()
            < FRAME_DURATION.as_micros() / *fast_forward as u128
        {
            std::hint::spin_loop();
        }
    }
}
