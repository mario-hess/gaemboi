use crate::cpu::Cpu;

pub fn disable_interrupt(cpu: &mut Cpu) {
    // Disables interrupt handling by setting IME=0
    // and cancelling any scheduled effects of the EI
    // instruction if any

    cpu.interrupt_enabled = false;
}
