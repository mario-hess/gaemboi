use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn add_reg(cpu: &mut Cpu, target: Target) {
    // Adds to the 8-bit A register, the 8-bit register r,
    // and stores the result back into the A register

    let r = cpu.registers.get_register_value(&target);
    let a = cpu.registers.get_a();

    let result = a.wrapping_add(r);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
    cpu.registers.f.set_carry(a < r);
}

pub fn sub_n(cpu: &mut Cpu) {
    // Subtracts from the 8-bit A register, the
    // immediate data n, and stores the result
    // back into the A register.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();
    let result = a.wrapping_sub(n);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
    cpu.registers.f.set_carry(a < n);
}

pub fn and_n(cpu: &mut Cpu) {
    // Performs a bitwise AND operation between the
    // 8-bit A register and immediate data n, and
    // stores the result back into the A register

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();
    let result = a & n;
    cpu.registers.set_a(result);

    cpu.registers.f.set_flags(result == 0, false, true, false);
}

pub fn inc_reg(cpu: &mut Cpu, target: Target) {
    // Increments data in the 8-bit register r

    let reg = cpu.registers.get_register_value(&target);
    let set_reg = cpu.registers.get_register_setter(&target);

    let result = reg.wrapping_add(1);
    set_reg(&mut cpu.registers, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
}

pub fn inc_pair(cpu: &mut Cpu, target: Target) {
    // Increments data in the 16-bit target register by 1

    let value = cpu.registers.get_pair_value(&target);
    let set_reg = cpu.registers.get_pair_setter(&target);
    set_reg(&mut cpu.registers, value.wrapping_add(1));
}

pub fn dec_pair(cpu: &mut Cpu, target: Target) {
    // Decrements data in the 16-bittarget register

    let reg = cpu.registers.get_pair_value(&target);
    let set_reg = cpu.registers.get_pair_setter(&target);

    let result = reg.wrapping_sub(1);
    set_reg(&mut cpu.registers, result);
}

// --- OR / XOR instructions ---
pub fn or_reg(cpu: &mut Cpu, target: Target) {
    // Performs a bitwise OR operation between the 8-bit
    // A register and the 8-bit register r, and stores
    // the result back into the A register

    let r = cpu.registers.get_register_value(&target);
    let a = cpu.registers.get_a();

    let result = a | r;

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(result == 0, false, false, false);
}

pub fn xor_reg(cpu: &mut Cpu, target: Target) {
    // Performs a bitwise XOR operation between the
    // 8-bit A register and the 8-bit target register,
    // and stores the result back into the A register

    let a = cpu.registers.get_a();
    let value = cpu.registers.get_register_value(&target);

    let result = a ^ value;
    let flag = result == 0;

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(flag, false, false, false);
}

pub fn cp_n(cpu: &mut Cpu) {
    // Subtracts from the 8-bit A register, the immediate
    // data n, and updates flags based on the result.
    // This instructions basically identical to SUB n,
    // but does not update the A register

    let byte = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let zero = a.wrapping_sub(byte) == 0;
    let half_carry = (a & 0x0F) < (byte & 0x0F);
    let carry = a < byte;

    cpu.registers.f.set_flags(zero, true, half_carry, carry);
}
