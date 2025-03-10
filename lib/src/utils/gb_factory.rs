use std::error::Error;

use crate::{Emulator, EmulatorType, gb_advance::GameBoyAdvance, gb_classic::GameBoyClassic};

pub enum GameBoyType {
    GameBoyClassic,
    GameBoyColor,
    GameBoyAdvance,
}

pub struct GameBoyFactory;

impl GameBoyFactory {
    pub fn build(
        gb_type: &GameBoyType,
        rom_data: &[u8],
    ) -> Result<EmulatorType, Box<dyn Error>> {
        match gb_type {
            GameBoyType::GameBoyClassic | GameBoyType::GameBoyColor => {
                let gbc = GameBoyClassic::build(gb_type, rom_data)?;
                Ok(EmulatorType::Classic(Box::new(gbc)))
            }
            GameBoyType::GameBoyAdvance => {
                let gba = GameBoyAdvance::build(gb_type, rom_data)?;
                Ok(EmulatorType::Advance(Box::new(gba)))
            }
        }
    }
}
