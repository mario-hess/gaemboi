use crate::cartridge::Cartridge;
use crate::ppu::{Ppu, BLACK, DARK, LIGHT, WHITE};
use crate::ppu::tile::{Tile, TILE_HEIGHT, TILE_WIDTH};
use crate::timer::Timer;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x7FFF;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

pub const CARTRIDGE_RAM_START: u16 = 0xA000;
pub const CARTRIDGE_RAM_END: u16 = 0xBFFF;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;

const JOYPAD_INPUT: u16 = 0xFF00;
const SERIAL_SB: u16 = 0xFF01;
const SERIAL_SC: u16 = 0xFF02;

const TIMER_START: u16 = 0xFF04;
const TIMER_END: u16 = 0xFF07;

const INTERRUPT_FLAG: u16 = 0xFF0F;

const AUDIO_START: u16 = 0xFF10;
const AUDIO_END: u16 = 0xFF26;
const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;

const PPU_IO_START: u16 = 0xFF40;
const PPU_IO_END: u16 = 0xFF4B;

const SPEED_SWITCH: u16 = 0xFF4D;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE: u16 = 0xFFFF;

pub const TILE_TABLE_WIDTH: usize = 256;
pub const TILE_TABLE_HEIGHT: usize = 96;
const TILE_TABLE_PER_ROW: usize = 32;


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
    pub ppu: Ppu,
    wram: [u8; 8192],
    hram: [u8; 128],
    pub interrupt_enable: u8,
    joypad_input: u8,
    serial_sb: u8,
    serial_sc: u8,
    pub timer: Timer,
    pub interrupt_flag: u8,
    audio: [u8; 23],
    wave_pattern: [u8; 16],
    speed_switch: u8,
}

impl MemoryBus {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let cartridge = Cartridge::build(rom_data);

        Self {
            cartridge,
            ppu: Ppu::new(),
            wram: [0; 8192],
            hram: [0; 128],
            interrupt_enable: 0,
            joypad_input: 0,
            serial_sb: 0,
            serial_sc: 0,
            timer: Timer::new(),
            interrupt_flag: 0,
            audio: [0; 23],
            wave_pattern: [0; 16],
            speed_switch: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        self.timer.tick(m_cycles);
        self.interrupt_flag |= self.timer.interrupt_request;
        self.timer.reset_interrupt();
    }

    pub fn get_interrupt_flag(&mut self) -> u8 {
        self.interrupt_flag
    }

    pub fn get_interrupt_enable(&mut self) -> u8 {
        self.interrupt_enable
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            // 0000 - 7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.read(address),
            // 8000 - 9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.read_byte(address),
            // A000 - BFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.read(address),
            // C000 - DFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize],
            // FF00 - FF7F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.read_byte(address),
            // FF00 (Joypad)
            JOYPAD_INPUT => self.joypad_input,
            // FF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb,
            // FF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc,
            // FF04 - FF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.read_byte(address),
            // FF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag,
            // FF10 - FF26 (Audio Channel Control)
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize],
            // FF30 - FF3F (Audio Wave storage)
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize]
            }
            // FF40 - FF4B (PPU Registers)
            PPU_IO_START..=PPU_IO_END => self.ppu.read_byte(address),
            // FF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch,
            // FF80 - FFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize],
            // FFFF (Interrupt Enable Register)
            INTERRUPT_ENABLE => self.interrupt_enable,
            _ => panic!("Unknown address: {:#X} Can't read byte.", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Test rom logs
        if address == SERIAL_SB {
            print!("{}", char::from(value));
        }

        match address {
            // 0000 - 7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.write(address, value),
            // 8000 - 9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.write_byte(address, value),
            // A000 - BFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.write(address, value),
            // C000 - DFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize] = value,
            // FF00 - FF7F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.write_byte(address, value),
            // FF00 (Joypad)
            JOYPAD_INPUT => self.joypad_input = value,
            // FF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb = value,
            // FF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc = value,
            // FF04 - FF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.write_byte(address, value),
            // FF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag = value,
            // FF10 - FF26 (Audio Channel Control)
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize] = value,
            // FF30 - FF3F (Audio Wave storage)
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize] = value;
            }
            // FF40 - FF4B (PPU Registers)
            PPU_IO_START..=PPU_IO_END => self.ppu.write_byte(address, value),
            // FF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch = value,
            // FF80 - FFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize] = value,
            // FFFF (Interrupt Enable Register)
            INTERRUPT_ENABLE => self.interrupt_enable = value,
            _ => panic!("Unknown address: {:#X} Can't write byte.", address),
        }
    }

    pub fn debug_draw_tile_table(
        &mut self,
        tile_data_canvas: &mut Canvas<Window>,
    ) {
        // Tile data is stored in VRAM in the memory area at 0x8000-0x97FF.
        // Block 0 at 0x8000–0x87FF, Objects 0–127.
        // Block 1 at 0x8800–0x8FFF, Objects 128–255.
        // Block 2 at 0x9000–0x97FF, (Can't use, Objects always use “0x8000 addressing”).
        // 0x8000 addressing: unsigned addressing (Block 0 and 1).
        // 0x8800 addressing: signed addressing (Block 2 and 1).

        // 6144 bytes
        let mut tile_data = Vec::<u8>::new();

        let addressing_mode = self.ppu.get_lcd_control4();

        self.ppu.set_lcd_control4(false);
        for i in VRAM_START..=VRAM_END {
            tile_data.push(self.read_byte(i));
        }
        self.ppu.set_lcd_control4(addressing_mode);

        let mut tile_table = Vec::<Tile>::new();
        // Each tile taking 16 bytes
        for chunk in tile_data.chunks(16) {
            let mut tile_bytes = [0; 16];
            tile_bytes.copy_from_slice(chunk);

            let tile = Tile::new(tile_bytes);
            tile_table.push(tile);
        }

        for row in 0..TILE_HEIGHT {
            for col in 0..TILE_TABLE_WIDTH {
                let tile_index = row * TILE_TABLE_PER_ROW + col;

                if tile_index < tile_table.len() {
                    let tile = &tile_table[tile_index];

                    let x = col * TILE_WIDTH;
                    let y = row * TILE_HEIGHT;

                    for (tile_row, row_pixels) in tile.data.iter().enumerate() {
                        for (tile_col, pixel) in row_pixels.iter().enumerate() {
                            let color = match *pixel {
                                WHITE => WHITE,
                                LIGHT => LIGHT,
                                DARK => DARK,
                                BLACK => BLACK,
                                _ => unreachable!(),
                            };

                            tile_data_canvas.set_draw_color(color);

                            tile_data_canvas
                                .draw_point(Point::new(
                                    x as i32 + tile_col as i32,
                                    y as i32 + tile_row as i32,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

}
