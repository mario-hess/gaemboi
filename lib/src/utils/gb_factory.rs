use std::error::Error;

use crate::{Emulator, gb_advance::GameBoyAdvance, gb_classic::GameBoyClassic};

#[derive(Clone)]
pub enum GameBoyType {
    Classic,
    Color,
    Advance,
}

pub struct GameBoyFactory;

impl GameBoyFactory {
    pub fn build(
        gb_type: &GameBoyType,
        rom_data: &[u8],
    ) -> Result<Box<dyn Emulator>, Box<dyn Error>> {
        match gb_type {
            GameBoyType::Classic | GameBoyType::Color => {
                let gbc = GameBoyClassic::build(gb_type, rom_data)?;
                Ok(Box::new(gbc))
            }
            GameBoyType::Advance => {
                let gba = GameBoyAdvance::build(gb_type, rom_data)?;
                Ok(Box::new(gba))
            }
        }
    }
}
