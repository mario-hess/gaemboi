use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn bit_r(cpu: &mut Cpu, position: u8, target: Target) {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = 1 << position;
    let result = byte & bitmask;
    let is_set = result != 0;

    cpu.registers.f.set_zero(!is_set);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(true);
}

pub fn bit_hl(cpu: &mut Cpu, position: u8) {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = 1 << position;
    let result = byte & bitmask;
    let is_set = result != 0;

    cpu.registers.f.set_zero(!is_set);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry(true);
}

pub fn res_r(cpu: &mut Cpu, position: u8, target: Target) {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = !(1 << position);
    let result = byte & bitmask;

    cpu.registers.set_register(target, result);
}

pub fn res_hl(cpu: &mut Cpu, position: u8) {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = !(1 << position);
    let result = byte & bitmask;

    cpu.memory_bus.write_byte(address, result);
}

pub fn set_r(cpu: &mut Cpu, position: u8, target: Target) {
    let byte = cpu.registers.get_register(&target);

    let bitmask: u8 = 1 << position;
    let result = byte | bitmask;

    cpu.registers.set_register(target, result);
}

pub fn set_hl(cpu: &mut Cpu, position: u8) {
    let address = cpu.registers.get_hl();
    let byte = cpu.memory_bus.read_byte(address);

    let bitmask: u8 = 1 << position;
    let result = byte | bitmask;

    cpu.memory_bus.write_byte(address, result);
}
