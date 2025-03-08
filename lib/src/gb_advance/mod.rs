use crate::{
    gb_factory::{Emulator, GameBoyType},
    AudioSamplesObserver, FrameBufferObserver, InputProvider,
};
use std::error::Error;

pub struct GameBoyAdvance;

impl Emulator for GameBoyAdvance {
    fn build(gb_type: GameBoyType, rom_data: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    fn step_frame(&mut self) {}
    fn set_frame_buffer_observer(&mut self, observer: Box<dyn FrameBufferObserver>) {}
    fn set_audio_samples_observer(&mut self, observer: Box<dyn AudioSamplesObserver>) {}
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {}
}
