use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x7FFF;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

pub const CARTRIDGE_RAM_START: u16 = 0xA000;
pub const CARTRIDGE_RAM_END: u16 = 0xBFFF;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

pub const BOOT_ROM_END: u16 = 0x100;

/*
  0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
  4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
  8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
  A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
  C000-CFFF   4KB Work RAM Bank 0 (WRAM)
  D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
  E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
  FE00-FE9F   Sprite Attribute Table (OAM)
  FEA0-FEFF   Not Usable
  FF00-FF7F   I/O Ports
  FF80-FFFE   High RAM (HRAM)
  FFFF        Interrupt Enable Register
*/

pub struct MemoryBus {
    cartridge: Cartridge,
    gpu: Gpu,
    wram: [u8; 8192],
    hram: [u8; 128],
}

impl MemoryBus {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let cartridge = Cartridge::build(rom_data);

        Self {
            cartridge,
            gpu: Gpu::new(),
            wram: [0; 8192],
            hram: [0; 128],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.read(address),
            /*
            VRAM_START..=VRAM_END => {
                // read from VRAM
            }
            */
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.read(address),
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize],
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize],
            _ => {
                println!("Unknown address: {:#X} Can't read byte.", address);
                0x00
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.write(address, value),
            /*
            VRAM_START..=VRAM_END => {
                // read from VRAM
            }
            */
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.write(address, value),
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize] = value,
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize] = value,
            _ => panic!("Unknown address: {:#X} Can't write byte.", address),
        }
    }
}
