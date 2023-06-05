use crate::instruction::{Instruction, Mnemonic, Target, Flag};
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
        print!("PC: {:#X} | ", self.program_counter.value);
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        print!("Opcode: {:#X} | ", byte);
        let instruction = Instruction::from_byte(byte);
        println!("Instruction: {:?} | new PC: {:#X}", instruction.mnemonic, self.program_counter.value);
        println!("-------------------------------------------------------------");
        self.execute_instruction(instruction);
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::Nop => {},
            Mnemonic::Rst(address) => self.rst(address),
            Mnemonic::JPnn => self.jp_nn(),
            Mnemonic::CPn => self.cp_n(),
            Mnemonic::INCPair(target) => self.inc_pair(target),
            Mnemonic::XORReg(target) => self.xor_reg(target),
            Mnemonic::LDRegPair(pair_target, reg_target) => {
                self.ld_reg_pair(pair_target, reg_target);
            }
            Mnemonic::LDPairReg(reg_target, pair_target) => {
                self.ld_pair_reg(reg_target, pair_target);
            }
            Mnemonic::LoadNextToReg(target) => self.load_next_to_reg(target),
            Mnemonic::JRcce(flag) => self.jr_cc_e(flag),
            _ => panic!("Unknown mnemonic."),
        }
    }

    // Misc instructions
    fn rst(&mut self, address: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);

        let pc = self.program_counter.next();
        let high_byte = (pc >> 8) as u8;
        let low_byte = pc as u8;

        self.memory_bus.write_byte(self.stack_pointer, low_byte);
        self.memory_bus.write_byte(self.stack_pointer.wrapping_add(1), high_byte);

        self.program_counter.set(address);
    }

    // Jump instructions
    fn jp_nn(&mut self) {
        // jump to nn, PC=nn
        let low_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let high_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let address = (high_byte << 8) | low_byte;

        self.program_counter.set(address);
    }

    fn jr_cc_e(&mut self, flag: Flag) {
        let value = self.memory_bus.read_byte(self.program_counter.next()) as i8;

        let flag = match flag {
            Flag::Z => self.registers.f.get_zero(),
            Flag::N => self.registers.f.get_subtract(),
            Flag::H => self.registers.f.get_half_carry(),
            Flag::C => self.registers.f.get_carry(),
        };

        if flag {
            self.program_counter.increment_signed(value);
        }
    }

    // Compare instructions
    fn cp_n(&mut self) {
        // compare A-n
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();

        let zero = a.wrapping_sub(byte) == 0;
        let half_carry = (a & 0x0F) < (byte & 0x0F);
        let carry = a < byte;

        self.registers.f.set_flags(zero, true, half_carry, carry);
    }

    // Increment instructions
    fn inc_pair(&mut self, target: Target) {
        let value = self.registers.get_pair_value(&target);
        let set_reg = self.registers.get_pair_setter(&target);
        set_reg(&mut self.registers, value.wrapping_add(1));
    }

    // XOR instructions
    fn xor_reg(&mut self, target: Target) {
        let mut value = self.registers.get_register_value(&target);
        let set_reg = self.registers.get_register_setter(&target);

        value ^= value;
        let flag = value == 0;

        set_reg(&mut self.registers, value);
        self.registers.f.set_flags(flag, false, false, false);
    }

    // Load instructions
    fn ld_reg_pair(
        &mut self,
        pair_target: Target,
        reg_target: Target,
    ) {
        let address = self.registers.get_pair_value(&pair_target);
        let value = self.registers.get_register_value(&reg_target);
        self.memory_bus.write_byte(address, value);
    }

    fn ld_pair_reg(
        &mut self,
        reg_target: Target,
        pair_target: Target,
    ) {
        let address = self.registers.get_pair_value(&pair_target);
        let set_reg = self.registers.get_register_setter(&reg_target);
        let value = self.memory_bus.read_byte(address);
        set_reg(&mut self.registers, value);
    }

    fn load_next_to_reg(&mut self, target: Target) {
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let set_reg = self.registers.get_register_setter(&target);
        set_reg(&mut self.registers, byte);
    }
}
