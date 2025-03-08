use super::length_counter::LengthCounter;

pub struct ChannelCore {
    pub length_counter: LengthCounter,
    pub enabled: bool,
    pub dac_enabled: bool,
    pub output: u8,
    pub timer: i32,
    pub triggered: bool,
}

impl ChannelCore {
    pub fn new(enabled: bool) -> Self {
        Self {
            length_counter: LengthCounter::default(),
            enabled,
            dac_enabled: false,
            output: 0,
            timer: 0,
            triggered: false,
        }
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled && self.dac_enabled {
            self.output
        } else {
            0
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.output = 0;
        self.timer = 0;
        self.triggered = false;
    }
}

impl Default for ChannelCore {
    fn default() -> Self {
        Self::new(false)
    }
}
