/**
 * @file    cartridge/core.rs
 * @brief   Initializes cartridge ROM-, and RAM-Bank data.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use crate::cartridge::{RAM_BANK_SIZE, RAM_SIZE_ADDRESS, ROM_BANK_SIZE};

pub struct Core {
    pub rom_data: Vec<u8>,
    pub ram_data: Option<Vec<u8>>,
    pub rom_bank: u8,
    pub ram_bank: u8,
    pub rom_offset: usize,
    pub ram_offset: usize,
    pub ram_enabled: bool,
}

impl Core {
    pub fn new(rom_data: &[u8]) -> Self {
        let ram_data = create_ram(rom_data);
        let rom_bank = 1;
        let ram_bank = 0;
        let rom_offset: usize = ROM_BANK_SIZE;
        let ram_offset: usize = RAM_BANK_SIZE;
        let ram_enabled = false;

        Self {
            rom_data: rom_data.to_vec(),
            ram_data,
            rom_bank,
            ram_bank,
            rom_offset,
            ram_offset,
            ram_enabled,
        }
    }
}

fn create_ram(rom_data: &[u8]) -> Option<Vec<u8>> {
    let ram_size = match rom_data[RAM_SIZE_ADDRESS] {
        0x00 => None,
        0x01 => None,
        0x02 => Some(8 * 1024),
        0x03 => Some(32 * 1024),
        0x04 => Some(128 * 1024),
        0x05 => Some(64 * 1024),
        _ => None,
    };

    ram_size.map(|size| vec![0; size])
}
