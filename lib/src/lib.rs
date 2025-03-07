use std::error::Error;

use gbc::GameBoyColor;

mod apu;
mod bus;
mod cartridge;
mod cpu;
mod gbc;
mod interrupt;
mod io;
mod ppu;
mod utils;

pub fn build(rom_data: &Vec<u8>) -> Result<GameBoyColor, Box<dyn Error>> {
    GameBoyColor::new(rom_data)
}
