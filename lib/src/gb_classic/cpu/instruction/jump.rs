use crate::gb_classic::{
    bus::Bus,
    cpu::{
        instruction::{CycleDuration, Flag},
        Cpu,
    },
};

// Unconditional jump to the absolute address
// specified by the 16-bit immediate values
pub fn jp_nn(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let address = cpu.get_nn_little_endian(bus);
    cpu.program_counter.set(address);

    CycleDuration::Default
}

// Conditional jump to the absolute address
// specified by the 16-bit operand nn, depending
// on the condition cc. Note that the operand
// (absolute address) is read even when the
// condition is false
pub fn jp_c_nn(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let nn = cpu.get_nn_little_endian(bus);
    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        cpu.program_counter.set(nn);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Conditional jump to the absolute address
// specified by the 16-bit operand nn,
// depending on the condition cc
pub fn jp_nc_nn(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let nn = cpu.get_nn_little_endian(bus);
    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        cpu.program_counter.set(nn);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Unconditional jump to the absolute address
// specified by the 16-bit register HL
pub fn jp_hl(cpu: &mut Cpu) -> CycleDuration {
    let hl = cpu.registers.get_hl();
    cpu.program_counter.set(hl);

    CycleDuration::Default
}

// Unconditional jump to the relative address
// specified by the signed 8-bit immediate value
pub fn jr_e(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let address = bus.read_byte(cpu.program_counter.next()) as i8;
    cpu.program_counter.relative_jump(address);

    CycleDuration::Default
}

// Conditional jump to the relative address specified
// by the signed 8-bit immediate value, depending on the
// flag condition
pub fn jr_c_e(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let address = bus.read_byte(cpu.program_counter.next()) as i8;
    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        cpu.program_counter.relative_jump(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Conditional jump to the relative address specified
// by the signed 8-bit immediate value, depending on the
// flag condition
pub fn jr_nc_e(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let address = bus.read_byte(cpu.program_counter.next()) as i8;
    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        cpu.program_counter.relative_jump(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Unconditional function call to the absolute address
// specified by the 16-bit operand nn
pub fn call_nn(cpu: &mut Cpu, bus: &mut Bus) -> CycleDuration {
    let address = cpu.get_nn_little_endian(bus);
    cpu.stack.push(bus, cpu.program_counter.get());
    cpu.program_counter.set(address);

    CycleDuration::Default
}

// Conditional call to a subroutine at the absolute
// 16-bit memory address a16 if the flag is set
pub fn call_c_nn(cpu: &mut Cpu, bus: &mut Bus, flag: Flag) -> CycleDuration {
    let flag = cpu.registers.flags.get_flag(flag);
    let address = cpu.get_nn_little_endian(bus);

    if flag {
        cpu.stack.push(bus, cpu.program_counter.get());
        cpu.program_counter.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Conditional call to a subroutine at the absolute
// 16-bit memory address a16 if the flag is set.
pub fn call_nc_nn(cpu: &mut Cpu, bus: &mut Bus, flag: Flag) -> CycleDuration {
    let flag = cpu.registers.flags.get_flag(flag);
    let address = cpu.get_nn_little_endian(bus);

    if !flag {
        cpu.stack.push(bus, cpu.program_counter.get());
        cpu.program_counter.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Unconditional function call to the absolute
// fixed address defined by the oprogram_counterode
pub fn rst(cpu: &mut Cpu, bus: &mut Bus, address: u16) -> CycleDuration {
    cpu.stack.push(bus, cpu.program_counter.get());
    cpu.program_counter.set(address);

    CycleDuration::Default
}

// Unconditional return from a function
pub fn ret(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let address = cpu.stack.pop(bus);
    cpu.program_counter.set(address);

    CycleDuration::Default
}

// Unconditional return from a function
// Also enables interrupts by setting IME=1
pub fn reti(cpu: &mut Cpu, bus: &Bus) -> CycleDuration {
    let address = cpu.stack.pop(bus);
    cpu.program_counter.set(address);
    cpu.ime = true;

    CycleDuration::Default
}

// Conditional return from a function,
// depending on the condition c
pub fn ret_c(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        let address = cpu.stack.pop(bus);
        cpu.program_counter.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

// Conditional return from a function,
// depending on the condition nc
pub fn ret_nc(cpu: &mut Cpu, bus: &Bus, flag: Flag) -> CycleDuration {
    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        let address = cpu.stack.pop(bus);
        cpu.program_counter.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}
