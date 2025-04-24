mod gb_advance;
mod gb_classic;
mod utils;

pub use crate::{
    gb_advance::utils::constants::{
        SCREEN_HEIGHT as ADVANCE_SCREEN_HEIGHT, SCREEN_WIDTH as ADVANCE_SCREEN_WIDTH,
    },
    gb_classic::utils::{
        FRAME_DURATION, SCREEN_HEIGHT as CLASSIC_SCREEN_HEIGHT,
        SCREEN_WIDTH as CLASSIC_SCREEN_WIDTH,
    },
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
    fn set_frame_buffer_observer(&mut self, observer: Box<dyn FrameBufferObserver>);
    fn set_audio_samples_observer(&mut self, observer: Box<dyn AudioSamplesObserver>);
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>);
}

pub trait FrameBufferObserver {
    fn on_frame_ready(&mut self, frame_buffer: &FrameBuffer);
}

pub trait AudioSamplesObserver {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8));
}

pub trait InputProvider {
    fn on_input(&mut self) -> Vec<InputEvent>;
}
