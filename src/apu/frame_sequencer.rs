/*
 * @file    apu/frame_sequencer.rs
 * @brief   Implementation of the APU frame sequencer.
 * @author  Mario Hess
 * @date    May 25, 2024
 */

use crate::{
    apu::{NoiseChannel, SquareChannel, WaveChannel, APU_CLOCK_SPEED},
    cpu::clock::CPU_CLOCK_SPEED,
};

const CYCLES_DIV: u16 = (CPU_CLOCK_SPEED / APU_CLOCK_SPEED as u32) as u16;

pub struct FrameSequencer {
    clock: u16,
    pub step: u8,
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self { clock: 0, step: 0 }
    }

    pub fn tick(
        &mut self,
        t_cycles: u16,
        ch1: &mut SquareChannel,
        ch2: &mut SquareChannel,
        ch3: &mut WaveChannel,
        ch4: &mut NoiseChannel,
    ) {
        /* https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware
        Every 8192 T-cycles (512 Hz) the frame sequencer is stepped and might clock other units
        Step   Length Ctr  Vol Env     Sweep
        ---------------------------------------
        0      Clock       -           -
        1      -           -           -
        2      Clock       -           Clock
        3      -           -           -
        4      Clock       -           -
        5      -           -           -
        6      Clock       -           Clock
        7      -           Clock       -
        ---------------------------------------
        Rate   256 Hz      64 Hz       128 Hz
        */

        self.clock += t_cycles;

        if self.clock >= CYCLES_DIV {
            match self.step {
                0 => self.tick_length_timers(ch1, ch2, ch3, ch4),
                1 => {},
                2 => {
                    ch1.tick_sweep();
                    self.tick_length_timers(ch1, ch2, ch3, ch4);
                }
                3 => {},
                4 => self.tick_length_timers(ch1, ch2, ch3, ch4),
                5 => {},
                6 => {
                    ch1.tick_sweep();
                    self.tick_length_timers(ch1, ch2, ch3, ch4);
                }
                7 => self.tick_envelopes(ch1, ch2, ch4),
                _ => unreachable!()
            }

            self.clock -= CYCLES_DIV;

            // Repeat step 0-7
            self.step = (self.step + 1) & 0x07;
        }
    }

    fn tick_length_timers(
        &mut self,
        ch1: &mut SquareChannel,
        ch2: &mut SquareChannel,
        ch3: &mut WaveChannel,
        ch4: &mut NoiseChannel,
    ) {
        ch1.length_counter.tick(&mut ch1.core.enabled);
        ch2.length_counter.tick(&mut ch2.core.enabled);
        ch3.length_counter.tick(&mut ch3.core.enabled);
        ch4.length_counter.tick(&mut ch4.core.enabled);
    }

    fn tick_envelopes(
        &mut self,
        ch1: &mut SquareChannel,
        ch2: &mut SquareChannel,
        ch4: &mut NoiseChannel,
    ) {
        ch1.volume_envelope.tick(&ch1.core.enabled);
        ch2.volume_envelope.tick(&ch2.core.enabled);
        ch4.volume_envelope.tick(&ch4.core.enabled);
    }

    pub fn reset(&mut self) {
        self.clock = 0;
        self.step = 0;
    }
}
