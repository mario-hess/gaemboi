mod gb_advance;
mod gb_classic;
mod utils;

pub use crate::{
    gb_classic::utils::FRAME_DURATION,
    utils::{
        frame_buffer::FrameBuffer,
        gb_factory::{GameBoyFactory, GameBoyType},
        input::{InputAction, InputButton, InputEvent, InputQueue},
    },
};
use std::error::Error;

pub trait Emulator {
    fn build(gb_type: &GameBoyType, rom_data: &[u8]) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn step_frame(&mut self);
    fn set_frame_buffer_listener(&mut self, listener: Box<dyn FrameBufferListener>);
    fn set_audio_samples_listener(&mut self, listener: Box<dyn AudioSamplesListener>);
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>);
}

pub trait FrameBufferListener {
    fn on_frame_ready(&mut self, frame_buffer: &FrameBuffer);
}

pub trait AudioSamplesListener {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8));
}

pub trait InputProvider {
    fn on_input(&mut self) -> Vec<InputEvent>;
}
