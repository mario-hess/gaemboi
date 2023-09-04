use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn res_b_r(cpu: &mut Cpu, bit: u8, target: Target) {
    // clear bit of the target register

    let reg = cpu.registers.get_register(&target);
    let result = reg & !(1 << bit);

    cpu.registers.set_register(target, result);
}
