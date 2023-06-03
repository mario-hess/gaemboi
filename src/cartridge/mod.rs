mod base;
mod mbc1;
mod mbc3;

use crate::cartridge::base::Base;
use crate::cartridge::mbc1::Mbc1;
use crate::cartridge::mbc3::Mbc3;

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

const RAM_ADDRESS: usize = 0xA000;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;

// const ROM_SIZE_ADDRESS: usize = 0x0148;
const RAM_SIZE_ADDRESS: usize = 0x149;

const MASK_MSB: u16 = 0xF000;

pub trait MemoryBankController {
    fn read_rom(&self, base: &Base, address: u16) -> u8;
    fn write_rom(&mut self, base: &mut Base, address: u16, value: u8);
    fn read_ram(&self, base: &Base, address: u16) -> u8;
    fn write_ram(&mut self, base: &mut Base, address: u16, value: u8);
}

pub struct Cartridge {
    pub base: Base,
    pub mbc: Box<dyn MemoryBankController>,
}

impl Cartridge {
    pub fn build(rom_data: Vec<u8>) -> Self {
        let base = Base::new(&rom_data);

        let mbc: Box<dyn MemoryBankController> = match rom_data[CARTRIDGE_TYPE_ADDRESS] {
            0x01..=0x03 => Box::new(Mbc1::new()),
            0x0F..=0x13 => Box::new(Mbc3::new()),
            _ => panic!("Error: Cartridge type not supported"),
        };

        Self { base, mbc }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match (addr & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.read_rom(&self.base, addr),
            0xA | 0xB => self.mbc.read_ram(&self.base, addr),
            _ => {
                println!("Reading from unknown Cartridge space 0x{:#X}", addr);
                0x00
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match (addr & MASK_MSB) >> 12 {
            0x0..=0x7 => self.mbc.write_rom(&mut self.base, addr, value),
            0xA | 0xB => self.mbc.write_ram(&mut self.base, addr, value),
            _ => println!("Writing to unknown Cartridge address 0x{:#X}", addr),
        }
    }
}
