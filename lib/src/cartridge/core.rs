use crate::cartridge::{CARTRIDGE_TYPE_ADDRESS, RAM_BANK_SIZE, RAM_SIZE_ADDRESS, ROM_BANK_SIZE};

pub struct CartridgeCore {
    pub rom_data: Vec<u8>,
    pub ram_data: Option<Vec<u8>>,
    pub rom_bank: u16,
    pub ram_bank: u8,
    pub rom_offset: usize,
    pub ram_offset: usize,
    pub ram_enabled: bool,
}

impl CartridgeCore {
    pub fn new(rom_data: &[u8]) -> Self {
        let ram_data = match rom_data[CARTRIDGE_TYPE_ADDRESS] {
            // Mbc2 internal ram
            0x05 | 0x06 => {
                let ram_size = Some(512);
                ram_size.map(|size| vec![0; size])
            }
            _ => create_ram(rom_data),
        };
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

    pub fn set_rom_bank(&mut self) {
        let max_banks = (self.rom_data.len() / self.rom_offset).max(1);
        if self.rom_bank as usize >= max_banks {
            self.rom_bank = (self.rom_bank as usize % max_banks) as u16;
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
