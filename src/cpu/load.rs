use crate::cpu::Cpu;
use crate::instruction::Target;

pub fn ld_r_r(cpu: &mut Cpu, to: Target, from: Target) {
    // 8-bit load instructions transfer one byte of data
    // between two 8-bit registers, or between one 8-bit
    // register and location in memory

    let set_reg = cpu.registers.get_register_setter(&to);
    let value = cpu.registers.get_register_value(&from);

    set_reg(&mut cpu.registers, value);
}

pub fn ld_rr_r(cpu: &mut Cpu, pair_target: Target, reg_target: Target) {
    // Load data from the 8-bit target register to the
    // absolute address specified by the 16-bit register

    let address = cpu.registers.get_pair_value(&pair_target);
    let value = cpu.registers.get_register_value(&reg_target);

    cpu.memory_bus.write_byte(address, value);
}

pub fn ld_rr_nn(cpu: &mut Cpu, target: Target) {
    // Load to the 16-bit register rr, the
    // immediate 16-bit data nn

    let value = cpu.get_nn_little_endian();
    let set_pair = cpu.registers.get_pair_setter(&target);
    set_pair(&mut cpu.registers, value);
}

pub fn ld_r_rr(cpu: &mut Cpu, reg_target: Target, pair_target: Target) {
    // Load data from the absolute address specified
    // by the 16-bit register to the 8-bit register

    let address = cpu.registers.get_pair_value(&pair_target);
    let set_reg = cpu.registers.get_register_setter(&reg_target);
    let value = cpu.memory_bus.read_byte(address);
    set_reg(&mut cpu.registers, value);
}

pub fn ld_r_n(cpu: &mut Cpu, target: Target) {
    // Load the immediate 8-bit value to the 8-bit target register

    let byte = cpu.memory_bus.read_byte(cpu.program_counter.next());
    let set_reg = cpu.registers.get_register_setter(&target);
    set_reg(&mut cpu.registers, byte);
}

pub fn ld_hl_n(cpu: &mut Cpu) {
    // Load to the absolute address specified by the 16-bit register
    // HL, the immediate data n

    let hl = cpu.registers.get_hl();
    let n = cpu.memory_bus.read_byte(cpu.program_counter.next());

    cpu.memory_bus.write_byte(hl, n);
}

pub fn ld_hl_plus_a(cpu: &mut Cpu) {
    // Load to the absolute address specified by the 16-bit
    // register HL, data from the 8-bit A register. The
    // value of HL is incremented after the memory write

    let hl = cpu.registers.get_hl();
    let a = cpu.registers.get_a();
    cpu.memory_bus.write_byte(hl, a);

    cpu.registers.set_hl(hl.wrapping_add(1));
}

pub fn ld_hl_minus_a(cpu: &mut Cpu) {
    // Load to the absolute address specified by the 16-bit
    // register HL, data from the 8-bit A register. The
    // value of HL is decremented after the memory write

    let hl = cpu.registers.get_hl();
    let a = cpu.registers.get_a();
    cpu.memory_bus.write_byte(hl, a);

    cpu.registers.set_hl(hl.wrapping_sub(1));
}

pub fn ld_hl_sp_plus_n(cpu: &mut Cpu) {
    // Add the signed immediate value to SP and store the result in HL

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next()) as i8;
    let sp = cpu.stack_pointer as i32;

    let result = sp.wrapping_add(n as i32) as u16;

    let carry = (sp ^ n as i32 ^ result as i32) & 0x100 != 0;
    let half_carry = (sp ^ n as i32 ^ result as i32) & 0x10 != 0;

    cpu.registers.set_hl(result);

    cpu.registers.f.set_flags(false, false, half_carry, carry);
}

pub fn ld_a_hl_plus(cpu: &mut Cpu) {
    // Load to the 8-bit A register, data from the absolute
    // address specified by the 16-bit register HL. The value
    // of HL is incremented after the memory read

    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);

    cpu.registers.set_a(value);
    cpu.registers.set_hl(hl.wrapping_add(1));
}

pub fn ld_a_hl_minus(cpu: &mut Cpu) {
    // Load to the 8-bit A register, data from the absolute
    // address specified by the 16-bit register HL. The value
    // of HL is decremented after the memory read

    let hl = cpu.registers.get_hl();
    let value = cpu.memory_bus.read_byte(hl);

    cpu.registers.set_a(value);
    cpu.registers.set_hl(hl.wrapping_sub(1));
}

pub fn ld_a_nn(cpu: &mut Cpu) {
    // Load to the 8-bit A register, data from the absolute
    // address specified by the 16-bit operand nn

    let address = cpu.get_nn_little_endian();
    let value = cpu.memory_bus.read_byte(address);

    cpu.registers.set_a(value);
}

pub fn ld_nn_a(cpu: &mut Cpu) {
    // Load data from the 8-bit A register to the absolute
    // address specified by the 16-bit immediate values

    let address = cpu.get_nn_little_endian();
    let a = cpu.registers.get_a();

    cpu.memory_bus.write_byte(address, a);
}

pub fn ldh_n_a(cpu: &mut Cpu) {
    // Load to the address specified by the 8-bit immediate
    // data n, data from the 8-bit A register. The full 16-bit
    // absolute address is obtained by setting the most significant
    // byte to 0xFF and the least significant byte to the value of
    // n, so the possible range is 0xFF00-0xFFFF

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next()) as u16;
    let address = 0xFF00 | n;

    let value = cpu.registers.get_a();
    cpu.memory_bus.write_byte(address, value)
}

pub fn ldh_a_n(cpu: &mut Cpu) {
    // Load to the 8-bit A register, data from the address specified
    // by the 8-bit immediate data n. The full 16-bit absolute address
    // is obtained by setting the most significant byte to 0xFF and
    // the least significant byte to the value of n, so the possible
    // range is 0xFF00-0xFFFF

    let n = cpu.memory_bus.read_byte(cpu.program_counter.next()) as u16;
    let address = 0xFF00 | n;

    let value = cpu.memory_bus.read_byte(address);
    cpu.registers.set_a(value);
}

pub fn ld_sp_nn(cpu: &mut Cpu) {
    // loads the immediate 16-bit value into the stack pointer register

    let value = cpu.get_nn_little_endian();
    cpu.stack_pointer = value;
}

pub fn ld_sp_hl(cpu: &mut Cpu) {
    // Load to the 16-bit SP register, data from the 16-bit HL register

    let hl = cpu.registers.get_hl();
    cpu.stack_pointer = hl;
}

pub fn ld_nn_sp(cpu: &mut Cpu) {
    // Load to the absolute address specified by the 16-bit operand
    // nn, data from the 16-bit SP register

    let nn = cpu.get_nn_little_endian();
    let sp = cpu.stack_pointer;

    let lsb = sp as u8;
    let msb = (sp >> 8) as u8;

    cpu.memory_bus.write_byte(nn, lsb);
    cpu.memory_bus.write_byte(nn.wrapping_add(1), msb);
}

pub fn push_rr(cpu: &mut Cpu, target: Target) {
    // Push to the stack memory, data from the 16-bit register rr

    let value = cpu.registers.get_pair_value(&target);
    cpu.push_stack(value);
}

pub fn pop_rr(cpu: &mut Cpu, target: Target) {
    // Pops to the 16-bit register rr, data from the stack memory

    let set_pair = cpu.registers.get_pair_setter(&target);
    let value = cpu.pop_stack();

    set_pair(&mut cpu.registers, value);
}

pub fn pop_af(cpu: &mut Cpu) {
    // Pops to the 16-bit register rr, data from the stack memory.
    // Completely replaces the F register value, so all
    // flags are changed based on the 8-bit data that is read from memory

    let value = cpu.pop_stack();
    cpu.registers.set_af(value);
}
