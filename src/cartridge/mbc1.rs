/**
 * @file    cartridge/mbc1.rs
 * @brief   MBC1 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    October 20, 2023
 */
use crate::cartridge::{core::Core, MemoryBankController, MASK_MSB, RAM_ADDRESS};

enum Mode {
    RomBanking,
    RamBanking,
}

pub struct Mbc1 {
    mode: Mode,
}

impl Mbc1 {
    pub fn new() -> Self {
        Self {
            mode: Mode::RomBanking,
        }
    }
}

impl MemoryBankController for Mbc1 {
    fn read_rom(&self, core: &Core, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => core.rom_data[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = core.rom_offset * core.rom_bank as usize;
                core.rom_data[(address as usize - core.rom_offset) + offset]
            }
            _ => panic!("Unknown address: {:#X}. Can't read byte.", address),
        }
    }

    fn write_rom(&mut self, core: &mut Core, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => core.ram_enabled = (value & 0x0F) == 0x0A,
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                // Specify the lower 5 bits
                let bank_number = if value == 0 { 1 } else { value };
                core.rom_bank = (core.rom_bank & 0b0110_0000) | (bank_number & 0b0001_1111);
            }
            // 0x4000 - 0x5FFF (RAM bank number — or — upper bits of ROM bank number)
            0x4 | 0x5 => match self.mode {
                Mode::RamBanking => core.ram_bank = value & 0b0000_0011,
                Mode::RomBanking => core.rom_bank |= (value & 0b0000_0011) << 5,
            },
            // 0x6000 - 0x7FFF (Banking mode select)
            0x6 | 0x7 => match value {
                0 => self.mode = Mode::RomBanking,
                1 => self.mode = Mode::RamBanking,
                _ => {}
            },
            _ => panic!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn write_ram(&mut self, core: &mut Core, address: u16, value: u8) {
        if !core.ram_enabled {
            return;
        }

        if let Some(ref mut ram_data) = core.ram_data {
            let offset = core.ram_offset * core.ram_bank as usize;
            ram_data[(address as usize - RAM_ADDRESS) + offset] = value;
        }
    }

    fn read_ram(&self, core: &Core, address: u16) -> u8 {
        if !core.ram_enabled {
            return 0xFF;
        }

        if let Some(ref ram_data) = core.ram_data {
            let offset = core.ram_offset * core.ram_bank as usize;
            return ram_data[(address as usize - RAM_ADDRESS) + offset];
        }

        0xFF
    }
}
