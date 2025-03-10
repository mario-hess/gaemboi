use crate::{
    AudioSamplesListener, Emulator, FrameBufferListener, InputProvider,
    utils::gb_factory::GameBoyType,
};
use std::error::Error;

pub struct GameBoyAdvance;

impl Emulator for GameBoyAdvance {
    fn build(gb_type: &GameBoyType, rom_data: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    fn step_frame(&mut self) {}
    fn set_frame_buffer_listener(&mut self, listener: Box<dyn FrameBufferListener>) {}
    fn set_audio_samples_listener(&mut self, listener: Box<dyn AudioSamplesListener>) {}
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {}
}
