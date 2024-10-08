#![cfg_attr(rustfmt, rustfmt::skip)]
/*
 * @file    registers/flags_register.rs
 * @brief   Handles CPU flag registers.
 * @author  Mario Hess
 * @date    May 30, 2024
 */

use crate::cpu::instruction::Flag;

const ZERO_MASK: u8 = 0x80;
const SUBTRACT_MASK: u8 = 0x40;
const HALF_CARRY_MASK: u8 = 0x20;
const CARRY_MASK: u8 = 0x10;

pub struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Flags {
    pub fn new(enable_flags: bool) -> Self {
        Self {
            zero: true,
            subtract: false,
            half_carry: enable_flags,
            carry: enable_flags,
        }
    }

    pub fn get_zero(&self) -> bool { self.zero }
    pub fn set_zero(&mut self, should_set: bool) { self.zero = should_set; }

    pub fn get_subtract(&self) -> bool { self.subtract }
    pub fn set_subtract(&mut self, should_set: bool) { self.subtract = should_set; }

    pub fn get_half_carry(&self) -> bool { self.half_carry }
    pub fn set_half_carry(&mut self, should_set: bool) { self.half_carry = should_set; }

    pub fn get_carry(&self) -> bool { self.carry }
    pub fn set_carry(&mut self, should_set: bool) { self.carry = should_set; }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.get_zero(),
            Flag::N => self.get_subtract(),
            Flag::H => self.get_half_carry(),
            Flag::C => self.get_carry(),
        }
    }
}

impl std::convert::From<&Flags> for u8 {
    fn from(flags: &Flags) -> u8 {
        (if flags.zero { ZERO_MASK } else { 0 })
            | (if flags.subtract { SUBTRACT_MASK } else { 0 })
            | (if flags.half_carry { HALF_CARRY_MASK } else { 0 })
            | (if flags.carry { CARRY_MASK } else { 0 })
    }
}

impl std::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Self {
            zero: (byte & ZERO_MASK) != 0,
            subtract: (byte & SUBTRACT_MASK) != 0,
            half_carry: (byte & HALF_CARRY_MASK) != 0,
            carry: (byte & CARRY_MASK) != 0,
        }
    }
}
