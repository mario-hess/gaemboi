/**
 * @file    apu/mixer.rs
 * @brief   Responsible for mixing the channels outputs.
 * @author  Mario Hess
 * @date    June 5, 2024
 */
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

#[derive(Copy, Clone)]
pub struct Mixer {
    pub panning: [bool; 8],
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            panning: [false; 8],
        }
    }

    pub fn mix(&self, channels: [&ChannelCore; 4]) -> (u8, u8) {
        let (mut output_left, mut output_right) = (0, 0);

        for (i, channel) in channels.iter().enumerate() {
            mix_channels(
                &mut output_left,
                &mut output_right,
                self.panning[i + 4],
                self.panning[i],
                channel.get_output(),
            );
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

impl std::convert::From<Mixer> for u8 {
    fn from(mixer: Mixer) -> u8 {
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

fn mix_channels(
    output_left: &mut u8,
    output_right: &mut u8,
    ch_left: bool,
    ch_right: bool,
    output: u8,
) {
    if ch_left {
        *output_left += output;
    }

    if ch_right {
        *output_right += output;
    }
}
