use crate::gb_classic::{
    bus::Bus,
    cpu::{
        instruction::{CycleDuration, Target},
        Cpu,
    },
    utils::cpu::{carry::Carry, half_carry::HalfCarry},
};

// Adds to the 8-bit A register, the 8-bit register r,
// and stores the result back into the A register
pub fn add_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);
    let a = cpu.registers.get_a();

    let result = a.wrapping_add(r);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8(a, r);
    let carry = Carry::add_from_u8(a, r);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Adds to the 8-bit A register, the immediate data n,
// and stores the result back into the A register
pub fn add_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a.wrapping_add(n);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8(a, n);
    let carry = Carry::add_from_u8(a, n);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Adds to the 8-bit A register, data from the absolute
// address specified by the 16-bit register HL, and stores
// the result back into the A register
pub fn add_a_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a.wrapping_add(byte);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8(a, byte);
    let carry = Carry::add_from_u8(a, byte);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Add the byte in r16 to HL
pub fn add_hl_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let rr = cpu.registers.get_pair(&target);
    let hl = cpu.registers.get_hl();

    let result = hl.wrapping_add(rr);
    cpu.registers.set_hl(result);

    let half_carry = HalfCarry::add_from_u16(hl, rr);
    let carry = Carry::add_from_u16(hl, rr);

    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Add the byte in SP to HL
pub fn add_hl_sp(cpu: &mut Cpu) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let sp = cpu.stack.get_pointer();

    let result = hl.wrapping_add(sp);
    cpu.registers.set_hl(result);

    let half_carry = HalfCarry::add_from_u16(hl, sp);
    let carry = Carry::add_from_u16(hl, sp);

    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Add the signed immediate byte to SP
pub fn add_sp_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next()) as i8;
    let sp = cpu.stack.get_pointer() as i32;

    let result = sp.wrapping_add(n as i32) as u16;
    cpu.stack.set_pointer(result);

    let half_carry = HalfCarry::add_from_i32(sp, n.into());
    let carry = Carry::add_from_i32(sp, n.into());

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Adds to the 8-bit A register, the carry flag
// and the 8-bit register r, and stores the result
// back into the A register
pub fn adc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);
    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_add(r).wrapping_add(carry);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8_with_carry(a, r, carry);
    let carry = Carry::add_from_u8_with_carry(a, r, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Adds to the 8-bit A register, the carry flag and
// the immediate data n, and stores the result back
// into the A register
pub fn adc_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_add(n).wrapping_add(carry);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8_with_carry(a, n, carry);
    let carry = Carry::add_from_u8_with_carry(a, n, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Adds to the 8-bit A register, the carry flag and data
// from the absolute address specified by the 16-bit
// register HL, and stores the result back into the A register
pub fn adc_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_add(carry).wrapping_add(byte);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::add_from_u8_with_carry(a, byte, carry);
    let carry = Carry::add_from_u8_with_carry(a, byte, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the
// 8-bit register r, and stores the result
// back into the A register
pub fn sub_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let result = a.wrapping_sub(r);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8(a, r);
    let carry = Carry::sub_from_u8(a, r);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the
// immediate data n, and stores the result
// back into the A register
pub fn sub_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a.wrapping_sub(n);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8(a, n);
    let carry = Carry::sub_from_u8(a, n);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, data from
// the absolute address specified by the 16-bit
// register HL, and stores the result back into
// the A register
pub fn sub_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a.wrapping_sub(byte);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8(a, byte);
    let carry = Carry::sub_from_u8(a, byte);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the
// carry flag and the 8-bit register r, and
// stores the result back into the A register
pub fn sbc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_sub(carry).wrapping_sub(r);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8_with_carry(a, r, carry);
    let carry = Carry::sub_from_u8_with_carry(a, r, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the
// carry flag and the immediate data n, and
// stores the result back into the A register
pub fn sbc_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let n = bus.read_byte(cpu.program_counter.next());

    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_sub(carry).wrapping_sub(n);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8_with_carry(a, n, carry);
    let carry = Carry::sub_from_u8_with_carry(a, n, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the
// carry flag and data from the absolute
// address specified by the 16-bit register HL,
// and stores the result back into the A register
pub fn sbc_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);
    let carry: u8 = cpu.registers.flags.get_carry().into();

    let result = a.wrapping_sub(carry).wrapping_sub(byte);
    cpu.registers.set_a(result);

    let half_carry = HalfCarry::sub_from_u8_with_carry(a, byte, carry);
    let carry = Carry::sub_from_u8_with_carry(a, byte, carry);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Performs a bitwise AND operation between the
// 8-bit A register and the 8-bit register r,
// and stores the result back into the A register
pub fn and_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
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

// Performs a bitwise AND operation between the
// 8-bit A register and immediate data n, and
// stores the result back into the A register
pub fn and_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a & n;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Performs a bitwise AND operation between the
// 8-bit A register and data from the absolute
// address specified by the 16-bit register HL,
// and stores the result back into the A register
pub fn and_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a & byte;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(true);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Increments data in the 8-bit target register
pub fn inc_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let result = r.wrapping_add(1);
    cpu.registers.set_register(target, result);

    let half_carry = HalfCarry::add_from_u8(r, 1);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

// Increments data in the 16-bit target register by 1
pub fn inc_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let byte = cpu.registers.get_pair(&target);
    cpu.registers.set_pair(target, byte.wrapping_add(1));

    CycleDuration::Default
}

// Increments data at the absolute address specified
// by the 16-bit register HL
pub fn inc_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = byte.wrapping_add(1);
    bus.write_byte(hl, result);

    let half_carry = HalfCarry::add_from_u8(byte, 1);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

// Increment SP by 1
pub fn inc_sp(cpu: &mut Cpu) -> CycleDuration {
    cpu.stack
        .set_pointer(cpu.stack.get_pointer().wrapping_add(1));

    CycleDuration::Default
}

// Decrements data in the 8-bit target register
pub fn dec_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_register(&target);

    let result = r.wrapping_sub(1);
    cpu.registers.set_register(target, result);

    let half_carry = HalfCarry::sub_from_u8(r, 1);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

// Decrements data in the 16-bit target register
pub fn dec_rr(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let r = cpu.registers.get_pair(&target);

    let result = r.wrapping_sub(1);
    cpu.registers.set_pair(target, result);

    CycleDuration::Default
}

// Decrement SP by 1
pub fn dec_sp(cpu: &mut Cpu) -> CycleDuration {
    cpu.stack
        .set_pointer(cpu.stack.get_pointer().wrapping_sub(1));

    CycleDuration::Default
}

// Decrements data at the absolute address
// specified by the 16-bit register HL
pub fn dec_hl(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = byte.wrapping_sub(1);
    bus.write_byte(hl, result);

    let half_carry = HalfCarry::sub_from_u8(byte, 1);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);

    CycleDuration::Default
}

// Performs a bitwise OR operation between the 8-bit
// A register and the 8-bit register r, and stores
// the result back into the A register
pub fn or_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
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

// Performs a bitwise OR operation between the 8-bit
// A register and immediate data n, and stores the
// result back into the A register
pub fn or_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let n = bus.read_byte(cpu.program_counter.next());

    let result = a | n;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Performs a bitwise OR operation between the 8-bit
// A register and data from the absolute address
// specified by the 16-bit register HL, and stores
// the result back into the A register
pub fn or_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a | byte;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Performs a bitwise XOR operation between the
// 8-bit A register and the 8-bit target register,
// and stores the result back into the A register
pub fn xor_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let a = cpu.registers.get_a();
    let byte = cpu.registers.get_register(&target);

    let result = a ^ byte;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Performs a bitwise XOR operation between the 8-bit
// A register and immediate data n, and stores the
// result back into the A register
pub fn xor_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let n = bus.read_byte(cpu.program_counter.next());

    let result = a ^ n;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Performs a bitwise XOR operation between the
// 8-bit A register and data from the absolute
// address specified by the 16-bit register HL,
// and stores the result back into the A register
pub fn xor_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a ^ byte;
    cpu.registers.set_a(result);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(false);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the immediate
// data n, and updates flags based on the result.
// This instructions basically identical to SUB n,
// but does not update the A register
pub fn cp_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let byte = bus.read_byte(cpu.program_counter.next());
    let a = cpu.registers.get_a();

    let result = a.wrapping_sub(byte);

    let half_carry = HalfCarry::sub_from_u8(a, byte);
    let carry = Carry::sub_from_u8(a, byte);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, the 8-bit
// register r, and updates flags based on the result.
// This instruction is basically identical to SUB r,
// but does not update the A register
pub fn cp_r(cpu: &mut Cpu, target: Target) -> CycleDuration {
    let a = cpu.registers.get_a();
    let r = cpu.registers.get_register(&target);

    let result = a.wrapping_sub(r);

    let half_carry = HalfCarry::sub_from_u8(a, r);
    let carry = Carry::sub_from_u8(a, r);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Subtracts from the 8-bit A register, data from the
// absolute address specified by the 16-bit register
// HL, and updates flags based on the result. This
// instruction is basically identical to SUB (HL), but
// does not update the A register
pub fn cp_hl(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let a = cpu.registers.get_a();
    let hl = cpu.registers.get_hl();
    let byte = bus.read_byte(hl);

    let result = a.wrapping_sub(byte);

    let half_carry = HalfCarry::sub_from_u8(a, byte);
    let carry = Carry::sub_from_u8(a, byte);

    cpu.registers.flags.set_zero(result == 0);
    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}
