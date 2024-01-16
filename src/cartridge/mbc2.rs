/**
 * @file    cartridge/mbc2.rs
 * @brief   MBC2 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    January 16, 2024
 */
use crate::cartridge::{core::Core, MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc2 {}

impl Mbc2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl MemoryBankController for Mbc2 {
    fn read_rom(&self, core: &Core, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => core.rom_data[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = core.rom_offset * core.rom_bank as usize;
                core.rom_data[(address as usize - core.rom_offset) + offset]
            }
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    fn write_rom(&mut self, core: &mut Core, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => {
                if address & 0x100 != 0 {
                    return;
                }

                core.ram_enabled = value == 0x0A
            }
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                if address & 0x100 != 0x100 {
                    return;
                }

                let bank_number = if value == 0 { 1 } else { value };
                core.rom_bank = bank_number & 0xF;
            }
            0x4..=0x7 => {}
            _ => eprintln!(
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
            ram_data[address as usize - RAM_ADDRESS] = value & 0xF;
        }
    }

    fn read_ram(&self, core: &Core, address: u16) -> u8 {
        if !core.ram_enabled {
            return 0xFF;
        }

        if let Some(ref ram_data) = core.ram_data {
            return ram_data[address as usize - RAM_ADDRESS] & 0xF;
        }

        0xFF
    }
}
