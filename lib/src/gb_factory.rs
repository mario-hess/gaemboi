use std::error::Error;

use crate::{
    gb_advance::GameBoyAdvance, gb_classic::GameBoyClassic, AudioSamplesObserver,
    FrameBufferObserver, InputProvider,
};

pub struct GameBoyFactory;

pub trait Emulator {
    fn build(gb_type: GameBoyType, rom_data: &Vec<u8>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn step_frame(&mut self);
    fn set_frame_buffer_observer(&mut self, observer: Box<dyn FrameBufferObserver>);
    fn set_audio_samples_observer(&mut self, observer: Box<dyn AudioSamplesObserver>);
    fn set_input_provider(&mut self, provider: Box<dyn InputProvider>);
}

pub enum GameBoyType {
    GameBoyClassic,
    GameBoyColor,
    GameBoyAdvance,
}

impl GameBoyFactory {
    pub fn build(
        gb_type: GameBoyType,
        rom_data: &Vec<u8>,
    ) -> Result<Box<dyn Emulator>, Box<dyn Error>> {
        match gb_type {
            GameBoyType::GameBoyClassic | GameBoyType::GameBoyColor => {
                let gbc = GameBoyClassic::build(gb_type, rom_data)?;
                Ok(Box::new(gbc))
            }
            GameBoyType::GameBoyAdvance => {
                let gba = GameBoyAdvance::build(gb_type, rom_data)?;
                Ok(Box::new(gba))
            }
        }
    }
}
