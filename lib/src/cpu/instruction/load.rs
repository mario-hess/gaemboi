use crate::{
    bus::Bus,
    cpu::{
        instruction::{CycleDuration, Target},
        Cpu,
    },
    utils::cpu::{carry::Carry, half_carry::HalfCarry},
};

// 8-bit load instructions transfer one byte of data
// between two 8-bit registers, or between one 8-bit
// register and location in memory
pub fn ld_r_r(cpu: &mut Cpu, to: Target, from: Target) -> CycleDuration {
    let value = cpu.registers.get_register(&from);
    cpu.registers.set_register(to, value);

    CycleDuration::Default
}

// Load data from the 8-bit target register to the
// absolute address specified by the 16-bit register
pub fn ld_rr_r(
    cpu: &mut Cpu,
    bus: &mut Bus,
    pair_target: Target,
    reg_target: Target,
) -> CycleDuration {
    let address = cpu.registers.get_pair(&pair_target);
    let value = cpu.registers.get_register(&reg_target);
    bus.write_byte(address, value);

    CycleDuration::Default
}

// Load to the 16-bit register rr, the
// immediate 16-bit data nn
pub fn ld_rr_nn(cpu: &mut Cpu, bus: &Bus, target: Target) -> CycleDuration {
    let value = cpu.get_nn_little_endian(bus);
    cpu.registers.set_pair(target, value);

    CycleDuration::Default
}

// Load data from the absolute address specified
// by the 16-bit register to the 8-bit register
pub fn ld_r_rr(cpu: &mut Cpu, bus: &Bus, reg_target: Target, pair_target: Target) -> CycleDuration {
    let address = cpu.registers.get_pair(&pair_target);
    let value = bus.read_byte(address);
    cpu.registers.set_register(reg_target, value);

    CycleDuration::Default
}

// Load the immediate 8-bit value to the 8-bit target register
pub fn ld_r_n(cpu: &mut Cpu, bus: &Bus, target: Target) -> CycleDuration {
    let value = bus.read_byte(cpu.program_counter.next());
    cpu.registers.set_register(target, value);

    CycleDuration::Default
}

// Load to the absolute address specified by the 16-bit register
// HL, the immediate data n
pub fn ld_hl_n(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let n = bus.read_byte(cpu.program_counter.next());
    bus.write_byte(hl, n);

    CycleDuration::Default
}

// Load to the absolute address specified by the 16-bit
// register HL, data from the 8-bit A register. The
// value of HL is incremented after the memory write
pub fn ld_hl_plus_a(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let a = cpu.registers.get_a();

    bus.write_byte(hl, a);
    cpu.registers.set_hl(hl.wrapping_add(1));

    CycleDuration::Default
}

// Load to the absolute address specified by the 16-bit
// register HL, data from the 8-bit A register. The
// value of HL is decremented after the memory write
pub fn ld_hl_minus_a(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let a = cpu.registers.get_a();

    bus.write_byte(hl, a);
    cpu.registers.set_hl(hl.wrapping_sub(1));

    CycleDuration::Default
}

// Add the signed immediate value to SP and store the result in HL
pub fn ld_hl_sp_plus_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next()) as i8;
    let sp = cpu.stack.get_pointer() as i32;

    let result = sp.wrapping_add(n as i32) as u16;
    cpu.registers.set_hl(result);

    let half_carry = HalfCarry::add_from_i32(sp, n.into());
    let carry = Carry::add_from_i32(sp, n.into());

    cpu.registers.flags.set_zero(false);
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(half_carry);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Load to the 8-bit A register, data from the absolute
// address specified by the 16-bit register HL. The value
// of HL is incremented after the memory read
pub fn ld_a_hl_plus(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let value = bus.read_byte(hl);

    cpu.registers.set_a(value);
    cpu.registers.set_hl(hl.wrapping_add(1));

    CycleDuration::Default
}

// Load to the 8-bit A register, data from the absolute
// address specified by the 16-bit register HL. The value
// of HL is decremented after the memory read
pub fn ld_a_hl_minus(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    let value = bus.read_byte(hl);

    cpu.registers.set_a(value);
    cpu.registers.set_hl(hl.wrapping_sub(1));

    CycleDuration::Default
}

// Load to the 8-bit A register, data from the absolute
// address specified by the 16-bit operand nn
pub fn ld_a_nn(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let address = cpu.get_nn_little_endian(bus);
    let value = bus.read_byte(address);

    cpu.registers.set_a(value);

    CycleDuration::Default
}

// Load data from the 8-bit A register to the absolute
// address specified by the 16-bit immediate values
pub fn ld_nn_a(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.get_nn_little_endian(bus);
    let a = cpu.registers.get_a();

    bus.write_byte(address, a);

    CycleDuration::Default
}

// Load to the address specified by the 8-bit immediate
// data n, data from the 8-bit A register. The full 16-bit
// absolute address is obtained by setting the most significant
// byte to 0xFF and the least significant byte to the value of
// n, so the possible range is 0xFF00-0xFFFF
pub fn ldh_n_a(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next()) as u16;
    let address = 0xFF00 | n;

    let value = cpu.registers.get_a();
    bus.write_byte(address, value);

    CycleDuration::Default
}

// Load to the 8-bit A register, data from the address specified
// by the 8-bit immediate data n. The full 16-bit absolute address
// is obtained by setting the most significant byte to 0xFF and
// the least significant byte to the value of n, so the possible
// range is 0xFF00-0xFFFF
pub fn ldh_a_n(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let n = bus.read_byte(cpu.program_counter.next()) as u16;
    let address = 0xFF00 | n;

    let value = bus.read_byte(address);
    cpu.registers.set_a(value);

    CycleDuration::Default
}

// Load to the 8-bit A register, data from the address specified
// by the 8-bit C register. The full 16-bit absolute address is
// obtained by setting the most significant byte to 0xFF and the
// least significant byte to the value of C, so the possible range
// is 0xFF00-0xFFFF
pub fn ldh_a_c(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let c: u8 = cpu.registers.get_c();
    let address: u16 = 0xFF00 | c as u16;

    let value = bus.read_byte(address);
    cpu.registers.set_a(value);

    CycleDuration::Default
}

// Loads the immediate 16-bit value into the stack pointer register
pub fn ld_sp_nn(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let value = cpu.get_nn_little_endian(bus);
    cpu.stack.set_pointer(value);

    CycleDuration::Default
}

// Load to the 16-bit SP register, data from the 16-bit HL register
pub fn ld_sp_hl(cpu: &mut Cpu) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    cpu.stack.set_pointer(hl);

    CycleDuration::Default
}

// Load to the absolute address specified by the 16-bit operand
// nn, data from the 16-bit SP register.
pub fn ld_nn_sp(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let nn = cpu.get_nn_little_endian(bus);
    let sp = cpu.stack.get_pointer();

    let lsb = sp as u8;
    let msb = (sp >> 8) as u8;

    bus.write_byte(nn, lsb);
    bus.write_byte(nn.wrapping_add(1), msb);

    CycleDuration::Default
}

// Load to the address specified by the 8-bit C register, data
// from the 8-bit A register. The full 16-bit absolute
// address is obtained by setting the most significant byte to
// 0xFF and the least significant byte to the value of C,
// so the possible range is 0xFF00-0xFFFF
pub fn ld_c_a(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let c = cpu.registers.get_c();
    let value = cpu.registers.get_a();

    let address: u16 = 0xFF00 | (c as u16);
    bus.write_byte(address, value);

    CycleDuration::Default
}

// Push to the stack memory, data from the 16-bit register rr
pub fn push_rr(cpu: &mut Cpu, bus: &mut Bus, target: Target) -> CycleDuration {
    let value = cpu.registers.get_pair(&target);
    cpu.stack.push(bus, value);

    CycleDuration::Default
}

// Pops to the 16-bit register rr, data from the stack memory
pub fn pop_rr(cpu: &mut Cpu, bus: &Bus, target: Target) -> CycleDuration {
    let value = cpu.stack.pop(bus);
    cpu.registers.set_pair(target, value);

    CycleDuration::Default
}

// Pops to the 16-bit register rr, data from the stack memory.
// Completely replaces the F register value, so all
// flags are changed based on the 8-bit data that is read from memory
pub fn pop_af(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let value = cpu.stack.pop(bus);
    cpu.registers.set_af(value);

    CycleDuration::Default
}
