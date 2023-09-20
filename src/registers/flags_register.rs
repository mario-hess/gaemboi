/**
 * @file    registers/flags_register.rs
 * @brief   Handles CPU flag registers.
 * @author  Mario Hess
 * @date    September 20, 2023
 */
use crate::instruction::Flag;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Copy, Clone)]
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl FlagsRegister {
    pub fn new(enable_flags: bool) -> Self {
        Self {
            zero: true,
            subtract: false,
            half_carry: enable_flags,
            carry: enable_flags,
        }
    }

    pub fn set_zero(&mut self, should_set: bool) {
        self.zero = should_set;
    }

    pub fn get_zero(&self) -> bool {
        self.zero
    }

    pub fn set_subtract(&mut self, should_set: bool) {
        self.subtract = should_set;
    }

    pub fn get_subtract(&self) -> bool {
        self.subtract
    }

    pub fn set_half_carry(&mut self, should_set: bool) {
        self.half_carry = should_set;
    }

    pub fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn set_carry(&mut self, should_set: bool) {
        self.carry = should_set;
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.get_zero(),
            Flag::N => self.get_subtract(),
            Flag::H => self.get_half_carry(),
            Flag::C => self.get_carry(),
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
