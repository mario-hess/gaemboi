use crate::cpu::Cpu;

pub fn rlca(cpu: &mut Cpu) {
    // rotate the contents of the 8-bit A register to the left by one bit.
    // The bit that is rotated out from the left side is moved to the
    // rightmost position, and the carry flag (C) is set to the value
    // of the bit that was rotated out.

    let a = cpu.registers.get_a();
    let shifted_out = (a & 0b1000_0000) != 0;
    let rotated = (a << 1) | (a >> 7);

    cpu.registers.set_a(rotated);

    cpu.registers.f.set_zero(false);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_carry(shifted_out);
}
