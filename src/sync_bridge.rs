use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{
    apu::{AUDIO_BUFFER_THRESHOLD_MAX, AUDIO_BUFFER_THRESHOLD_MIN},
    FRAME_DURATION, FRAME_DURATION_MICROS,
};

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
        fast_forward: &u32,
        performance_mode: bool,
        apu_enabled: bool,
        audio_buffer: Arc<Mutex<VecDeque<u8>>>,
    ) {
        if apu_enabled {
            if performance_mode {
                let buffer_size = audio_buffer.lock().unwrap().len();

                if buffer_size > AUDIO_BUFFER_THRESHOLD_MAX {
                    self.sleep(frame_start_time, fast_forward, Some(1.2));
                } else if buffer_size < AUDIO_BUFFER_THRESHOLD_MIN {
                    self.sleep(frame_start_time, fast_forward, Some(0.8));
                } else {
                    self.sleep(frame_start_time, fast_forward, None);
                }
            } else {
                self.spin(frame_start_time, fast_forward);
            }
        } else if performance_mode {
            self.sleep(frame_start_time, fast_forward, None);
        } else {
            self.spin(frame_start_time, fast_forward);
        }
    }

    fn sleep(
        &mut self,
        frame_start_time: &Instant,
        fast_forward: &u32,
        adjustment_factor: Option<f64>,
    ) {
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

        let adjusted_target_duration = if let Some(factor) = adjustment_factor {
            new_target_duration.mul_f64(factor)
        } else {
            new_target_duration
        };

        let target_duration = if adjusted_target_duration.as_micros() > 0 {
            adjusted_target_duration
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

    fn spin(&self, frame_start_time: &Instant, fast_forward: &u32) {
        while frame_start_time.elapsed().as_micros()
            < FRAME_DURATION.as_micros() / *fast_forward as u128
        {
            std::hint::spin_loop();
        }
    }
}
