use crate::{
    bus::Bus,
    cpu::{
        instruction::{CycleDuration, Target},
        Cpu,
    },
    utils::constants::{BIT_0_MASK, BIT_7_MASK},
};

// Rotate the contents of the 8-bit A register to the right by one bit.
// The bit that is rotated out from the right side is moved to the
// leftmost position, and the carry flag (C) is set to the byte
// of the bit that was rotated out
pub fn rrca(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();

    let (shifted_out, result) = rotate_right(a);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate register A to the right through carry
pub fn rra(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_right_with_carry(a, carry);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the contents of the 8-bit A register to the left by one bit.
// The bit that is rotated out from the left side is moved to the
// rightmost position, and the carry flag (C) is set to the byte
// of the bit that was rotated out
pub fn rlca(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();

    let (shifted_out, result) = rotate_left(a);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate register A to the left through carry
pub fn rla(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_left_with_carry(a, carry);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the target register to the left
pub fn rlc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let (shifted_out, result) = rotate_left(r);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the byte pointed to by HL to the left
pub fn rlc_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let (shifted_out, result) = rotate_left(byte);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate target register to the right
pub fn rrc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let (shifted_out, result) = rotate_right(r);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the byte pointed to by HL to the right
pub fn rrc_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);

    let (shifted_out, result) = rotate_right(byte);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate bits in the target register to the left through carry
pub fn rl_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_left_with_carry(r, carry);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate bits in the byte pointed to by HL to the left through carry
pub fn rl_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_left_with_carry(byte, carry);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate target register to the right through carry
pub fn rr_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_right_with_carry(r, carry);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the byte pointed to by HL to the right through carry
pub fn rr_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.registers.get_hl();
    let byte = bus.read_byte(address);
    let carry = cpu.registers.flags.get_carry();

    let (shifted_out, result) = rotate_right_with_carry(byte, carry);
    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

fn rotate_left(byte: u8) -> (bool, u8) {
    let shifted_out = byte & BIT_7_MASK != 0;
    let result = byte.rotate_left(1);

    (shifted_out, result)
}

fn rotate_left_with_carry(byte: u8, carry: bool) -> (bool, u8) {
    let shifted_out = byte & BIT_7_MASK != 0;
    let result = (byte << 1) | (carry as u8);

    (shifted_out, result)
}

fn rotate_right(byte: u8) -> (bool, u8) {
    let shifted_out = byte & BIT_0_MASK != 0;
    let result = byte.rotate_right(1);

    (shifted_out, result)
}

fn rotate_right_with_carry(byte: u8, carry: bool) -> (bool, u8) {
    let shifted_out = byte & BIT_0_MASK != 0;
    let result = (byte >> 1) | ((carry as u8) << 7);

    (shifted_out, result)
}
