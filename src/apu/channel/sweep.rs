/*
 * @file    apu/channel/sweep.rs
 * @brief   Channel 1 has a frequency sweep unit that can periodically adjust the channel's frequency up or down. 
 * @author  Mario Hess
 * @date    May 26, 2024
 */

pub struct Sweep {
    step: u8,
    direction: bool,
    pace: u8,
    pub sequence: u8,
}

impl Sweep {
    fn new() -> Self {
        Self {
            step: 0,
            direction: true,
            pace: 0,
            sequence: 0,
        }
    }

    pub fn tick(&mut self, frequency: &mut u16, channel_enabled: &mut bool) {
        if self.pace == 0 {
            return;
        }

        self.sequence += 1;

        if self.sequence >= self.pace {
            let delta = *frequency >> self.step;

            *frequency = if self.direction {
                frequency.saturating_add(delta)
            } else {
                frequency.saturating_sub(delta)
            };

            // Overflow check
            if *frequency > 0x07FF {
                *channel_enabled = false;
                *frequency = 0x07FF;
            }

            self.sequence = 0;
        }
    }

    pub fn set(&mut self, value: u8) {
        self.step = value & 0x07;
        self.direction = (value & 0x08) == 0x00;
        self.pace = (value & 0x70) >> 4;
    }

    pub fn get(&self) -> u8 {
        let shift = self.step & 0x07;
        let direction = if self.direction { 0x08 } else { 0x0 };
        let pace = (self.pace & 0x07) << 4;

        0x80 | shift | direction | pace
    }

    pub fn reset(&mut self) {
        self.step = 0;
        self.direction = true;
        self.pace = 0;
        self.sequence = 0;
    }

    pub fn default() -> Self {
        Sweep::new()
    }
}
