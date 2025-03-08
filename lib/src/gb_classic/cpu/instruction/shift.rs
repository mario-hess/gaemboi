use crate::{
    gb_classic::{
        bus::Bus,
        cpu::{
            instruction::{CycleDuration, Target},
            Cpu,
        },
    },
    utils::constants::{BIT_0_MASK, BIT_7_MASK},
};

// Shifts all the bits of the register to the
// right by one position
pub fn srl_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & BIT_0_MASK) != 0;
    let result = r >> 1;
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Shifts all the bits of the byte pointed to by HL
// to the right by one position
pub fn srl_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let shifted_out = (byte & BIT_0_MASK) != 0;
    let result = byte >> 1;
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Shift target register to the left arithmetically
pub fn sla_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & BIT_7_MASK) != 0;
    let result = r << 1;

    cpu.registers.set_register(target, result);
    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Shift the byte pointed to by HL to the left arithmetically
pub fn sla_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let shifted_out = (byte & BIT_7_MASK) != 0;
    let result = byte << 1;
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Shift target register to the right arithmetically
pub fn sra_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let shifted_out = (r & BIT_0_MASK) != 0;
    let result = (r >> 1) | (r & BIT_7_MASK);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Shift the byte pointed to by HL to the right arithmetically
pub fn sra_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let shifted_out = (byte & BIT_0_MASK) != 0;
    let result = (byte >> 1) | (byte & BIT_7_MASK);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Swap the upper 4 bits in the target register and the lower 4 ones
pub fn swap_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let result = r.rotate_left(4);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Swap the upper 4 bits in the byte pointed by HL and the lower 4 ones
pub fn swap_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let result = byte.rotate_left(4);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}
