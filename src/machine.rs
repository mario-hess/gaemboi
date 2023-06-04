use crate::cpu::Cpu;

pub struct Machine {
    cpu: Cpu,
}

impl Machine {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom_data),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}
