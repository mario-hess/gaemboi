/**
 * @file    cpu/jump.rs
 * @brief   Implementation of jump instructions.
 * @author  Mario Hess
 * @date    November 11, 2023
 */
use crate::{
    cpu::Cpu,
    instruction::{CycleDuration, Flag},
};

pub fn jp_nn(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional jump to the absolute address
    // specified by the 16-bit immediate values

    let address = cpu.get_nn_little_endian();
    cpu.pc.set(address);

    CycleDuration::Default
}

pub fn jp_c_nn(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional jump to the absolute address
    // specified by the 16-bit operand nn, depending
    // on the condition cc. Note that the operand
    // (absolute address) is read even when the
    // condition is false

    let nn = cpu.get_nn_little_endian();
    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        cpu.pc.set(nn);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn jp_nc_nn(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional jump to the absolute address
    // specified by the 16-bit operand nn,
    // depending on the condition cc

    let nn = cpu.get_nn_little_endian();
    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        cpu.pc.set(nn);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn jp_hl(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional jump to the absolute address
    // specified by the 16-bit register HL

    let hl = cpu.registers.get_hl();
    cpu.pc.set(hl);

    CycleDuration::Default
}

pub fn jr_e(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional jump to the relative address
    // specified by the signed 8-bit immediate value

    let address = cpu.memory_bus.read_byte(cpu.pc.next()) as i8;
    cpu.pc.relative_jump(address);

    CycleDuration::Default
}

pub fn jr_c_e(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional jump to the relative address specified
    // by the signed 8-bit immediate value, depending on the
    // flag condition

    let address = cpu.memory_bus.read_byte(cpu.pc.next()) as i8;
    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        cpu.pc.relative_jump(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn jr_nc_e(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional jump to the relative address specified
    // by the signed 8-bit immediate value, depending on the
    // flag condition

    let address = cpu.memory_bus.read_byte(cpu.pc.next()) as i8;
    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        cpu.pc.relative_jump(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn call_nn(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional function call to the absolute address
    // specified by the 16-bit operand nn

    let address = cpu.get_nn_little_endian();
    cpu.push_stack(cpu.pc.get());
    cpu.pc.set(address);

    CycleDuration::Default
}

pub fn call_c_nn(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional call to a subroutine at the absolute
    // 16-bit memory address a16 if the flag is set

    let flag = cpu.registers.flags.get_flag(flag);
    let address = cpu.get_nn_little_endian();

    if flag {
        cpu.push_stack(cpu.pc.get());
        cpu.pc.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn call_nc_nn(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional call to a subroutine at the absolute
    // 16-bit memory address a16 if the flag is set.

    let flag = cpu.registers.flags.get_flag(flag);
    let address = cpu.get_nn_little_endian();

    if !flag {
        cpu.push_stack(cpu.pc.get());
        cpu.pc.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn rst(cpu: &mut Cpu, address: u16) -> CycleDuration {
    // Unconditional function call to the absolute
    // fixed address defined by the opcode

    cpu.push_stack(cpu.pc.get());
    cpu.pc.set(address);

    CycleDuration::Default
}

pub fn ret(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional return from a function

    let address = cpu.pop_stack();
    cpu.pc.set(address);

    CycleDuration::Default
}

pub fn reti(cpu: &mut Cpu) -> CycleDuration {
    // Unconditional return from a function
    // Also enables interrupts by setting IME=1

    let address = cpu.pop_stack();
    cpu.pc.set(address);
    cpu.ime = true;

    CycleDuration::Default
}

pub fn ret_c(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional return from a function,
    // depending on the condition c

    let flag = cpu.registers.flags.get_flag(flag);

    if flag {
        let address = cpu.pop_stack();
        cpu.pc.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}

pub fn ret_nc(cpu: &mut Cpu, flag: Flag) -> CycleDuration {
    // Conditional return from a function,
    // depending on the condition nc

    let flag = cpu.registers.flags.get_flag(flag);

    if !flag {
        let address = cpu.pop_stack();
        cpu.pc.set(address);
        return CycleDuration::Optional;
    }

    CycleDuration::Default
}
