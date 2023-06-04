use crate::instruction::{ArithmeticTarget, Instruction, Mnemonic};
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
            Mnemonic::IncPair(ref pair_target) => self.inc_pair(&instruction, pair_target),
            Mnemonic::XorReg(ref reg_target) => self.xor_reg(&instruction, reg_target),
            Mnemonic::LoadRegToPairAddr(ref pair_target, ref reg_target) => self
                .load_reg_to_pair_addr(
                    &instruction,
                    self.registers.get_pair_value(pair_target),
                    self.registers.get_register_value(reg_target),
                ),
            Mnemonic::LoadNextToReg(ref reg_target) => {
                self.load_next_to_reg(&instruction, reg_target)
            }
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
        let n = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();

        let zero = a.wrapping_sub(n) == 0;
        let half_carry = (a & 0x0F) < (n & 0x0F);
        let carry = a < n;

        self.registers.f.set_zero(zero);
        self.registers.f.set_subtract(true);
        self.registers.f.set_half_carry(half_carry);
        self.registers.f.set_carry(carry);

        self.program_counter.increment(instruction.length);
    }

    // Increment instructions

    fn inc_pair(&mut self, instruction: &Instruction, pair_target: &ArithmeticTarget) {
        match pair_target {
            ArithmeticTarget::BC => {
                let bc = self.registers.get_bc();
                self.registers.set_bc(bc.wrapping_add(1));
            }
            ArithmeticTarget::DE => {
                let de = self.registers.get_de();
                self.registers.set_de(de.wrapping_add(1));
            }
            ArithmeticTarget::HL => {
                let hl = self.registers.get_hl();
                self.registers.set_hl(hl.wrapping_add(1));
            }
            _ => panic!("inc_pair: ArithmeticTarget not found."),
        }
        self.program_counter.increment(instruction.length);
    }

    // XOR instructions

    fn xor_reg(&mut self, instruction: &Instruction, reg_target: &ArithmeticTarget) {
        let n = self.memory_bus.read_byte(self.program_counter.next());
        let reg = match reg_target {
            ArithmeticTarget::A => self.registers.get_a(),
            ArithmeticTarget::B => self.registers.get_b(),
            ArithmeticTarget::C => self.registers.get_c(),
            ArithmeticTarget::D => self.registers.get_d(),
            ArithmeticTarget::E => self.registers.get_e(),
            ArithmeticTarget::H => self.registers.get_h(),
            ArithmeticTarget::L => self.registers.get_l(),
            _ => unreachable!(),
        };

        let result = reg ^ n;
        let flag = result == 0;

        match reg_target {
            ArithmeticTarget::A => self.registers.set_a(result),
            ArithmeticTarget::B => self.registers.set_b(result),
            ArithmeticTarget::C => self.registers.set_c(result),
            ArithmeticTarget::D => self.registers.set_d(result),
            ArithmeticTarget::E => self.registers.set_e(result),
            ArithmeticTarget::H => self.registers.set_h(result),
            ArithmeticTarget::L => self.registers.set_l(result),
            _ => unreachable!(),
        };

        self.registers.f.set_zero(flag);
        self.registers.f.set_subtract(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_carry(false);

        self.program_counter.increment(instruction.length);
    }

    // Load instructions

    fn load_reg_to_pair_addr(
        &mut self,
        instruction: &Instruction,
        pair_address: u16,
        register: u8,
    ) {
        self.memory_bus.write_byte(pair_address, register);
        self.program_counter.increment(instruction.length);
    }

    fn load_next_to_reg(&mut self, instruction: &Instruction, reg_target: &ArithmeticTarget) {
        let n = self.memory_bus.read_byte(self.program_counter.next());
        match reg_target {
            ArithmeticTarget::A => self.registers.set_a(n),
            ArithmeticTarget::B => self.registers.set_b(n),
            ArithmeticTarget::C => self.registers.set_c(n),
            ArithmeticTarget::D => self.registers.set_d(n),
            ArithmeticTarget::E => self.registers.set_e(n),
            ArithmeticTarget::H => self.registers.set_h(n),
            ArithmeticTarget::L => self.registers.set_l(n),
            _ => panic!("load_next_to_reg: ArithmeticTarget not found."),
        }
        self.program_counter.increment(instruction.length);
    }
}
