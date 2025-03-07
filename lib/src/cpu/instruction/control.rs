use crate::cpu::{instruction::CycleDuration, Cpu};

// Decimal Adjust Accumulator to get a correct
// BCD representation after an arithmetic instruction
pub fn daa(cpu: &mut Cpu) -> CycleDuration {
    let subtract = cpu.registers.flags.get_subtract();
    let half_carry = cpu.registers.flags.get_half_carry();
    let carry = cpu.registers.flags.get_carry();

    if subtract {
        if carry {
            cpu.registers
                .set_a(cpu.registers.get_a().wrapping_sub(0x60));
        }

        if half_carry {
            cpu.registers
                .set_a(cpu.registers.get_a().wrapping_sub(0x06));
        }
    } else {
        if carry || cpu.registers.get_a() > 0x99 {
            cpu.registers
                .set_a(cpu.registers.get_a().wrapping_add(0x60));
            cpu.registers.flags.set_carry(true);
        }

        if half_carry || (cpu.registers.get_a() & 0x0F) > 0x09 {
            cpu.registers
                .set_a(cpu.registers.get_a().wrapping_add(0x06));
        }
    }

    cpu.registers.flags.set_zero(cpu.registers.get_a() == 0);
    cpu.registers.flags.set_half_carry(false);

    CycleDuration::Default
}

// Flips all the bits in the 8-bit A register, and sets the N and H flags
pub fn cpl(cpu: &mut Cpu) -> CycleDuration {
    let a = cpu.registers.get_a();
    cpu.registers.set_a(!a);

    cpu.registers.flags.set_subtract(true);
    cpu.registers.flags.set_half_carry(true);

    CycleDuration::Default
}

// Sets the carry flag, and clears the N and H flags
pub fn scf(cpu: &mut Cpu) -> CycleDuration {
    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(true);

    CycleDuration::Default
}

// Flips the carry flag, and clears the N and H flags
pub fn ccf(cpu: &mut Cpu) -> CycleDuration {
    let carry = !cpu.registers.flags.get_carry();

    cpu.registers.flags.set_subtract(false);
    cpu.registers.flags.set_half_carry(false);
    cpu.registers.flags.set_carry(carry);

    CycleDuration::Default
}

// Disables interrupt handling by setting IME=0
// and cancelling any scheduled effects of the EI
// instruction if any
pub fn disable_interrupt(cpu: &mut Cpu) -> CycleDuration {
    cpu.ime = false;
    cpu.ime_scheduled = false;

    CycleDuration::Default
}

pub fn enable_interrupt(cpu: &mut Cpu) -> CycleDuration {
    cpu.ime_scheduled = true;

    CycleDuration::Default
}
