/*
 * @file    cpu/bit_ops.rs
 * @brief   Implementation of bit operation instructions.
 * @author  Mario Hess
 * @date    May 26, 2024
 */

use crate::cpu::{
    instruction::{CycleDuration, Target},
    Cpu, MemoryAccess,
};

// Test bit at position in target register,
// set the zero flag if bit not set
pub fn bit_r(cpu: &mut Cpu, position: u8, target: Target) -> CycleDuration {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = 1 << position;
    let result = byte & bitmask;
    let is_set = result != 0;

    cpu.registers.flags.set_zero(!is_set);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);

    CycleDuration::Default
}

// Test bit at position in the byte pointed by HL,
// set the zero flag if bit not set
pub fn bit_hl(cpu: &mut Cpu, position: u8) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = 1 << position;
    let result = byte & bitmask;
    let is_set = result != 0;

    cpu.registers.flags.set_zero(!is_set);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);

    CycleDuration::Default
}

// Set bit at position in target register to 0
pub fn res_r(cpu: &mut Cpu, position: u8, target: Target) -> CycleDuration {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = !(1 << position);
    let result = byte & bitmask;

    cpu.registers.set_register(target, result);

    CycleDuration::Default
}

// Set bit at position in the byte pointed by HL to 0
pub fn res_hl(cpu: &mut Cpu, position: u8) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = !(1 << position);
    let result = byte & bitmask;

    cpu.memory_bus.write_byte(address, result);

    CycleDuration::Default
}

// Set bit at position in target register to 1
pub fn set_r(cpu: &mut Cpu, position: u8, target: Target) -> CycleDuration {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = 1 << position;
    let result = byte | bitmask;

    cpu.registers.set_register(target, result);

    CycleDuration::Default
}

// Set bit at position in the byte pointed by HL to 1
pub fn set_hl(cpu: &mut Cpu, position: u8) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = 1 << position;
    let result = byte | bitmask;

    cpu.memory_bus.write_byte(address, result);

    CycleDuration::Default
}
