use crate::{cpu::Cpu, instruction::Target};

pub fn rrca(cpu: &mut Cpu) {
    // rotate the contents of the 8-bit A register to the right by one bit.
    // The bit that is rotated out from the right side is moved to the
    // leftmost position, and the carry flag (C) is set to the value
    // of the bit that was rotated out.

    let a = cpu.registers.get_a();
    let shifted_out = (a & 0x01) != 0;
    let result = (a >> 1) | (a << 7);

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(false, false, false, shifted_out);
}

pub fn rra(cpu: &mut Cpu) {
    // rotate A right through carry

    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.f.get_carry().into();

    let shifted_out = (a & 0x01) != 0;
    let result = (a >> 1) | (carry << 7);

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(false, false, false, shifted_out);
}

pub fn rlca(cpu: &mut Cpu) {
    // rotate the contents of the 8-bit A register to the left by one bit.
    // The bit that is rotated out from the left side is moved to the
    // rightmost position, and the carry flag (C) is set to the value
    // of the bit that was rotated out.

    let a = cpu.registers.get_a();
    let shifted_out = (a & 0b1000_0000) != 0;
    let result = (a << 1) | (a >> 7);

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(false, false, false, shifted_out);
}

pub fn rla(cpu: &mut Cpu) {
    // Rotate register A left through carry
    
    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.f.get_carry().into();

    let shifted_out = (a & 0b1000_0000) != 0;
    let result = (a << 1) | (carry >> 7);
    
    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(false, false, false, shifted_out);
}

pub fn rr_r(cpu: &mut Cpu, target: Target) {
    // rotate target right through carry

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);
    let carry: u8 = cpu.registers.f.get_carry().into();

    let result = (r >> 1) | (carry << 7);
    let new_carry = (r & 0x01) != 0;

    cpu.registers
        .f
        .set_flags(result == 0, false, false, new_carry);
    set_r(&mut cpu.registers, result);
}
