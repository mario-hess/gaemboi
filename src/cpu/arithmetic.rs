/**
 * @file    cpu/arithmetic.rs
 * @brief   Implementation of arithmetic instructions.
 * @author  Mario Hess
 * @date    October 20, 2023
 */
use crate::{
    cpu::Cpu,
    instruction::{CycleDuration, Target},
};

pub fn add_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Adds to the 8-bit A register, the 8-bit register r,
    // and stores the result back into the A register.

    let r = cpu.registers.get_register(&target);
    let a = cpu.registers.get_a();

    let half_carry = ((a & 0x0F) + (r & 0x0F)) > 0x0F;

    let result = a.wrapping_add(r);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(a as u16 + r as u16 > 0xFF);

    CycleDuration::Default
}

pub fn add_n(cpu: &mut Cpu) -> CycleDuration {
    // Adds to the 8-bit A register, the immediate data n,
    // and stores the result back into the A register.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let half_carry = ((a & 0x0F) + (n & 0x0F)) > 0x0F;

    let result = a.wrapping_add(n);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(a as u16 + n as u16 > 0xFF);

    CycleDuration::Default
}

pub fn add_a_hl(cpu: &mut Cpu) -> CycleDuration {
    // Adds to the 8-bit A register, data from the absolute
    // address specified by the 16-bit register HL, and stores
    // the result back into the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a.wrapping_add(data);
    cpu.registers.set_a(result);

    let zero = result == 0;
    let half_carry = ((a & 0x0F) + (data & 0x0F)) > 0x0F;
    let carry = a as u16 + data as u16 > 0xFF;

    cpu.registers.flags.set_zero(zero);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

pub fn add_hl_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Add the value in r16 to HL.

    let rr = cpu.registers.get_pair(&target);
    let hl = cpu.registers.get_hl();

    let half_carry = (hl & 0xFFF) + (rr & 0xFFF) > 0xFFF;

    let result = hl.wrapping_add(rr);
    cpu.registers.set_hl(result);

    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(result < hl);

    CycleDuration::Default
}

pub fn add_hl_sp(cpu: &mut Cpu) -> CycleDuration {
    // Add the value in SP to HL.

    let hl = cpu.registers.get_hl();
    let result = hl.wrapping_add(cpu.stack_pointer);

    let half_carry = (hl & 0xFFF) + (cpu.stack_pointer & 0xFFF) > 0xFFF;

    cpu.registers.set_hl(result);

    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(result < hl);

    CycleDuration::Default
}

pub fn add_sp_n(cpu: &mut Cpu) -> CycleDuration {
    // Add the signed immediate value to SP.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next()) as i8;
    let sp = cpu.stack_pointer as i32;
    let result = sp.wrapping_add(n as i32) as u16;

    let carry = (sp ^ n as i32 ^ result as i32) & 0x100 != 0;
    let half_carry = (sp ^ n as i32 ^ result as i32) & 0x10 != 0;

    cpu.stack_pointer = result;

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

pub fn adc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Adds to the 8-bit A register, the carry flag
    // and the 8-bit register r, and stores the result
    // back into the A register.

    let r = cpu.registers.get_register(&target);
    let a = cpu.registers.get_a();

    let carry: u8 = cpu.registers.flags.get_carry().into();
    let half_carry = ((a & 0x0F) + (r & 0x0F)) + carry > 0x0F;
    let new_carry = (a as u16) + (r as u16) + (carry as u16) > 0xFF;

    let result = a.wrapping_add(r).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn adc_n(cpu: &mut Cpu) -> CycleDuration {
    // Adds to the 8-bit A register, the carry flag and
    // the immediate data n, and stores the result back
    // into the A register.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let carry: u8 = cpu.registers.flags.get_carry().into();
    let half_carry = (a & 0x0F) + (n & 0x0F) + (carry & 0x0F) > 0x0F;
    let new_carry = (a as u16) + (n as u16) + (carry as u16) > 0xFF;

    let result = a.wrapping_add(n).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn adc_hl(cpu: &mut Cpu) -> CycleDuration {
    // Adds to the 8-bit A register, the carry flag and data
    // from the absolute address specified by the 16-bit
    // register HL, and stores the result back into the A register.

    let a = cpu.registers.get_a();

    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);

    let carry: u8 = cpu.registers.flags.get_carry().into();
    let half_carry = (a & 0x0F) + (value & 0x0F) + (carry & 0x0F) > 0x0F;
    let new_carry = (a as u16) + (value as u16) + (carry as u16) > 0xFF;

    let result = a.wrapping_add(carry).wrapping_add(value);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn sub_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Subtracts from the 8-bit A register, the
    // 8-bit register r, and stores the result
    // back into the A register.

    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let result = a.wrapping_sub(r);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry((a & 0x0F) < (r & 0x0F));
    cpu.registers.flags.set_carry(a < r);

    CycleDuration::Default
}

pub fn sub_n(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, the
    // immediate data n, and stores the result
    // back into the A register.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a.wrapping_sub(n);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry((a & 0x0F) < (n & 0x0F));
    cpu.registers.flags.set_carry(a < n);

    CycleDuration::Default
}

pub fn sub_hl(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, data from
    // the absolute address specified by the 16-bit
    // register HL, and stores the result back into
    // the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);
    let half_carry = (a & 0x0F) < (value & 0x0F);

    let result = a.wrapping_sub(value);
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(a < value);

    CycleDuration::Default
}

pub fn sbc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Subtracts from the 8-bit A register, the
    // carry flag and the 8-bit register r, and
    // stores the result back into the A register.

    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_sub(carry).wrapping_sub(r);
    let half_carry = (a ^ r ^ result) & 0x10 != 0;
    let new_carry = (a as u16) < ((r as u16) + (carry as u16));

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn sbc_n(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, the
    // carry flag and the immediate data n, and
    // stores the result back into the A register.

    let a = cpu.registers.get_a();
    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());

    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_sub(carry).wrapping_sub(n);
    let half_carry = (a ^ n ^ result) & 0x10 != 0;
    let new_carry = (a as u16) < ((n as u16) + (carry as u16));

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn sbc_hl(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, the
    // carry flag and data from the absolute
    // address specified by the 16-bit register HL,
    // and stores the result back into the A register.

    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.flags.get_carry().into();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a.wrapping_sub(carry).wrapping_sub(data);
    let half_carry = (a ^ data ^ result) & 0x10 != 0;
    let new_carry = (a as u16) < ((data as u16) + (carry as u16));
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(new_carry);

    CycleDuration::Default
}

pub fn and_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Performs a bitwise AND operation between the
    // 8-bit A register and the 8-bit register r,
    // and stores the result back into the A register.

    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let result = a & r;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn and_n(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise AND operation between the
    // 8-bit A register and immediate data n, and
    // stores the result back into the A register.

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();
    let result = a & n;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn and_hl(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise AND operation between the
    // 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL,
    // and stores the result back into the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a & data;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn inc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Increments data in the 8-bit target register.

    let reg = cpu.registers.get_register(&target);
    let half_carry = (reg & 0x0F).wrapping_add(1) > 0x0F;

    let result = reg.wrapping_add(1);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

pub fn inc_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Increments data in the 16-bit target register by 1.

    let value = cpu.registers.get_pair(&target);
    cpu.registers.set_pair(target, value.wrapping_add(1));

    CycleDuration::Default
}

pub fn inc_hl(cpu: &mut Cpu) -> CycleDuration {
    // Increments data at the absolute address specified
    // by the 16-bit register HL.

    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);
    let half_carry = (data & 0x0F).wrapping_add(1) > 0x0F;

    let result = data.wrapping_add(1);
    cpu.memory_bus.write_byte(hl, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

pub fn inc_sp(cpu: &mut Cpu) -> CycleDuration {
    // Increment SP by 1.

    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);

    CycleDuration::Default
}

pub fn dec_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Decrements data in the 8-bit target register.

    let r = cpu.registers.get_register(&target);
    let half_carry = (r & 0x0F).wrapping_sub(1) & 0x10 != 0;

    let result = r.wrapping_sub(1);
    cpu.registers.set_register(target, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

pub fn dec_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Decrements data in the 16-bit target register.

    let reg = cpu.registers.get_pair(&target);

    let result = reg.wrapping_sub(1);
    cpu.registers.set_pair(target, result);

    CycleDuration::Default
}

pub fn dec_sp(cpu: &mut Cpu) -> CycleDuration {
    // Decrement SP by 1.

    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);

    CycleDuration::Default
}

pub fn dec_hl(cpu: &mut Cpu) -> CycleDuration {
    // Decrements data at the absolute address
    // specified by the 16-bit register HL.

    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);
    let result = value.wrapping_sub(1);

    cpu.memory_bus.write_byte(hl, result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry((value & 0x0F) == 0x00);

    CycleDuration::Default
}

// --- OR / XOR instructions ---
pub fn or_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Performs a bitwise OR operation between the 8-bit
    // A register and the 8-bit register r, and stores
    // the result back into the A register.

    let r = cpu.registers.get_register(&target);
    let a = cpu.registers.get_a();

    let result = a | r;

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn or_n(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise OR operation between the 8-bit
    // A register and immediate data n, and stores the
    // result back into the A register.

    let a = cpu.registers.get_a();
    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());

    let result = a | n;

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn or_hl(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise OR operation between the 8-bit
    // A register and data from the absolute address
    // specified by the 16-bit register HL, and stores
    // the result back into the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);

    let result = a | value;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn xor_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Performs a bitwise XOR operation between the
    // 8-bit A register and the 8-bit target register,
    // and stores the result back into the A register.

    let a = cpu.registers.get_a();
    let value = cpu.registers.get_register(&target);

    let result = a ^ value;

    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn xor_n(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise XOR operation between the 8-bit
    // A register and immediate data n, and stores the
    // result back into the A register.

    let a = cpu.registers.get_a();
    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());

    let result = a ^ n;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn xor_hl(cpu: &mut Cpu) -> CycleDuration {
    // Performs a bitwise XOR operation between the
    // 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL,
    // and stores the result back into the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a ^ data;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

pub fn cp_n(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, the immediate
    // data n, and updates flags based on the result.
    // This instructions basically identical to SUB n,
    // but does not update the A register.

    let byte = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let zero = a.wrapping_sub(byte) == 0;
    let half_carry = (a & 0x0F) < (byte & 0x0F);
    let carry = a < byte;

    cpu.registers.flags.set_zero(zero);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

pub fn cp_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    // Subtracts from the 8-bit A register, the 8-bit
    // register r, and updates flags based on the result.
    // This instruction is basically identical to SUB r,
    // but does not update the A register.

    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let result = a.wrapping_sub(r);

    let half_carry = (a & 0x0F) < (r & 0x0F);
    let carry = a < r;

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

pub fn cp_hl(cpu: &mut Cpu) -> CycleDuration {
    // Subtracts from the 8-bit A register, data from the
    // absolute address specified by the 16-bit register
    // HL, and updates flags based on the result. This
    // instruction is basically identical to SUB (HL), but
    // does not update the A register.

    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let data = cpu.memory_bus.read_byte(hl);

    let result = a.wrapping_sub(data);

    let half_carry = (a & 0x0F) < (data & 0x0F);
    let carry = a < data;

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}
