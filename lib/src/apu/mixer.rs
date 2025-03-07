use crate::apu::channel::core::ChannelCore;

const CH1_RIGHT_MASK: u8 = 0x01;
const CH2_RIGHT_MASK: u8 = 0x02;
const CH3_RIGHT_MASK: u8 = 0x04;
const CH4_RIGHT_MASK: u8 = 0x08;
const CH1_LEFT_MASK: u8 = 0x10;
const CH2_LEFT_MASK: u8 = 0x20;
const CH3_LEFT_MASK: u8 = 0x40;
const CH4_LEFT_MASK: u8 = 0x80;

const MASKS: [(u8, u8); 4] = [
    (CH1_RIGHT_MASK, CH1_LEFT_MASK),
    (CH2_RIGHT_MASK, CH2_LEFT_MASK),
    (CH3_RIGHT_MASK, CH3_LEFT_MASK),
    (CH4_RIGHT_MASK, CH4_LEFT_MASK),
];

// 0xFF25 — NR51 (Sound panning)
pub struct Mixer {
    pub panning: [bool; 8],
}

impl Mixer {
    pub fn new() -> Self {
        let value = 0xF3;
        let mut panning = [false; 8];

        for (i, &(right_mask, left_mask)) in MASKS.iter().enumerate() {
            panning[i] = value & right_mask != 0;
            panning[i + 4] = value & left_mask != 0;
        }

        Mixer { panning }
    }

    pub fn mix(&self, channels: [&ChannelCore; 4]) -> (u8, u8) {
        let (mut output_left, mut output_right) = (0, 0);

        for (i, channel) in channels.iter().enumerate() {
            if self.panning[i + 4] {
                output_left += channel.get_output();
            }

            if self.panning[i] {
                output_right += channel.get_output();
            }
        }

        (output_left / 4, output_right / 4)
    }

    pub fn reset(&mut self) {
        self.panning = [false; 8];
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Mixer::new()
    }
}

impl std::convert::From<&Mixer> for u8 {
    fn from(mixer: &Mixer) -> u8 {
        MASKS
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &(right_mask, left_mask))| {
                acc | if mixer.panning[i] { right_mask } else { 0 }
                    | if mixer.panning[i + 4] { left_mask } else { 0 }
            })
    }
}

impl std::convert::From<u8> for Mixer {
    fn from(value: u8) -> Self {
        let mut mixer = Mixer::new();
        for (i, &(right_mask, left_mask)) in MASKS.iter().enumerate() {
            mixer.panning[i] = value & right_mask != 0;
            mixer.panning[i + 4] = value & left_mask != 0;
        }

        mixer
    }
}
