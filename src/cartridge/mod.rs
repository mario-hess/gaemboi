/**
 * @file    cartridge/mod.rs
 * @brief   Module for constructing cartridges with memory bank controllers.
 * @author  Mario Hess
 * @date    January 16, 2024
 */
mod core;
mod mbc0;
mod mbc1;
mod mbc2;
mod mbc3;

use std::{fs::File, io::Write};

use crate::cartridge::{core::CartridgeCore, mbc0::Mbc0, mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3};

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

const RAM_ADDRESS: usize = 0xA000;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;
const RAM_SIZE_ADDRESS: usize = 0x149;

const MASK_MSB: u16 = 0xF000;

pub trait MemoryBankController {
    fn read_rom(&self, core: &CartridgeCore, address: u16) -> u8;
    fn write_rom(&mut self, core: &mut CartridgeCore, address: u16, value: u8);
    fn read_ram(&self, core: &CartridgeCore, address: u16) -> u8;
    fn write_ram(&mut self, core: &mut CartridgeCore, address: u16, value: u8);
}

pub struct Cartridge {
    pub core: CartridgeCore,
    pub mbc: Box<dyn MemoryBankController>,
}

impl Cartridge {
    pub fn build(rom_data: Vec<u8>) -> Self {
        let core = CartridgeCore::new(&rom_data);

        let mbc: Box<dyn MemoryBankController> = match rom_data[CARTRIDGE_TYPE_ADDRESS] {
            0x0 => Box::new(Mbc0::new()),
            0x01..=0x03 => Box::new(Mbc1::new()),
            0x05 | 0x06 => Box::new(Mbc2::new()),
            0x0F..=0x13 => Box::new(Mbc3::new()),
            _ => panic!("Error: Cartridge type not supported"),
        };

        Self { core, mbc }
    }

    pub fn read(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.read_rom(&self.core, address),
            0xA | 0xB => self.mbc.read_ram(&self.core, address),
            _ => {
                eprintln!("Unknown adress: {:#X} Can't read byte.", address);

                0x00
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.write_rom(&mut self.core, address, value),
            0xA | 0xB => self.mbc.write_ram(&mut self.core, address, value),
            _ => eprintln!(
                "Unknown address: {:#X} Can't write byte: {:#X}",
                address, value
            ),
        }
    }

    pub fn load_game(&mut self, ram_data: Vec<u8>) {
        self.core.ram_data = Some(ram_data);
        println!("Game loaded.")
    }

    pub fn save_game(&self, save_path: &str) {
        if let Some(ram_data) = &self.core.ram_data {
            let mut file = File::create(save_path).expect("Failed to create save file.");
            file.write_all(ram_data)
                .expect("Failed to write save file.");
            println!("Game saved.")
        }
    }
}
