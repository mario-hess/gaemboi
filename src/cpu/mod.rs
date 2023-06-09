mod program_counter;
mod arithmetic;
mod jump;
mod load;
mod control;

use crate::instruction::{Flag, Instruction, Mnemonic, Target};
use crate::memory_bus::MemoryBus;
use crate::cpu::program_counter::ProgramCounter;
use crate::registers::Registers;

const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;
const STACK_POINTER_START: u16 = 0xFFFE;

pub struct Cpu {
    memory_bus: MemoryBus,
    registers: Registers,
    program_counter: ProgramCounter,
    stack_pointer: u16,
    interrupt_enabled: bool,
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
            interrupt_enabled: false,
        }
    }

    pub fn step(&mut self) {
        print!("PC: {:#X} | ", self.program_counter.get());
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        print!("Opcode: {:#X} | ", byte);
        let instruction = Instruction::from_byte(byte);
        println!(
            "Instruction: {:?} | new PC: {:#X}",
            instruction.mnemonic, self.program_counter.value
        );
        self.execute_instruction(instruction);
        println!(
            "AF: {:#X}, BC: {:#X}, DE: {:#X}, HL: {:#X}",
            self.registers.get_af(),
            self.registers.get_bc(),
            self.registers.get_de(),
            self.registers.get_hl()
        );
        println!(
            "Char at 0xFF01: [{}], Val at 0xFF02: [{:#X}]",
            char::from(self.memory_bus.io[1]),
            self.memory_bus.io[2]
        );
        println!("-------------------------------------------------------------");
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::Nop => {}
            Mnemonic::Rst(address) => jump::rst(self, address),
            Mnemonic::JPnn => jump::jp_nn(self),
            Mnemonic::CPn => arithmetic::cp_n(self),
            Mnemonic::CALLnn => jump::call_nn(self),
            Mnemonic::CALLFnn(flag) => jump::call_f_nn(self, flag),
            Mnemonic::CALLNFnn(flag) => jump::call_nf_nn(self, flag),
            Mnemonic::ANDn => arithmetic::and_n(self),
            Mnemonic::AddReg(target) => arithmetic::add_reg(self, target),
            Mnemonic::INCReg(target) => arithmetic::inc_reg(self, target),
            Mnemonic::INCPair(target) => arithmetic::inc_pair(self, target),
            Mnemonic::DECPair(target) => arithmetic::dec_pair(self, target),
            Mnemonic::Subn => arithmetic::sub_n(self),
            Mnemonic::POPPair(target) => load::pop_pair(self, target),
            Mnemonic::POPaf => load::pop_af(self),
            Mnemonic::ORReg(target) => arithmetic::or_reg(self, target),
            Mnemonic::XORReg(target) => arithmetic::xor_reg(self, target),
            Mnemonic::LDRegReg(to, from) => load::ld_rr(self, to, from),
            Mnemonic::LDPairReg(pair_target, reg_target) => {
                load::ld_pair_reg(self, pair_target, reg_target)
            }
            Mnemonic::LDPairNN(target) => load::ld_pair_nn(self, target),
            Mnemonic::LDRegPair(reg_target, pair_target) => {
                load::ld_reg_pair(self, reg_target, pair_target)
            }
            Mnemonic::LDRegN(target) => load::ld_reg_n(self, target),
            Mnemonic::LDaHLp => load::ld_a_hl_p(self),
            Mnemonic::LDnnA => load::ld_nn_a(self),
            Mnemonic::LDHnA => load::ldh_n_a(self),
            Mnemonic::LDHAn => load::ldh_a_n(self),
            Mnemonic::LDSPnn => load::ld_sp_nn(self),
            Mnemonic::LDSPhl => load::ld_sp_hl(self),
            Mnemonic::LDaNN => load::ld_a_nn(self),
            Mnemonic::JRce(flag) => jump::jr_c_e(self, flag),
            Mnemonic::JRnce(flag) => jump::jr_nc_e(self, flag),
            Mnemonic::JRe => jump::jr_e(self),
            Mnemonic::PushPair(target) => load::push_pair(self, target),
            Mnemonic::DisableInterrupt => control::disable_interrupt(self),
            Mnemonic::Retc(flag) => jump::ret_c(self, flag),
            Mnemonic::Retnc(flag) => jump::ret_nc(self, flag),
            Mnemonic::Ret => jump::ret(self),
            Mnemonic::Prefix => self.prefix(),
            _ => panic!("Unknown mnemonic."),
        }
    }

    fn prefix(&mut self) {
        print!("PC: {:#X} | ", self.program_counter.get());
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        print!("Opcode: {:#X} | ", byte);
        let instruction = Instruction::from_prefix(byte);
        println!(
            "Instruction: {:?} | new PC: {:#X}",
            instruction.mnemonic, self.program_counter.value
        );
        self.execute_prefix(instruction);
    }

    fn execute_prefix(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::ResBReg(value, target) => self.res_b_reg(value, target),
            _ => panic!("Unknown PREFIX Mnemnoic."),
        }
    }


    // --- PREFIX CB ---
    // --- Reset instructions ---
    fn res_b_reg(&mut self, bit: u8, target: Target) {
        // clear bit of the target register

        let reg = self.registers.get_register_value(&target);
        let set_reg = self.registers.get_register_setter(&target);

        let result = reg & !(1 << bit);

        set_reg(&mut self.registers, result);
    }

    // --- Util ---
    fn get_nn_little_endian(&mut self) -> u16 {
        let low_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let high_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;

        (high_byte << 8) | low_byte
    }

    fn pop_stack(&mut self) -> u16 {
        let low_byte = self.memory_bus.read_byte(self.stack_pointer) as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        let high_byte = self.memory_bus.read_byte(self.stack_pointer) as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        (high_byte << 8) | low_byte
    }

    fn push_stack(&mut self, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory_bus.write_byte(self.stack_pointer, high_byte);

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory_bus.write_byte(self.stack_pointer, low_byte);
    }

    fn get_flag_value(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.registers.f.get_zero(),
            Flag::N => self.registers.f.get_subtract(),
            Flag::H => self.registers.f.get_half_carry(),
            Flag::C => self.registers.f.get_carry(),
        }
    }
}
