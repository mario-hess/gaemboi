/*
 * @file    apu/channel/volume_envelope.rs
 * @brief   An envelope can be configured for Channel 1, 2 and 4 which allows automatically adjusting the volume over time.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

pub struct VolumeEnvelope {
    pub enabled: bool,
    pub counter: u8,
    pub pace: u8,
    pub direction: bool,
    pub volume: u8,
}

impl VolumeEnvelope {
    pub fn new(value: u8) -> Self {
        let pace = value & 0x07;
        let direction = (value & 0x08) != 0;
        let volume = (value & 0xF0) >> 4;

        Self {
            enabled: false,
            counter: 0,
            pace,
            direction,
            volume,
        }
    }

    pub fn tick(&mut self, channel_enabled: &bool) {
        if !self.enabled || !channel_enabled {
            return;
        }

        self.counter += 1;
        if self.counter < self.pace {
            return;
        }

        self.volume = if self.direction {
            self.volume.saturating_add(1)
        } else {
            self.volume.saturating_sub(1)
        };

        if self.volume == 0 || self.volume == 15 {
            self.enabled = false;
        }

        self.counter = 0;
    }

    pub fn set(&mut self, value: u8) {
        self.pace = value & 0x07;
        self.direction = value & 0x08 != 0;
        self.volume = (value & 0xF0) >> 4;
        self.enabled = self.pace > 0;
        self.counter = 0;
    }

    pub fn get(&self) -> u8 {
        let pace = self.pace & 0x07;
        let direction = if self.direction { 0x08 } else { 0x00 };
        let volume = (self.volume & 0x0F) << 4;

        pace | direction | volume
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.counter = 0;
        self.pace = 0;
        self.direction = false;
        self.volume = 0;
    }
}

impl Default for VolumeEnvelope {
    fn default() -> Self {
        Self::new(0x00)
    }
}

#[cfg(test)]
mod volume_envelope_tests {
    use super::*;

    #[test]
    fn default_values() {
        let volume_envelope = VolumeEnvelope::default();

        assert_eq!(volume_envelope.get(), 0x00);
    }

    #[test]
    fn empty_fill() {
        let mut volume_envelope = VolumeEnvelope::default();

        let value = 0x00;
        volume_envelope.set(value);

        assert_eq!(volume_envelope.get(), 0x00);
    }

    #[test]
    fn saturate_all() {
        let mut volume_envelope = VolumeEnvelope::default();

        let value = 0xFF;
        volume_envelope.set(value);

        assert_eq!(volume_envelope.get(), 0xFF);
    }

    #[test]
    fn saturate_pace() {
        let mut volume_envelope = VolumeEnvelope::default();

        let value = 0x07;
        volume_envelope.set(value);

        assert_eq!(volume_envelope.get(), 0x07);
    }

    #[test]
    fn saturate_direction() {
        let mut volume_envelope = VolumeEnvelope::default();

        let value = 0x08;
        volume_envelope.set(value);

        assert_eq!(volume_envelope.get(), 0x08);
    }

    #[test]
    fn saturate_volume() {
        let mut volume_envelope = VolumeEnvelope::default();

        let value = 0xF0;
        volume_envelope.set(value);

        assert_eq!(volume_envelope.get(), 0xF0);
    }
}
