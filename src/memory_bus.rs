/**
 * @file    memory_bus.rs
 * @brief   Manages memory access and address decoding.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use crate::cartridge::Cartridge;
use crate::interrupt::VBLANK_MASK;
use crate::ppu::Ppu;
use crate::timer::Timer;

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
            interrupt_flag: 0xE1,
            audio: [0; 23],
            wave_pattern: [0; 16],
            speed_switch: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        self.timer.tick(m_cycles);
        self.interrupt_flag |= self.timer.interrupt_request;
        self.timer.reset_interrupt();

        self.ppu.tick(m_cycles);
        self.interrupt_flag |= self.ppu.interrupts;
        self.ppu.reset_interrupt(VBLANK_MASK);
    }

    pub fn get_interrupt_flag(&mut self) -> u8 {
        self.interrupt_flag
    }

    pub fn get_interrupt_enable(&mut self) -> u8 {
        self.interrupt_enable
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            // 0x0000 - 0x7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.read(address),
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.read_byte(address),
            // 0xA000 - 0xBFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.read(address),
            // 0xC000 - 0xDFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize],
            // 0xE000 - 0xFDFF (Echo Ram)
            ECHO_RAM_START..=ECHO_RAM_END => self.wram[address as usize - ECHO_RAM_START as usize],
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.read_byte(address),
            // 0xFEA0 - 0xFEFF
            NOT_USABLE_START..=NOT_USABLE_END => 0,
            // 0xFF00 (Joypad)
            JOYPAD_INPUT => self.joypad_input,
            // 0xFF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb,
            // 0xFF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc,
            // 0xFF04 - 0xFF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.read_byte(address),
            // 0xFF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag,
            // 0xFF10 - 0xFF26 (Audio Channel Control)
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize],
            // 0xFF30 - 0xFF3F (Audio Wave storage)
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize]
            }
            // 0xFF40 - 0xFF4B (PPU Registers)
            PPU_IO_START..=PPU_IO_END => self.ppu.read_byte(address),
            // 0xFF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch,
            // 0xFF80 - 0xFFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize],
            // 0xFFFF (Interrupt Enable Register)
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
            // 0x0000 - 0x7FFF (Cartridge ROM Banks)
            CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END => self.cartridge.write(address, value),
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.ppu.write_byte(address, value),
            // 0xA000 - 0xBFFF (Cartridge RAM Banks)
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.write(address, value),
            // 0xC000 - 0xDFFF (Work RAM)
            WRAM_START..=WRAM_END => self.wram[address as usize - WRAM_START as usize] = value,
            // 0xE000 - 0xFDFF (Echo Ram)
            ECHO_RAM_START..=ECHO_RAM_END => self.wram[address as usize - ECHO_RAM_START as usize] = value,
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.ppu.write_byte(address, value),
            // 0xFEA0 - 0xFEFF
            NOT_USABLE_START..=NOT_USABLE_END => {},
            // 0xFF00 (Joypad)
            JOYPAD_INPUT => self.joypad_input = value,
            // 0xFF01 (Serial transfer data)
            SERIAL_SB => self.serial_sb = value,
            // 0xFF02 (Serial transfer control)
            SERIAL_SC => self.serial_sc = value,
            // 0xFF04 - 0xFF07 (Timer Registers)
            TIMER_START..=TIMER_END => self.timer.write_byte(address, value),
            // 0xFF0F (Interrupt Flag Register)
            INTERRUPT_FLAG => self.interrupt_flag = value,
            // 0xFF10 - 0xFF26 (Audio Channel Control)
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize] = value,
            // 0xFF30 - 0xFF3F (Audio Wave storage)
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize] = value;
            }
            // 0xFF40 - 0xFF4B (PPU Registers)
            PPU_IO_START..=PPU_IO_END => self.ppu.write_byte(address, value),
            // 0xFF4D (Speed Switch)
            SPEED_SWITCH => self.speed_switch = value,
            // 0xFF80 - 0xFFFE (High RAM)
            HRAM_START..=HRAM_END => self.hram[address as usize - HRAM_START as usize] = value,
            // 0xFFFF (Interrupt Enable Register)
            INTERRUPT_ENABLE => self.interrupt_enable = value,
            _ => println!(
                "Unknown address: {:#X} Can't write byte: {:#X}.",
                address, value
            ),
        }
    }
}
