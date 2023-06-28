use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn srl_r(cpu: &mut Cpu, target: Target) {
    // shifts all the bits of the register to the right by one position

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);
    let result = r >> 1;

    set_r(&mut cpu.registers, result);

    let carry = (r & 0b0000_0001) != 0;

    cpu.registers.f.set_flags(result == 0, false, false, carry);
}

pub fn sla_r(cpu: &mut Cpu, target: Target) {
    // Shift Left Arithmetically register r8

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);

    let shifted_out = (r & 0b1000_0000) != 0;
    let result = r << 1;

    set_r(&mut cpu.registers, result);
    cpu.registers
        .f
        .set_flags(result == 0, false, false, shifted_out);
}

pub fn sra_r(cpu: &mut Cpu, target: Target) {

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);

    let shifted_out = (r & 0x01) != 0;
    let result = (r >> 1) | (r & 0b1000_0000);

    set_r(&mut cpu.registers, result);
    cpu.registers.f.set_flags(result == 0, false, false, shifted_out);
}

pub fn swap_r(cpu: &mut Cpu, target: Target) {
    // Swap the upper 4 bits in register r8 and the lower 4 ones

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);

    let result = (r >> 4) | (r << 4);
    set_r(&mut cpu.registers, result);
}
