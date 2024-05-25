/**
 * @file    apu/channel/length_counter.rs
 * @brief   All channels can be individually set to automatically shut themselves down after a certain amount of time.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

pub struct LengthCounter {
    pub enabled: bool,
    pub timer: u16,
}

impl LengthCounter {
    pub fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
        }
    }

    pub fn tick(&mut self, channel_enabled: &mut bool) {
        if !self.enabled || self.timer == 0 {
            return;
        }

        self.timer = self.timer.saturating_sub(1);
        if self.timer == 0 {
            *channel_enabled = false;
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.timer = 0;
    }
}

impl Default for LengthCounter {
    fn default() -> Self {
        Self::new()
    }
}
