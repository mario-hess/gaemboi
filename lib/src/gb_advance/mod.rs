use crate::{
    AudioSamplesListener, Emulator, FrameBufferListener, InputProvider,
    utils::gb_factory::GameBoyType,
};
use std::error::Error;

pub struct GameBoyAdvance;

impl Emulator<u16> for GameBoyAdvance {
    fn build(gb_type: &GameBoyType, rom_data: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    fn step_frame(&mut self) {}
    fn set_frame_buffer_listener(&mut self, listener: Box<dyn FrameBufferListener<u16>>) {}
    fn set_audio_samples_listener(&mut self, listener: Box<dyn AudioSamplesListener>) {}
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {}
}
