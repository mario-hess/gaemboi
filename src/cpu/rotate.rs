/**
 * @file    cpu/rotate.rs
 * @brief   Implementation of rotate instructions.
 * @author  Mario Hess
 * @date    May 26, 2024
 */
use crate::cpu::{
    instruction::{CycleDuration, Target},
    Cpu,
};

pub fn rrca(cpu: &mut Cpu) -> CycleDuration {
    // Rotate the contents of the 8-bit A register to the right by one bit.
    // The bit that is rotated out from the right side is moved to the
    // leftmost position, and the carry flag (C) is set to the value
    // of the bit that was rotated out

    let a = cpu.registers.get_a();
    let shifted_out = (a & 0x01) != 0;
    let result = (a >> 1) | (a << 7);

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rra(cpu: &mut Cpu) -> CycleDuration {
    // Rotate register A to the right through carry

    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (a & 0x01) != 0;
    let result = (a >> 1) | (carry << 7);

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rlca(cpu: &mut Cpu) -> CycleDuration {
    // Rotate the contents of the 8-bit A register to the left by one bit.
    // The bit that is rotated out from the left side is moved to the
    // rightmost position, and the carry flag (C) is set to the value
    // of the bit that was rotated out

    let a = cpu.registers.get_a();
    let shifted_out = (a & 0b1000_0000) != 0;
    let result = (a << 1) | (a >> 7);

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rla(cpu: &mut Cpu) -> CycleDuration {
    // Rotate register A to the left through carry

    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (a & 0b1000_0000) != 0;
    let result = (a << 1) | carry;

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rlc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Rotate the target register to the left

    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & 0b1000_0000) != 0;
    let result = (r << 1) | (r >> 7);

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rlc_hl(cpu: &mut Cpu) -> CycleDuration {
    // Rotate the byte pointed to by HL to the left

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let shifted_out = (byte & 0b1000_0000) != 0;
    let result = (byte << 1) | (byte >> 7);

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rrc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Rotate target register to the right

    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & 0x01) != 0;
    let result = (r >> 1) | (r << 7);

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rrc_hl(cpu: &mut Cpu) -> CycleDuration {
    // Rotate the byte pointed to by HL to the right

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let shifted_out = (byte & 0x01) != 0;
    let result = (byte >> 1) | (byte << 7);

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rl_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Rotate bits in the target register to the left through carry

    let r = cpu.registers.get_register(&target);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (r & 0b1000_0000) != 0;
    let result = (r << 1) | carry;

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rl_hl(cpu: &mut Cpu) -> CycleDuration {
    // Rotate bits in the byte pointed to by HL to the left through carry

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (byte & 0b1000_0000) != 0;
    let result = (byte << 1) | carry;

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rr_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Rotate target register to the right through carry

    let r = cpu.registers.get_register(&target);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (r & 0x01) != 0;
    let result = (r >> 1) | (carry << 7);

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn rr_hl(cpu: &mut Cpu) -> CycleDuration {
    // Rotate the byte pointed to by HL to the right through carry

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (byte & 0x01) != 0;
    let result = (byte >> 1) | (carry << 7);

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}
