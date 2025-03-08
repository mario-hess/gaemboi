mod gb_advance;
mod gb_classic;
mod gb_factory;
mod input_buttons;

pub use crate::{
    gb_factory::{GameBoyFactory, GameBoyType},
    input_buttons::{InputButton, InputButtons},
};

pub trait FrameBufferObserver {
    fn on_frame_ready(&mut self, frame_buffer: &[u8]);
}

pub trait AudioSamplesObserver {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8));
}

pub trait InputProvider {
    fn get_inputs(&self) -> InputButtons;
}
