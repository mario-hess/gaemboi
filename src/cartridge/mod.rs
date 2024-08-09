/**
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

use mbc5::Mbc5;

use crate::{
    cartridge::{core::CartridgeCore, mbc0::Mbc0, mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3},
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
    fn get_core(&self) -> &CartridgeCore;
    fn get_core_mut(&mut self) -> &mut CartridgeCore;
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
        let mbc: Box<dyn MemoryBankController> = match rom_data[CARTRIDGE_TYPE_ADDRESS] {
            0x0 => {
                println!("MBC0");
                Box::new(Mbc0::new(rom_data))
            },
            0x01..=0x03 => {
                println!("MBC1");
                Box::new(Mbc1::new(rom_data))
            },
            0x05 | 0x06 => {
                println!("MBC3");
                Box::new(Mbc2::new(rom_data))
            },
            0x0F..=0x13 => {
                println!("MBC3");
                Box::new(Mbc3::new(rom_data))
            },
            0x19..= 0x1E => {
                println!("MBC5");
                Box::new(Mbc5::new(rom_data))
            },
            _ => panic!("Error: Cartridge type not supported"),
        };

        Self { mbc }
    }

    pub fn load_game(&mut self, ram_data: Vec<u8>) {
        let core = self.mbc.get_core_mut();
        core.ram_data = Some(ram_data);
        println!("Game loaded.")
    }

    pub fn save_game(&self, save_path: &str) -> Result<(), Error> {
        let core = self.mbc.get_core();
        if let Some(ram_data) = &core.ram_data {
            let mut file = File::create(save_path)?;
            file.write_all(ram_data)?;
        }

        Ok(())
    }
}
