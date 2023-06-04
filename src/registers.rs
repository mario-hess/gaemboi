use crate::flags_register::FlagsRegister;
use crate::instruction::ArithmeticTarget;

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    pub f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new(enable_flags: bool) -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: FlagsRegister::new(enable_flags),
            h: 0x01,
            l: 0x4D,
        }
    }

    pub fn get_af(self) -> u16 {
        let f: u8 = self.f.into();
        (self.a as u16) << 8 | f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0xFF) as u8).into();
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

    pub fn get_register_value(&self, reg_target: &ArithmeticTarget) -> u8 {
        match reg_target {
            ArithmeticTarget::A => self.get_a(),
            ArithmeticTarget::B => self.get_b(),
            ArithmeticTarget::C => self.get_c(),
            ArithmeticTarget::D => self.get_d(),
            ArithmeticTarget::E => self.get_e(),
            ArithmeticTarget::H => self.get_h(),
            ArithmeticTarget::L => self.get_l(),
            _ => unreachable!(),
        }
    }

    pub fn get_pair_value(&self, pair_target: &ArithmeticTarget) -> u16 {
        match pair_target {
            ArithmeticTarget::BC => self.get_bc(),
            ArithmeticTarget::DE => self.get_de(),
            ArithmeticTarget::HL => self.get_hl(),
            _ => unreachable!(),
        }
    }
}
