/**
 * @file    registers/mod.rs
 * @brief   Module for handling CPU registers.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
mod flags_register;

use crate::instruction::Target;
use crate::registers::flags_register::FlagsRegister;

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    pub flags: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new(flags_enable: bool) -> Self {
        // Registers are set to skip the power-up sequence,
        // as the copyrighted boot rom can't be included.

        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            flags: FlagsRegister::new(flags_enable),
            h: 0x01,
            l: 0x4D,
        }
    }

    pub fn get_af(&self) -> u16 {
        let f: u8 = self.flags.into();
        (self.a as u16) << 8 | f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.flags = ((value & 0xFF) as u8).into();
    }
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn get_f(&self) -> u8 {
        let f: u8 = self.flags.into();
        f
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn get_register(&self, target: &Target) -> u8 {
        match target {
            Target::A => self.get_a(),
            Target::B => self.get_b(),
            Target::C => self.get_c(),
            Target::D => self.get_d(),
            Target::E => self.get_e(),
            Target::H => self.get_h(),
            Target::L => self.get_l(),
            _ => unreachable!(),
        }
    }

    pub fn get_pair(&self, target: &Target) -> u16 {
        match target {
            Target::AF => self.get_af(),
            Target::BC => self.get_bc(),
            Target::DE => self.get_de(),
            Target::HL => self.get_hl(),
            _ => unreachable!(),
        }
    }

    pub fn set_register(&mut self, target: Target, value: u8) {
        match target {
            Target::A => self.set_a(value),
            Target::B => self.set_b(value),
            Target::C => self.set_c(value),
            Target::D => self.set_d(value),
            Target::E => self.set_e(value),
            Target::H => self.set_h(value),
            Target::L => self.set_l(value),
            _ => unreachable!(),
        }
    }

    pub fn set_pair(&mut self, target: Target, value: u16) {
        match target {
            Target::AF => self.set_af(value),
            Target::BC => self.set_bc(value),
            Target::DE => self.set_de(value),
            Target::HL => self.set_hl(value),
            _ => unreachable!(),
        }
    }
}
