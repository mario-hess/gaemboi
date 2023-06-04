use crate::instruction::{Instruction, Mnemonic, Target};
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
            Mnemonic::Nop => self.nop(instruction),
            Mnemonic::JumpNN => self.jump_nn(),
            Mnemonic::CpN => self.cp_n(instruction),
            Mnemonic::IncPair(ref target) => self.inc_pair(&instruction, target),
            Mnemonic::XorReg(ref target) => self.xor_reg(&instruction, target),
            Mnemonic::LoadRegToPairAddr(ref pair_target, ref reg_target) => {
                self.load_reg_to_pair_addr(&instruction, pair_target, reg_target)
            }
            Mnemonic::LoadNextToReg(ref target) => self.load_next_to_reg(&instruction, target),
            _ => panic!("Unknown mnemonic."),
        }
    }

    // Misc instructions

    fn nop(&mut self, instruction: Instruction) {
        // no operation
        self.program_counter.increment(instruction.length);
    }

    // Jump instructions

    fn jump_nn(&mut self) {
        // jump to nn, PC=nn
        let low_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let high_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let address = (high_byte << 8) | low_byte;

        self.program_counter.set(address);
    }

    // Compare instructions

    fn cp_n(&mut self, instruction: Instruction) {
        // compare A-n
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();

        let zero = a.wrapping_sub(byte) == 0;
        let half_carry = (a & 0x0F) < (byte & 0x0F);
        let carry = a < byte;

        self.registers.f.set_flags(zero, true, half_carry, carry);

        self.program_counter.increment(instruction.length);
    }

    // Increment instructions

    fn inc_pair(&mut self, instruction: &Instruction, target: &Target) {
        let value = self.registers.get_pair_value(target);
        let set_reg = self.registers.get_pair_setter(target);
        set_reg(&mut self.registers, value.wrapping_add(1));

        self.program_counter.increment(instruction.length);
    }

    // XOR instructions

    fn xor_reg(&mut self, instruction: &Instruction, target: &Target) {
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let value = self.registers.get_register_value(target);
        let set_reg = self.registers.get_register_setter(target);

        let result = value ^ byte;
        let flag = result == 0;

        set_reg(&mut self.registers, result);
        self.registers.f.set_flags(flag, false, false, false);

        self.program_counter.increment(instruction.length);
    }

    // Load instructions

    fn load_reg_to_pair_addr(
        &mut self,
        instruction: &Instruction,
        pair_target: &Target,
        reg_target: &Target,
    ) {
        let address = self.registers.get_pair_value(pair_target);
        let value = self.registers.get_register_value(reg_target);
        self.memory_bus.write_byte(address, value);

        self.program_counter.increment(instruction.length);
    }

    fn load_next_to_reg(&mut self, instruction: &Instruction, target: &Target) {
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let set_reg = self.registers.get_register_setter(target);
        set_reg(&mut self.registers, byte);

        self.program_counter.increment(instruction.length);
    }
}
