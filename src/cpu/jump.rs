use crate::cpu::Cpu;
use crate::instruction::Flag;

pub fn jp_nn(cpu: &mut Cpu) {
    // Unconditional jump to the absolute address
    // specified by the 16-bit immediate values

    let address = cpu.get_nn_little_endian();
    cpu.program_counter.set(address);
}

pub fn jp_nc_nn(cpu: &mut Cpu, flag: Flag) {
    // Conditional jump to the absolute address
    // specified by the 16-bit operand nn,
    // depending on the condition cc

    let nn = cpu.get_nn_little_endian();
    let flag = cpu.get_flag_value(flag);

    if !flag {
        cpu.program_counter.set(nn);
    }
    
}

pub fn jp_hl(cpu: &mut Cpu) {
    // Unconditional jump to the absolute address
    // specified by the 16-bit register HL

    let hl = cpu.registers.get_hl();
    cpu.program_counter.set(hl);
}

pub fn jr_e(cpu: &mut Cpu) {
    // Unconditional jump to the relative address
    // specified by the signed 8-bit immediate value

    let address = cpu.memory_bus.read_byte(cpu.program_counter.next()) as i8;
    cpu.program_counter.relative_jump(address);
}

pub fn jr_c_e(cpu: &mut Cpu, flag: Flag) {
    // Conditional jump to the relative address specified
    // by the signed 8-bit immediate value, depending on the
    // flag condition

    let address = cpu.memory_bus.read_byte(cpu.program_counter.next()) as i8;
    let flag = cpu.get_flag_value(flag);

    if flag {
        cpu.program_counter.relative_jump(address);
    }
}

pub fn jr_nc_e(cpu: &mut Cpu, flag: Flag) {
    // Conditional jump to the relative address specified
    // by the signed 8-bit immediate value, depending on the
    // flag condition

    let address = cpu.memory_bus.read_byte(cpu.program_counter.next()) as i8;
    let flag = cpu.get_flag_value(flag);

    if !flag {
        cpu.program_counter.relative_jump(address);
    }
}

pub fn call_nn(cpu: &mut Cpu) {
    // Unconditional function call to the absolute address
    // specified by the 16-bit operand nn

    let address = cpu.get_nn_little_endian();
    cpu.push_stack(cpu.program_counter.get());
    cpu.program_counter.set(address);
}

pub fn call_c_nn(cpu: &mut Cpu, flag: Flag) {
    // conditional call to a subroutine at the absolute
    // 16-bit memory address a16 if the flag is set.

    let flag = cpu.get_flag_value(flag);
    let address = cpu.get_nn_little_endian();

    if flag {
        cpu.push_stack(cpu.program_counter.get());
        cpu.program_counter.set(address)
    }
}

pub fn call_nc_nn(cpu: &mut Cpu, flag: Flag) {
    // conditional call to a subroutine at the absolute
    // 16-bit memory address a16 if the flag is set.

    let flag = cpu.get_flag_value(flag);
    let address = cpu.get_nn_little_endian();

    if !flag {
        cpu.push_stack(cpu.program_counter.get());
        cpu.program_counter.set(address)
    }
}

pub fn rst(cpu: &mut Cpu, address: u16) {
    // Unconditional function call to the absolute
    // fixed address defined by the opcode

    cpu.push_stack(cpu.program_counter.get());
    cpu.program_counter.set(address);
}

pub fn ret(cpu: &mut Cpu) {
    // Unconditional return from a function

    let address = cpu.pop_stack();
    cpu.program_counter.set(address);
}

pub fn ret_c(cpu: &mut Cpu, flag: Flag) {
    // Conditional return from a function,
    // depending on the condition c

    let flag = cpu.get_flag_value(flag);

    if flag {
        let address = cpu.pop_stack();
        cpu.program_counter.set(address);
    }
}

pub fn ret_nc(cpu: &mut Cpu, flag: Flag) {
    // Conditional return from a function,
    // depending on the condition nc

    let flag = cpu.get_flag_value(flag);

    if !flag {
        let address = cpu.pop_stack();
        cpu.program_counter.set(address);
    }
}
