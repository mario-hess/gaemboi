use crate::{AudioSamplesListener, Emulator, FrameBufferListener, InputProvider};

pub enum EmulatorType {
    Classic(Box<dyn Emulator<u8>>),
    Advance(Box<dyn Emulator<u16>>),
}

impl EmulatorType {
    pub fn set_frame_buffer_listener_u8(
        &mut self,
        listener: Box<dyn FrameBufferListener<u8>>,
    ) -> Result<(), &'static str> {
        match self {
            EmulatorType::Classic(gbc) => {
                gbc.set_frame_buffer_listener(listener);
                Ok(())
            }
            EmulatorType::Advance(_gba) => {
                Err("Cannot set u8 frame buffer listener on GameBoy Advance".into())
            }
        }
    }

    pub fn set_frame_buffer_listener_u16(
        &mut self,
        listener: Box<dyn FrameBufferListener<u16>>,
    ) -> Result<(), &'static str> {
        match self {
            EmulatorType::Classic(_gbc) => {
                Err("Cannot set u16 frame buffer listener on GameBoy Classic or GameBoy Color")
            }
            EmulatorType::Advance(gba) => {
                gba.set_frame_buffer_listener(listener);
                Ok(())
            }
        }
    }

    pub fn step_frame(&mut self) {
        match self {
            EmulatorType::Classic(gbc) => {
                gbc.step_frame();
            }
            EmulatorType::Advance(gba) => {
                gba.step_frame();
            }
        }
    }

    pub fn set_audio_samples_listener(&mut self, listener: Box<dyn AudioSamplesListener>) {
        match self {
            EmulatorType::Classic(gbc) => {
                gbc.set_audio_samples_listener(listener);
            }
            EmulatorType::Advance(gba) => {
                gba.set_audio_samples_listener(listener);
            }
        }
    }

    pub fn set_input_provider(&mut self, provider: Box<dyn InputProvider>) {
        match self {
            EmulatorType::Classic(gbc) => {
                gbc.set_input_provider(provider);
            }
            EmulatorType::Advance(gba) => {
                gba.set_input_provider(provider);
            }
        }
    }
}
