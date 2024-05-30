pub struct ChannelCore {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub output: u8,
    pub timer: i16,
    pub triggered: bool,
    pub volume: u8,
}

impl ChannelCore {
    pub fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            output: 0,
            timer: 0,
            triggered: false,
            volume: 0,
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
        self.volume = 0;
    }
}

impl Default for ChannelCore {
    fn default() -> Self {
        Self::new()
    }
}
