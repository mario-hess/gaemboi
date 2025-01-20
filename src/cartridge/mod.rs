/*
 * @file    cartridge/mod.rs
 * @brief   Module for constructing cartridges with memory bank controllers.
 * @author  Mario Hess
 * @date    June 8, 2024
 */

mod core;
mod mbc0;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;

use std::{
    fs::File,
    io::{Error, Write},
};

use crate::{
    cartridge::{core::CartridgeCore, mbc0::Mbc0, mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3, mbc5::Mbc5},
    memory_bus::MemoryAccess,
};

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

const RAM_ADDRESS: usize = 0xA000;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;
const RAM_SIZE_ADDRESS: usize = 0x149;

const MASK_MSB: u16 = 0xF000;

pub trait MemoryBankController {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, ram_data: Vec<u8>);
    fn save_ram(&self) -> Option<Vec<u8>>;
}

pub struct Cartridge {
    pub mbc: Box<dyn MemoryBankController>,
}

impl MemoryAccess for Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.read_rom(address),
            0xA | 0xB => self.mbc.read_ram(address),
            _ => {
                eprintln!("Unknown adress: {:#X} Can't read byte.", address);

                0x00
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.write_rom(address, value),
            0xA | 0xB => self.mbc.write_ram(address, value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}",
                address, value
            ),
        }
    }
}

impl Cartridge {
    pub fn build(rom_data: Vec<u8>) -> Self {
        let core = CartridgeCore::new(&rom_data);

        let mbc: Box<dyn MemoryBankController> = match rom_data[CARTRIDGE_TYPE_ADDRESS] {
            0x0 => Box::new(Mbc0::new(core)),
            0x01..=0x03 => Box::new(Mbc1::new(core)),
            0x05 | 0x06 => Box::new(Mbc2::new(core)),
            0x0F..=0x13 => Box::new(Mbc3::new(core)),
            0x19..=0x1E => Box::new(Mbc5::new(core)),
            _ => panic!("Error: Cartridge type not supported"),
        };

        Self { mbc }
    }

    pub fn load_game(&mut self, ram_data: Vec<u8>) {
        self.mbc.load_ram(ram_data);
        println!("Game loaded.")
    }

    pub fn save_game(&self, save_path: &str) -> Result<(), Error> {
        if let Some(ram_data) = self.mbc.save_ram() {
            let mut file = File::create(save_path)?;
            file.write_all(&ram_data)?;
        }

        Ok(())
    }
}
