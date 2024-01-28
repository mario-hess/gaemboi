const CH1_RIGHT_POS: u8 = 0x01;
const CH2_RIGHT_POS: u8 = 0x02;
const CH3_RIGHT_POS: u8 = 0x04;
const CH4_RIGHT_POS: u8 = 0x08;
const CH1_LEFT_POS: u8 = 0x10;
const CH2_LEFT_POS: u8 = 0x20;
const CH3_LEFT_POS: u8 = 0x40;
const CH4_LEFT_POS: u8 = 0x80;

#[derive(Copy, Clone)]
pub struct Mixer {
    ch1_right: bool,
    ch1_left: bool,
    ch2_right: bool,
    ch2_left: bool,
    ch3_right: bool,
    ch3_left: bool,
    ch4_right: bool,
    ch4_left: bool,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            ch1_right: false,
            ch1_left: false,
            ch2_right: false,
            ch2_left: false,
            ch3_right: false,
            ch3_left: false,
            ch4_right: false,
            ch4_left: false,
        }
    }

    pub fn set_panning(&mut self, value: u8) {
        self.ch1_right = value & CH1_RIGHT_POS != 0;
        self.ch2_right = value & CH2_RIGHT_POS != 0;
        self.ch3_right = value & CH3_RIGHT_POS != 0;
        self.ch4_right = value & CH4_RIGHT_POS != 0;
        self.ch1_left = value & CH1_LEFT_POS != 0;
        self.ch2_left = value & CH2_LEFT_POS != 0;
        self.ch3_left = value & CH3_LEFT_POS != 0;
        self.ch4_left = value & CH4_LEFT_POS != 0;
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Mixer::new()
    }
}

impl std::convert::From<Mixer> for u8 {
    fn from(mixer: Mixer) -> u8 {
        let ch1_right = if mixer.ch1_right { CH1_RIGHT_POS } else { 0x00 };
        let ch2_right = if mixer.ch2_right { CH2_RIGHT_POS } else { 0x00 };
        let ch3_right = if mixer.ch3_right { CH3_RIGHT_POS } else { 0x00 };
        let ch4_right = if mixer.ch4_right { CH4_RIGHT_POS } else { 0x00 };
        let ch1_left = if mixer.ch1_left { CH1_LEFT_POS } else { 0x00 };
        let ch2_left = if mixer.ch2_left { CH2_LEFT_POS } else { 0x00 };
        let ch3_left = if mixer.ch3_left { CH3_LEFT_POS } else { 0x00 };
        let ch4_left = if mixer.ch4_left { CH3_LEFT_POS } else { 0x00 };

        ch1_right | ch2_right | ch3_right | ch4_right | ch1_left | ch2_left | ch3_left | ch4_left
    }
}
