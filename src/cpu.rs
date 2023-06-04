use crate::instruction::{Instruction, Mnemonic};
use crate::memory_bus::MemoryBus;
use crate::program_counter::ProgramCounter;
use crate::registers::Registers;

const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;
const STACK_POINTER_START: u16 = 0xFFFE;

pub struct Cpu {
    memory_bus: MemoryBus,
    registers: Registers,
    program_counter: ProgramCounter,
    stack_pointer: u16,
}

impl Cpu {
    pub fn new(rom_data: Vec<u8>) -> Self {
        // If the header checksum is 0x00, then the carry and
        // half-carry flags are clear; otherwise, they are both set
        let enable_flags = rom_data[HEADER_CHECKSUM_ADDRESS] != 0x00;

        Self {
            memory_bus: MemoryBus::new(rom_data),
            registers: Registers::new(enable_flags),
            program_counter: ProgramCounter::new(),
            stack_pointer: STACK_POINTER_START,
        }
    }

    pub fn step(&mut self) {
        let byte = self.memory_bus.read_byte(self.program_counter.value);
        let instruction = Instruction::from_byte(byte);
        self.execute_instruction(instruction);
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::Nop => {
                // no operation
                self.program_counter.increment(instruction.length);
            }
            Mnemonic::JumpNN => {
                // jump to nn, PC=nn
                let low_byte = self.memory_bus.read_byte(self.program_counter.get() + 1) as u16;
                let high_byte = self.memory_bus.read_byte(self.program_counter.get() + 2) as u16;

                let address = (high_byte << 8) | low_byte;

                self.program_counter.set(address);
            }
            _ => panic!("Unknown mnemonic."),
        }
    }
}
