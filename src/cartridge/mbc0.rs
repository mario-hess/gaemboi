/*
 * @file    cartridge/mbc0.rs
 * @brief   MBC0 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    June 8, 2024
 */

use crate::cartridge::{core::CartridgeCore, MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc0 {
    core: CartridgeCore,
}

impl Mbc0 {
    pub fn new(core: CartridgeCore) -> Self {
        Self { core }
    }
}

impl MemoryBankController for Mbc0 {
    fn read_rom(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x7FFF (Bank 00)
            0x0..=0x7 => self.core.rom_data[address as usize],
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}

    fn read_ram(&self, address: u16) -> u8 {
        if let Some(ref ram_data) = self.core.ram_data {
            return ram_data[address as usize - RAM_ADDRESS];
        }

        0xFF
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if let Some(ref mut ram_data) = self.core.ram_data {
            ram_data[address as usize - RAM_ADDRESS] = value;
        }
    }

    fn load_ram(&mut self, ram_data: Vec<u8>) {
        self.core.ram_data = Some(ram_data);
    }

    fn save_ram(&self) -> Option<Vec<u8>> {
        self.core.ram_data.clone()
    }
}
