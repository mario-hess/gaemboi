/**
 * @file    apu/channel/length_counter.rs
 * @brief   An envelope can be configured for Channel 1, 2 and 4 which allows automatically adjusting the volume over time.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

pub struct VolumeEnvelope {
    pub enabled: bool,
    pub sequence: u8,
    pub pace: u8,
    pub direction: bool,
    pub volume: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        Self {
            enabled: false,
            sequence: 0,
            pace: 0,
            direction: true,
            volume: 0,
        }
    }

    pub fn tick(&mut self, channel_enabled: &bool) {
        if !self.enabled || !channel_enabled {
            return;
        }

        self.sequence += 1;
        if self.sequence >= self.pace {
            self.volume = if self.direction {
                self.volume.saturating_add(1)
            } else {
                self.volume.saturating_sub(1)
            };
            if self.volume == 0 || self.volume == 15 {
                self.enabled = false;
            }

            self.sequence = 0;
        }
    }

    pub fn set(&mut self, value: u8) {
        self.pace = value & 0x07;
        self.direction = value & 0x08 != 0;
        self.volume = (value & 0xF0) >> 4;
        self.enabled = self.pace > 0;
        self.sequence = 0;
    }

    pub fn get(&self) -> u8 {
        let pace = self.pace & 0x07;
        let direction = if self.direction { 0x08 } else { 0x00 };
        let volume = (self.volume & 0x0F) << 4;

        pace | direction | volume
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.sequence = 0;
        self.pace = 0;
        self.direction = true;
        self.volume = 0;
    }
}

impl Default for VolumeEnvelope {
    fn default() -> Self {
        Self::new()
    }
}
