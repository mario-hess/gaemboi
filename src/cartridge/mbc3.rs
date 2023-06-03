use crate::cartridge::base::Base;
use crate::cartridge::{MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc3 {}

impl Mbc3 {
    pub fn new() -> Self {
        Self {}
    }
}

impl MemoryBankController for Mbc3 {
    fn read_rom(&self, base: &Base, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF
            // Bank 00 (read-only)
            0x0..=0x3 => base.rom_data[address as usize],
            // 0x4000 - 0x7FFF
            // Bank 01-7F (read-only)
            0x4..=0x7 => {
                let offset = base.rom_offset * base.rom_bank as usize;
                base.rom_data[(address as usize - base.rom_offset) + offset]
            }
            _ => panic!("Address unknown: 0x{:#X}", address),
        }
    }

    fn write_rom(&mut self, base: &mut Base, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF
            // RAM enable (write-only)
            0x0 | 0x1 => {
                base.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // 0x2000 - 0x3FFF
            // ROM bank number (write-only)
            0x2 | 0x3 => {
                // Specify the lower 5 bits
                let bank_number = if value == 0 { 1 } else { value };
                base.rom_bank = (base.rom_bank & 0b0110_0000) | (bank_number & 0b0111_1111);
            }
            // 0x4000 - 0x5FFF
            // RAM bank number (write-only)
            0x4 | 0x5 => {
                base.ram_bank = value & 0b0000_0011;
            }
            _ => println!(
                "Writing to unknown Cartridge ROM location 0x{:04x}",
                address
            ),
        }
    }

    fn write_ram(&mut self, base: &mut Base, address: u16, value: u8) {
        if !base.ram_enabled {
            return;
        }

        if let Some(ref mut ram_data) = base.ram_data {
            let offset = base.ram_offset * base.ram_bank as usize;
            ram_data[(address as usize - RAM_ADDRESS) + offset] = value;
        }
    }

    fn read_ram(&self, base: &Base, address: u16) -> u8 {
        if !base.ram_enabled {
            return 0xFF;
        }

        if let Some(ref ram_data) = base.ram_data {
            let offset = base.ram_offset * base.ram_bank as usize;
            return ram_data[(address as usize - RAM_ADDRESS) + offset];
        }

        0xFF
    }
}
