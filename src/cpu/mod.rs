mod arithmetic;
mod control;
mod jump;
mod load;
mod program_counter;
mod reset;
mod rotate;
mod shift;

use crate::cpu::program_counter::ProgramCounter;
use crate::instruction::{Flag, Instruction, Mnemonic, Target};
use crate::memory_bus::MemoryBus;
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
        //print!("PC: {:#X} | ", self.program_counter.get());
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        //print!("Opcode: {:#X} | ", byte);
        let instruction = Instruction::from_byte(byte);
        //println!(
        //    "Instruction: {:?} | new PC: {:#X}",
        //    instruction.mnemonic, self.program_counter.value
        //);
        self.execute_instruction(instruction);
        //println!(
        //    "AF: {:#X}, BC: {:#X}, DE: {:#X}, HL: {:#X}",
        //    self.registers.get_af(),
        //    self.registers.get_bc(),
        //    self.registers.get_de(),
        //    self.registers.get_hl()
        //);
        //println!(
        //    "Char at 0xFF01: [{}], Val at 0xFF02: [{:#X}]",
        //    char::from(self.memory_bus.io[1]),
        //    self.memory_bus.io[2]
        //);
        //println!("-------------------------------------------------------------");
        //if self.memory_bus.io[1] != 0xA {
        //    print!("{:#X}, ", self.memory_bus.io[1]);
        //}
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::NOP => {}
            Mnemonic::RST(address) => jump::rst(self, address),
            Mnemonic::JP_nn => jump::jp_nn(self),
            Mnemonic::JP_hl => jump::jp_hl(self),
            Mnemonic::CP_n => arithmetic::cp_n(self),
            Mnemonic::CALL_nn => jump::call_nn(self),
            Mnemonic::CALL_c_nn(flag) => jump::call_c_nn(self, flag),
            Mnemonic::CALL_nc_nn(flag) => jump::call_nc_nn(self, flag),
            Mnemonic::AND_n => arithmetic::and_n(self),
            Mnemonic::ADD_r(target) => arithmetic::add_r(self, target),
            Mnemonic::ADD_n => arithmetic::add_n(self),
            Mnemonic::ADD_hl_rr(target) => arithmetic::add_hl_rr(self, target),
            Mnemonic::ADC_r(target) => arithmetic::adc_r(self, target),
            Mnemonic::ADC_n => arithmetic::adc_n(self),
            Mnemonic::INC_r(target) => arithmetic::inc_r(self, target),
            Mnemonic::INC_rr(target) => arithmetic::inc_rr(self, target),
            Mnemonic::DEC_r(target) => arithmetic::dec_r(self, target),
            Mnemonic::DEC_rr(target) => arithmetic::dec_rr(self, target),
            Mnemonic::DEC_hl => arithmetic::dec_hl(self),
            Mnemonic::SUB_n => arithmetic::sub_n(self),
            Mnemonic::POP_rr(target) => load::pop_rr(self, target),
            Mnemonic::POP_af => load::pop_af(self),
            Mnemonic::OR_r(target) => arithmetic::or_r(self, target),
            Mnemonic::OR_hl => arithmetic::or_hl(self),
            Mnemonic::XOR_r(target) => arithmetic::xor_r(self, target),
            Mnemonic::XOR_n => arithmetic::xor_n(self),
            Mnemonic::XOR_hl => arithmetic::xor_hl(self),
            Mnemonic::LD_r_r(to, from) => load::ld_r_r(self, to, from),
            Mnemonic::LD_rr_r(pair_target, reg_target) => {
                load::ld_rr_r(self, pair_target, reg_target)
            }
            Mnemonic::LD_rr_nn(target) => load::ld_rr_nn(self, target),
            Mnemonic::LD_r_rr(reg_target, pair_target) => {
                load::ld_r_rr(self, reg_target, pair_target)
            }
            Mnemonic::LD_r_n(target) => load::ld_r_n(self, target),
            Mnemonic::LD_hl_plus_a => load::ld_hl_plus_a(self),
            Mnemonic::LD_hl_minus_a => load::ld_hl_minus_a(self),
            Mnemonic::LD_a_hl_plus => load::ld_a_hl_plus(self),
            Mnemonic::LD_nn_a => load::ld_nn_a(self),
            Mnemonic::LDH_n_a => load::ldh_n_a(self),
            Mnemonic::LDH_a_n => load::ldh_a_n(self),
            Mnemonic::LD_sp_nn => load::ld_sp_nn(self),
            Mnemonic::LD_sp_hl => load::ld_sp_hl(self),
            Mnemonic::LD_a_nn => load::ld_a_nn(self),
            Mnemonic::JR_c_e(flag) => jump::jr_c_e(self, flag),
            Mnemonic::JR_nc_e(flag) => jump::jr_nc_e(self, flag),
            Mnemonic::JR_e => jump::jr_e(self),
            Mnemonic::PUSH_rr(target) => load::push_rr(self, target),
            Mnemonic::DisableInterrupt => control::disable_interrupt(self),
            Mnemonic::RRA => rotate::rra(self),
            Mnemonic::RLCA => rotate::rlca(self),
            Mnemonic::RET_c(flag) => jump::ret_c(self, flag),
            Mnemonic::RET_nc(flag) => jump::ret_nc(self, flag),
            Mnemonic::RET => jump::ret(self),
            Mnemonic::Prefix => self.prefix(),
            _ => panic!("Unknown mnemonic."),
        }
    }

    fn prefix(&mut self) {
        //print!("PC: {:#X} | ", self.program_counter.get());
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        //print!("Opcode: {:#X} | ", byte);
        let instruction = Instruction::from_prefix(byte);
        //println!(
        //   "Instruction: {:?} | new PC: {:#X}",
        //   instruction.mnemonic, self.program_counter.value
        //);
        self.execute_prefix(instruction);
    }

    fn execute_prefix(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::RR_r(target) => rotate::rr_r(self, target),
            Mnemonic::RES_b_r(value, target) => reset::res_b_r(self, value, target),
            Mnemonic::SRL_r(target) => shift::srl_r(self, target),
            _ => panic!("Unknown PREFIX Mnemnoic."),
        }
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
