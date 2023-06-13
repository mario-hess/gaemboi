use crate::cpu::Cpu;

pub fn daa(cpu: &mut Cpu) {
    // Decimal Adjust Accumulator to get a correct
    // BCD representation after an arithmetic instruction

    let subtract = cpu.registers.f.get_subtract();
    let half_carry = cpu.registers.f.get_half_carry();
    let carry = cpu.registers.f.get_carry();

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
            cpu.registers.f.set_carry(true);
        }

        if half_carry || (cpu.registers.get_a() & 0x0F) > 0x09 {
            cpu.registers
                .set_a(cpu.registers.get_a().wrapping_add(0x06));
        }
    }

    cpu.registers.f.set_zero(cpu.registers.get_a() == 0);
    cpu.registers.f.set_half_carry(false);
}

pub fn cpl(cpu: &mut Cpu) {
    // Flips all the bits in the 8-bit A register, and sets the N and H flags

    let a = cpu.registers.get_a();
    cpu.registers.set_a(!a);

    cpu.registers.f.set_subtract(true);
    cpu.registers.f.set_half_carry(true);
}

pub fn disable_interrupt(cpu: &mut Cpu) {
    // Disables interrupt handling by setting IME=0
    // and cancelling any scheduled effects of the EI
    // instruction if any

    cpu.interrupt_enabled = false;
}
