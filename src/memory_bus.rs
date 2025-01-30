/*
 * @file    memory_bus.rs
 * @brief   Manages memory access and address decoding.
 * @author  Mario Hess
 * @date    May 28, 2024
 */

use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{
    apu::{Apu, AUDIO_END, AUDIO_START},
    cartridge::Cartridge,
    io::{joypad::Joypad, timer::Timer},
    ppu::{colors::Colors, Ppu},
};

pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x7FFF;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

pub const CARTRIDGE_RAM_START: u16 = 0xA000;
pub const CARTRIDGE_RAM_END: u16 = 0xBFFF;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;

const NOT_USABLE_START: u16 = 0xFEA0;
const NOT_USABLE_END: u16 = 0xFEFF;

const JOYPAD_INPUT: u16 = 0xFF00;
const SERIAL_SB: u16 = 0xFF01;
const SERIAL_SC: u16 = 0xFF02;

const TIMER_START: u16 = 0xFF04;
const TIMER_END: u16 = 0xFF07;

const INTERRUPT_FLAG: u16 = 0xFF0F;

const PPU_IO_START: u16 = 0xFF40;
const LINE_Y_COMPARE: u16 = 0xFF45;
const DMA: u16 = 0xFF46;
const BG_PALETTE: u16 = 0xFF47;
const PPU_IO_END: u16 = 0xFF4B;

const SPEED_SWITCH: u16 = 0xFF4D;
const CGB_VRAM_SELECT: u16 = 0xFF4F;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE: u16 = 0xFFFF;

pub trait MemoryAccess {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

pub trait ComponentTick {
    fn tick(&mut self, m_cycles: u8);
}

pub struct MemoryBus {
    cartridge: Cartridge,
    pub ppu: Ppu,
    pub apu: Apu,
    wram: [u8; 8192],
    hram: [u8; 128],
    pub interrupt_enabled: u8,
    pub interrupt_flag: u8,
    pub timer: Timer,
    pub joypad: Joypad,
    serial_sb: u8,
    serial_sc: u8,
    speed_switch: u8,
}

impl MemoryAccess for MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 0x0000 - 0x7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.read_byte(address),
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.read_byte(address),
            // 0xA000 - 0xBFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.read_byte(address),
            // 0xC000 - 0xDFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize],
            // 0xE000 - 0xFDFF (Echo Ram)
            ECHO_RAM_START..=ECHO_RAM_END => self.wram[address as usize - ECHO_RAM_START as usize],
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.read_byte(address),
            // 0xFEA0 - 0xFEFF
            NOT_USABLE_START..=NOT_USABLE_END => 0,
            // 0xFF00 (Joypad)
            JOYPAD_INPUT => self.joypad.get(),
            // 0xFF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb,
            // 0xFF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc,
            // 0xFF04 - 0xFF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.read_byte(address),
            // 0xFF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag,
            // 0xFF10 - 0xFF3F (APU)
            AUDIO_START..=AUDIO_END => self.apu.read_byte(address),
            // 0xFF40 - 0xFF45 (PPU Registers)
            PPU_IO_START..=LINE_Y_COMPARE => self.ppu.read_byte(address),
            // 0xFF46 DMA Transfer (Write Only)
            DMA => 0,
            // 0xFF47 - 0xFF4B (PPU Registers)
            BG_PALETTE..=PPU_IO_END => self.ppu.read_byte(address),
            // 0xFF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch,
            // 0xFF4F (CGB VRAM Select)
            CGB_VRAM_SELECT => 0xFF,
            // 0xFF80 - 0xFFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize],
            // 0xFFFF (Interrupt Enable Register)
            INTERRUPT_ENABLE => self.interrupt_enabled,
            _ => {
                eprintln!(
                    "Memory Bus Unknown address: {:#X} Can't read byte.",
                    address
                );

                0xFF
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // 0x0000 - 0x7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.write_byte(address, value),
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.write_byte(address, value),
            // 0xA000 - 0xBFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.write_byte(address, value),
            // 0xC000 - 0xDFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize] = value,
            // 0xE000 - 0xFDFF (Echo Ram)
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.wram[address as usize - ECHO_RAM_START as usize] = value
            }
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.write_byte(address, value),
            // 0xFEA0 - 0xFEFF
            NOT_USABLE_START..=NOT_USABLE_END => {}
            // 0xFF00 (Joypad)
            JOYPAD_INPUT => self.joypad.set(value),
            // 0xFF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb = value,
            // 0xFF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc = value,
            // 0xFF04 - 0xFF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.write_byte(address, value),
            // 0xFF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag = value,
            // 0xFF10 - 0xFF3F (APU)
            AUDIO_START..=AUDIO_END => self.apu.write_byte(address, value),
            // 0xFF40 - 0xFF45 (PPU Registers)
            PPU_IO_START..=LINE_Y_COMPARE => self.ppu.write_byte(address, value),
            // 0xFF46 DMA Transfer (Write Only)
            DMA => self.dma_transfer(value),
            // 0xFF47 - 0xFF4B (PPU Registers)
            BG_PALETTE..=PPU_IO_END => self.ppu.write_byte(address, value),
            // 0xFF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch = value,
            // 0xFF4F (CGB VRAM Select)
            CGB_VRAM_SELECT => {}
            // 0xFF80 - 0xFFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize] = value,
            // 0xFFFF (Interrupt Enable Register)
            INTERRUPT_ENABLE => self.interrupt_enabled = value,
            _ => eprintln!(
                "Memory Bus Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}

impl ComponentTick for MemoryBus {
    fn tick(&mut self, m_cycles: u8) {
        self.timer.tick(m_cycles);
        self.interrupt_flag |= self.timer.interrupt;
        self.timer.reset_interrupt();

        self.ppu.tick(m_cycles);
        self.interrupt_flag |= self.ppu.interrupts;
        self.ppu.reset_interrupts();

        self.apu.tick(m_cycles);
    }
}

impl MemoryBus {
    pub fn new(
        rom_data: Vec<u8>,
        colors: Rc<RefCell<Colors>>,
        fast_forward: Rc<RefCell<u32>>,
    ) -> Result<Self, Box<dyn Error>> {
        let cartridge = Cartridge::build(rom_data)?;

        Ok(Self {
            cartridge,
            ppu: Ppu::new(colors),
            apu: Apu::new(fast_forward),
            wram: [0; 8192],
            hram: [0; 128],
            interrupt_enabled: 0x00,
            interrupt_flag: 0xE1,
            joypad: Joypad::default(),
            serial_sb: 0x00,
            serial_sc: 0x00,
            timer: Timer::new(),
            speed_switch: 0x00,
        })
    }

    pub fn get_interrupt_flag(&mut self) -> u8 {
        self.interrupt_flag
    }

    pub fn get_interrupt_enabled(&mut self) -> u8 {
        self.interrupt_enabled
    }

    fn dma_transfer(&mut self, value: u8) {
        let byte = (value as u16) << 8;
        for i in 0..160 {
            let new_byte = self.read_byte(byte + i);
            self.write_byte(OAM_START + i, new_byte);
        }
    }

    pub fn load_game(&mut self, ram_data: Vec<u8>) {
        self.cartridge.load_game(ram_data);
    }

    pub fn save_game(&self, file_path: &str) {
        match self.cartridge.save_game(file_path) {
            Ok(_) => println!("Game saved."),
            Err(e) => eprintln!("Error saving game: {e}."),
        }
    }
}
