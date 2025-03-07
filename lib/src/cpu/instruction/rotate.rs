use crate::{
    bus::Bus,
    cpu::{
        instruction::{CycleDuration, Target},
        Cpu,
    },
};

const MSB: u8 = 0x80;
const LSB: u8 = 0x01;

// Rotate the contents of the 8-bit A register to the right by one bit.
// The bit that is rotated out from the right side is moved to the
// leftmost position, and the carry flag (C) is set to the value
// of the bit that was rotated out
pub fn rrca(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();
    let shifted_out = (a & LSB) != 0;
    let result = (a >> 1) | (a << 7);

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (a & LSB) != 0;
    let result = (a >> 1) | (carry << 7);

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}

// Rotate the contents of the 8-bit A register to the left by one bit.
// The bit that is rotated out from the left side is moved to the
// rightmost position, and the carry flag (C) is set to the value
// of the bit that was rotated out
pub fn rlca(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();
    let shifted_out = (a & MSB) != 0;
    let result = (a << 1) | (a >> 7);

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (a & MSB) != 0;
    let result = (a << 1) | carry;

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

    let shifted_out = (r & MSB) != 0;
    let result = (r << 1) | (r >> 7);

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

    let shifted_out = (byte & MSB) != 0;
    let result = (byte << 1) | (byte >> 7);

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

    let shifted_out = (r & LSB) != 0;
    let result = (r >> 1) | (r << 7);

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

    let shifted_out = (byte & LSB) != 0;
    let result = (byte >> 1) | (byte << 7);

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (r & MSB) != 0;
    let result = (r << 1) | carry;

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (byte & MSB) != 0;
    let result = (byte << 1) | carry;

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (r & LSB) != 0;
    let result = (r >> 1) | (carry << 7);

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
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let shifted_out = (byte & LSB) != 0;
    let result = (byte >> 1) | (carry << 7);

    bus.write_byte(address, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(shifted_out);

    CycleDuration::Default
}
