use std::fs::File;
use std::io::LineWriter;
use std::io::{Write, Error};
use std::{thread, time};

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
        let path = "log/lines.txt";
        let file = File::create(path).expect("Could not create File.");
        let mut file = LineWriter::new(file);

        loop {
            self.cpu.step(&mut file);
        }
    }
}
