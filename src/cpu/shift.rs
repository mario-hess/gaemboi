use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn srl_r(cpu: &mut Cpu, target: Target) {
    // shifts all the bits of the register to the right by one position

    let r = cpu.registers.get_register_value(&target);
    let result = r >> 1;

    let carry = (r & 0b0000_0001) != 0;

    cpu.registers.f.set_flags(result == 0, false, false, carry);
}
