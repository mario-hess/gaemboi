/**
 * @file    cartridge/mbc0.rs
 * @brief   MBC0 Memory Bank Controller implementation.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use crate::cartridge::core::Core;
use crate::cartridge::{MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc0 {}

impl Mbc0 {
    pub fn new() -> Self {
        Self {}
    }
}

impl MemoryBankController for Mbc0 {
    fn read_rom(&self, core: &Core, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x7FFF (Bank 00)
            0x0..=0x7 => core.rom_data[address as usize],
            _ => panic!("Unknown address: {:#X}. Can't read byte.", address),
        }
    }

    fn write_rom(&mut self, _core: &mut Core, _address: u16, _value: u8) {}

    fn write_ram(&mut self, core: &mut Core, address: u16, value: u8) {
        if let Some(ref mut ram_data) = core.ram_data {
            ram_data[address as usize - RAM_ADDRESS] = value;
        }
    }

    fn read_ram(&self, core: &Core, address: u16) -> u8 {
        if let Some(ref ram_data) = core.ram_data {
            return ram_data[address as usize - RAM_ADDRESS];
        }

        0xFF
    }
}
