use crate::cpu::Cpu;

#[derive(Copy, Clone)]
pub struct Interrupt {
    interrupts: [(u8, u16); 5], // (bit_position, isr_address)
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            interrupts: [
                (0x01, 0x0040), // VBlank
                (0x02, 0x0048), // LCDStat
                (0x04, 0x0050), // Timer
                (0x08, 0x0058), // Serial
                (0x10, 0x0060), // Joypad
            ],
        }
    }

    pub fn interrupt_enabled(self, interrupt_enable: u8, interrupt_flag: u8) -> bool {
        let mut enabled = false;

        for (interrupt, _) in self.interrupts {
            enabled = self.is_enabled(interrupt_enable, interrupt_flag, interrupt);
        }

        enabled
    }

    pub fn is_enabled(self, interrupt_enable: u8, interrupt_flag: u8, value: u8) -> bool {
        let is_requested = interrupt_flag & value;
        let is_enabled = interrupt_enable & value;

        is_requested == value && is_enabled == value
    }

    pub fn handle_interrupts(self, cpu: &mut Cpu) -> Option<u8> {
        for (interrupt, isr_address) in self.interrupts {
            if self.handle_interrupt(cpu, interrupt, isr_address) {
                return Some(5);
            }
        }

        None
    }

    fn handle_interrupt(self, cpu: &mut Cpu, value: u8, isr_address: u16) -> bool {
        let interrupt_enable = cpu.memory_bus.interrupt_enable;
        let interrupt_flag = cpu.memory_bus.io.interrupt_flag;

        if !self.is_enabled(interrupt_enable, interrupt_flag, value) {
            return false;
        }

        cpu.interrupt_service_routine(isr_address, value);

        true
    }
}
