use super::{core::CartridgeCore, MemoryBankController, MASK_MSB, RAM_ADDRESS};

pub struct Mbc5 {
    core: CartridgeCore,
}

impl Mbc5 {
    pub fn new(core: CartridgeCore) -> Self {
        Self { core }
    }
}

impl MemoryBankController for Mbc5 {
    fn read_rom(&self, address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            0x0..=0x3 => self.core.rom_data[address as usize],
            0x4..=0x7 => {
                let offset = self.core.rom_offset * self.core.rom_bank as usize;
                self.core.rom_data[(address as usize - self.core.rom_offset) + offset]
            }
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            0x0 | 0x1 => self.core.ram_enabled = (value & 0x0F) == 0x0A,
            0x2 => self.core.rom_bank = (self.core.rom_bank & 0x100) | (value as u16),
            0x3 => self.core.rom_bank = (self.core.rom_bank & 0xFF) | ((value as u16 & 0x01) << 8),
            0x4 | 0x5 => self.core.ram_bank = value & 0x0F,
            0x6 | 0x7 => {}
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

        let ram_offset = self.core.ram_offset;
        let ram_bank = self.core.ram_bank;

        if let Some(ref mut ram_data) = self.core.ram_data {
            let offset = ram_offset * ram_bank as usize;
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
