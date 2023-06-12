use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn add_r(cpu: &mut Cpu, target: Target) {
    // Adds to the 8-bit A register, the 8-bit register r,
    // and stores the result back into the A register

    let r = cpu.registers.get_register_value(&target);
    let a = cpu.registers.get_a();

    let result = a.wrapping_add(r);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
    cpu.registers.f.set_carry(a < r);
}

pub fn add_n(cpu: &mut Cpu) {
    // Adds to the 8-bit A register, the immediate data n,
    // and stores the result back into the A register

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a.wrapping_add(n);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
    cpu.registers.f.set_carry(a < n);
}

pub fn add_hl_rr(cpu: &mut Cpu, target: Target) {
    // Add the value in r16 to HL
    
    let rr = cpu.registers.get_pair_value(&target);
    let hl = cpu.registers.get_hl();

    let result = hl.wrapping_add(rr);
    cpu.registers.set_hl(result);

    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry((hl & 0xFFF) + (rr & 0xFFF) > 0xFFF);
    cpu.registers.f.set_carry(result < hl);
}

pub fn adc_r(cpu: &mut Cpu, target: Target) {
    // Adds to the 8-bit A register, the carry flag
    // and the 8-bit register r, and stores the result
    // back into the A register

    let r = cpu.registers.get_register_value(&target);
    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.f.get_carry().into();

    let result = a.wrapping_add(r).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers
        .f
        .set_half_carry(((a & 0x0F) + (r & 0x0F)) + carry > 0x0F);
    cpu.registers
        .f
        .set_carry((a as u16) + (r as u16) + (carry as u16) > 0xFF);
}

pub fn adc_n(cpu: &mut Cpu) {
    // Adds to the 8-bit A register, the carry flag and
    // the immediate data n, and stores the result back
    // into the A register

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.f.get_carry().into();

    let result = a.wrapping_add(n).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers
        .f
        .set_half_carry(((a & 0x0F) + (n & 0x0F)) + carry > 0x0F);
    cpu.registers
        .f
        .set_carry((a as u16) + (n as u16) + (carry as u16) > 0xFF);
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
    cpu.registers.f.set_half_carry((a & 0x0F) < (n & 0x0F));
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

pub fn inc_r(cpu: &mut Cpu, target: Target) {
    // Increments data in the 8-bit register r

    let reg = cpu.registers.get_register_value(&target);
    let set_reg = cpu.registers.get_register_setter(&target);

    let result = reg.wrapping_add(1);
    set_reg(&mut cpu.registers, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(false);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
}

pub fn inc_rr(cpu: &mut Cpu, target: Target) {
    // Increments data in the 16-bit target register by 1

    let value = cpu.registers.get_pair_value(&target);
    let set_reg = cpu.registers.get_pair_setter(&target);
    set_reg(&mut cpu.registers, value.wrapping_add(1));
}

pub fn dec_r(cpu: &mut Cpu, target: Target) {
    // Decrements data in the 8-bit target register

    let r = cpu.registers.get_register_value(&target);
    let set_r = cpu.registers.get_register_setter(&target);

    let result = r.wrapping_sub(1);
    set_r(&mut cpu.registers, result);

    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry((result & 0x0F) == 0);
}

pub fn dec_rr(cpu: &mut Cpu, target: Target) {
    // Decrements data in the 16-bittarget register

    let reg = cpu.registers.get_pair_value(&target);
    let set_reg = cpu.registers.get_pair_setter(&target);

    let result = reg.wrapping_sub(1);
    set_reg(&mut cpu.registers, result);
}

pub fn dec_hl(cpu: &mut Cpu) {
    // Decrements data at the absolute address
    // specified by the 16-bit register HL

    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);
    let result = value.wrapping_sub(1);

    cpu.memory_bus.write_byte(hl, result);
    cpu.registers.f.set_zero(result == 0);
    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry((value & 0x0F) == 0x00);
}

// --- OR / XOR instructions ---
pub fn or_r(cpu: &mut Cpu, target: Target) {
    // Performs a bitwise OR operation between the 8-bit
    // A register and the 8-bit register r, and stores
    // the result back into the A register

    let r = cpu.registers.get_register_value(&target);
    let a = cpu.registers.get_a();

    let result = a | r;

    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(result == 0, false, false, false);
}

pub fn or_hl(cpu: &mut Cpu) {
    // Performs a bitwise OR operation between the 8-bit
    // A register and data from the absolute address
    // specified by the 16-bit register HL, and stores
    // the result back into the A register

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);

    let result = a | value;
    cpu.registers.set_a(result);

    cpu.registers.f.set_flags(result == 0, false, false, false);
}

pub fn xor_r(cpu: &mut Cpu, target: Target) {
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

pub fn xor_n(cpu: &mut Cpu) {
    // Performs a bitwise XOR operation between the 8-bit
    // A register and immediate data n, and stores the
    // result back into the A register

    let a = cpu.registers.get_a();
    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());

    let result = a ^ n;
    cpu.registers.set_a(result);

    cpu.registers.f.set_flags(result == 0, false, false, false);
}

pub fn xor_hl(cpu: &mut Cpu) {
    // Performs a bitwise XOR operation between the
    // 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL,
    // and stores the result back into the A register

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a ^ data;
    cpu.registers.set_a(result);
    cpu.registers.f.set_flags(result == 0, false, false, false);
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
