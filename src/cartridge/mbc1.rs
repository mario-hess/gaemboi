/*
 * @file    cartridge/mbc1.rs
 * @brief   MBC1 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    June 8, 2024
 */

use crate::cartridge::{core::CartridgeCore, MemoryBankController, MASK_MSB, RAM_ADDRESS};

enum Mode {
    RomBanking,
    RamBanking,
}

pub struct Mbc1 {
    core: CartridgeCore,
    mode: Mode,
}

impl Mbc1 {
    pub fn new(core: CartridgeCore) -> Self {
        Self {
            core,
            mode: Mode::RomBanking,
        }
    }
}

impl MemoryBankController for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => self.core.rom_data[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = self.core.rom_offset * self.core.rom_bank as usize;
                self.core.rom_data[(address as usize - self.core.rom_offset) + offset]
            }
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => self.core.ram_enabled = value == 0x0A,
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                // Specify the lower 5 bits
                let bank_number = if value == 0 { 1 } else { value };
                self.core.rom_bank = (self.core.rom_bank & 0b0110_0000) | (bank_number & 0b0001_1111) as u16;
            }
            // 0x4000 - 0x5FFF (RAM bank number — or — upper bits of ROM bank number)
            0x4 | 0x5 => match self.mode {
                Mode::RamBanking => self.core.ram_bank = value & 0b11,
                Mode::RomBanking => self.core.rom_bank |= ((value & 0b11) << 5) as u16,
            },
            // 0x6000 - 0x7FFF (Banking mode select)
            0x6 | 0x7 => match value & 0b1 {
                0 => self.mode = Mode::RomBanking,
                1 => self.mode = Mode::RamBanking,
                _ => {}
            },
            _ => unreachable!(),
        }

        self.core.set_rom_bank();
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

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.core.ram_enabled {
            return;
        }

        if let Some(ref mut ram_data) = self.core.ram_data {
            let offset = self.core.ram_offset * self.core.ram_bank as usize;
            ram_data[(address as usize - RAM_ADDRESS) + offset] = value;
        }
    }

    fn load_ram(&mut self, ram_data: Vec<u8>) {
        self.core.ram_data = Some(ram_data);
    }

    fn save_ram(&self) -> Option<Vec<u8>> {
        self.core.ram_data.clone()
    }
}
