/**
 * @file    cartridge/mbc3.rs
 * @brief   MBC3 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    June 8, 2024
 */
use crate::cartridge::{core::CartridgeCore, MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc3 {
    core: CartridgeCore,
}

impl Mbc3 {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            core: CartridgeCore::new(&rom_data),
        }
    }
}

impl MemoryBankController for Mbc3 {
    fn read_rom(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => self.core.rom_data[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = self.core.rom_offset * self.core.rom_bank as usize;
                self.core.rom_data[(address as usize - self.core.rom_offset) + offset]
            }
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => self.core.ram_enabled = (value & 0x0F) == 0x0A,
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                let bank_number = if value == 0 { 1 } else { value };
                self.core.rom_bank = (bank_number & 0b0111_1111) as u16;
            }
            // 0x4000 - 0x5FFF (RAM bank number)
            0x4 | 0x5 => self.core.ram_bank = value & 0b0000_0011,
            0x6 | 0x7 => {}
            _ => eprintln!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
        }

        let max_banks = (self.core.rom_data.len() / self.core.rom_offset).max(1);
        if self.core.rom_bank as usize >= max_banks {
            self.core.rom_bank = (self.core.rom_bank as usize % max_banks) as u16;
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.core.ram_enabled {
            return;
        }

        if let Some(ref mut ram_data) = self.core.ram_data {
            let offset = self.core.ram_offset * self.core.ram_bank as usize;
            ram_data[(address as usize - RAM_ADDRESS) + offset] = value;
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.core.ram_enabled {
            return 0xFF;
        }

        if let Some(ref ram_data) = self.core.ram_data {
            let offset = self.core.ram_offset * self.core.ram_bank as usize;
            return ram_data[(address as usize - RAM_ADDRESS) + offset];
        }

        0xFF
    }

    fn get_core(&self) -> &CartridgeCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut CartridgeCore {
        &mut self.core
    }
}
