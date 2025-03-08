const MASK_STEP: u8 = 0x07;
const MASK_DIR: u8 = 0x08;
const MASK_PACE: u8 = 0x70;

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
            direction: false,
            pace: 0,
            sequence: 0,
        }
    }

    pub fn tick(&mut self, frequency: &mut u16, channel_enabled: &mut bool) {
        if self.pace == 0 {
            return;
        }

        self.sequence += 1;

        if self.sequence < self.pace {
            return;
        }

        let delta = *frequency >> self.step;

        *frequency = if self.direction {
            frequency.saturating_sub(delta)
        } else {
            frequency.saturating_add(delta)
        };

        // Overflow check
        if *frequency > 0x07FF {
            *channel_enabled = false;
            *frequency = 0x07FF;
        }

        self.sequence = 0;
    }

    pub fn set(&mut self, value: u8) {
        self.step = value & MASK_STEP;
        self.direction = (value & MASK_DIR) != 0x00;
        self.pace = (value & MASK_PACE) >> 4;
    }

    pub fn get(&self) -> u8 {
        let step = self.step & MASK_STEP;
        let direction = if self.direction { MASK_DIR } else { 0x0 };
        let pace = (self.pace & 0x07) << 4;

        0x80 | step | direction | pace
    }

    pub fn reset(&mut self) {
        self.step = 0;
        self.direction = false;
        self.pace = 0;
        self.sequence = 0;
    }

    pub fn default() -> Self {
        Sweep::new()
    }
}

#[cfg(test)]
mod sweep_tests {
    use super::*;

    #[test]
    fn default_values() {
        let sweep = Sweep::new();

        assert_eq!(sweep.get(), 0x80);
    }

    #[test]
    fn empty_fill() {
        let mut sweep = Sweep::new();

        let value = 0x00;
        sweep.set(value);

        assert_eq!(sweep.get(), 0x80);
    }

    #[test]
    fn saturate_all() {
        let mut sweep = Sweep::new();

        let value = 0xFF;
        sweep.set(value);

        assert_eq!(sweep.get(), 0xFF);
    }

    #[test]
    fn saturate_step() {
        let mut sweep = Sweep::new();

        let value = 0x07;
        sweep.set(value);

        assert_eq!(sweep.get(), 0x87);
    }

    #[test]
    fn saturate_direction() {
        let mut sweep = Sweep::new();

        let value = 0x08;
        sweep.set(value);

        assert_eq!(sweep.get(), 0x88);
    }

    #[test]
    fn saturate_pace() {
        let mut sweep = Sweep::new();

        let value = 0x70;
        sweep.set(value);

        assert_eq!(sweep.get(), 0xF0);
    }
}
