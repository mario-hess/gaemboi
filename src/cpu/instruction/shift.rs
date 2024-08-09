/**
 * @file    cpu/shift.rs
 * @brief   Implementation of shift instructions.
 * @author  Mario Hess
 * @date    May 26, 2024
 */
use crate::cpu::{
    instruction::{CycleDuration, Target},
    Cpu, MemoryAccess,
};

pub fn srl_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Shifts all the bits of the register to the
    // right by one position

    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & 0b0000_0001) != 0;
    let result = r >> 1;

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn srl_hl(cpu: &mut Cpu) -> CycleDuration {
    // Shifts all the bits of the byte pointed to by HL
    // to the right by one position

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let shifted_out = (byte & 0b0000_0001) != 0;
    let result = byte >> 1;

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn sla_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Shift target register to the left arithmetically

    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & 0b1000_0000) != 0;
    let result = r << 1;

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn sla_hl(cpu: &mut Cpu) -> CycleDuration {
    // Shift the byte pointed to by HL to the left arithmetically

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let shifted_out = (byte & 0b1000_0000) != 0;
    let result = byte << 1;

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn sra_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Shift target register to the right arithmetically

    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & 0x01) != 0;
    let result = (r >> 1) | (r & 0b1000_0000);

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn sra_hl(cpu: &mut Cpu) -> CycleDuration {
    // Shift the byte pointed to by HL to the right arithmetically

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let shifted_out = (byte & 0x01) != 0;
    let result = (byte >> 1) | (byte & 0b1000_0000);

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

pub fn swap_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Swap the upper 4 bits in the target register and the lower 4 ones

    let r = cpu.registers.get_register(&target);

    let result = (r >> 4) | (r << 4);

    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn swap_hl(cpu: &mut Cpu) -> CycleDuration {
    // Swap the upper 4 bits in the byte pointed by HL and the lower 4 ones

    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let result = (byte >> 4) | (byte << 4);

    cpu.memory_bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}
