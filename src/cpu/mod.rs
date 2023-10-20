/**
 * @file    cpu/mod.rs
 * @brief   Overarching construct, facilitates instruction execution coordination.
 * @author  Mario Hess
 * @date    October 20, 2023
 */
mod arithmetic;
mod bit_ops;
mod control;
mod jump;
mod load;
mod program_counter;
mod rotate;
mod shift;

use crate::{
    cpu::program_counter::ProgramCounter,
    instruction::{CycleDuration, Instruction, Mnemonic},
    interrupt::Interrupt,
    memory_bus::MemoryBus,
    registers::Registers,
};

const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;
const STACK_POINTER_START: u16 = 0xFFFE;

pub struct Cpu {
    pub memory_bus: MemoryBus,
    registers: Registers,
    program_counter: ProgramCounter,
    stack_pointer: u16,
    interrupt: Interrupt,
    interrupt_master: bool,
    interrupt_master_state: bool,
    halted: bool,
    instruction: Option<Instruction>,
}

impl Cpu {
    pub fn new(rom_data: Vec<u8>) -> Self {
        // If the header checksum is 0x00, then the carry and
        // half-carry flags are clear; otherwise, they are both set.

        let flags_enable = rom_data[HEADER_CHECKSUM_ADDRESS] != 0x00;

        Self {
            memory_bus: MemoryBus::new(rom_data),
            registers: Registers::new(flags_enable),
            program_counter: ProgramCounter::new(),
            stack_pointer: STACK_POINTER_START,
            interrupt: Interrupt::new(),
            interrupt_master: false,
            interrupt_master_state: false,
            halted: false,
            instruction: None,
        }
    }

    pub fn tick(&mut self) -> u8 {
        let i_enable = self.memory_bus.get_interrupt_enable();
        let i_flag = self.memory_bus.get_interrupt_flag();

        if self.halted && self.interrupt.interrupt_enabled(i_enable, i_flag) {
            self.halted = false;
        } else if self.halted {
            // Halt consumes 1 m_cycle.
            return 1;
        }

        if self.interrupt_master {
            self.halted = false;
            if let Some(m_cycles) = self.interrupt.handle_interrupts(self) {
                return m_cycles;
            }
        }

        // interrupt_master is set after last instruction.
        self.interrupt_master = self.interrupt_master_state;

        let byte = self.memory_bus.read_byte(self.program_counter.next());
        self.instruction = Some(Instruction::from_byte(byte));

        let m_cycles = match self.instruction.unwrap().mnemonic {
            Mnemonic::Prefix => self.prefix_step(),
            _ => self.execute_instruction(self.instruction.unwrap()),
        };

        match m_cycles {
            CycleDuration::Default => self.instruction.unwrap().m_cycles,
            CycleDuration::Optional => self.instruction.unwrap().cond_m_cycles.unwrap(),
        }
    }

    fn prefix_step(&mut self) -> CycleDuration {
        let byte = self.memory_bus.read_byte(self.program_counter.next());
        self.instruction = Some(Instruction::from_prefix_byte(byte));
        self.execute_prefix(self.instruction.unwrap())
    }

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

    pub fn push_stack(&mut self, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory_bus.write_byte(self.stack_pointer, high_byte);

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory_bus.write_byte(self.stack_pointer, low_byte);
    }

    pub fn interrupt_service_routine(&mut self, isr_address: u16, value: u8) {
        self.interrupt_master = false;
        self.push_stack(self.program_counter.get());
        self.program_counter.set(isr_address);
        self.memory_bus.interrupt_flag &= value ^ 0xFF;
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> CycleDuration {
        match instruction.mnemonic {
            Mnemonic::NOP => CycleDuration::Default,
            Mnemonic::DAA => control::daa(self),
            Mnemonic::CPL => control::cpl(self),
            Mnemonic::SCF => control::scf(self),
            Mnemonic::CCF => control::ccf(self),
            Mnemonic::STOP => CycleDuration::Default,
            Mnemonic::HALT => {
                self.halted = true;
                CycleDuration::Default
            }
            Mnemonic::RST(address) => jump::rst(self, address),
            Mnemonic::JP_nn => jump::jp_nn(self),
            Mnemonic::JP_c_nn(flag) => jump::jp_c_nn(self, flag),
            Mnemonic::JP_nc_nn(flag) => jump::jp_nc_nn(self, flag),
            Mnemonic::JP_hl => jump::jp_hl(self),
            Mnemonic::CP_n => arithmetic::cp_n(self),
            Mnemonic::CP_r(target) => arithmetic::cp_r(self, target),
            Mnemonic::CP_hl => arithmetic::cp_hl(self),
            Mnemonic::CALL_nn => jump::call_nn(self),
            Mnemonic::CALL_c_nn(flag) => jump::call_c_nn(self, flag),
            Mnemonic::CALL_nc_nn(flag) => jump::call_nc_nn(self, flag),
            Mnemonic::AND_r(target) => arithmetic::and_r(self, target),
            Mnemonic::AND_n => arithmetic::and_n(self),
            Mnemonic::AND_hl => arithmetic::and_hl(self),
            Mnemonic::ADD_r(target) => arithmetic::add_r(self, target),
            Mnemonic::ADD_n => arithmetic::add_n(self),
            Mnemonic::ADD_a_hl => arithmetic::add_a_hl(self),
            Mnemonic::ADD_hl_rr(target) => arithmetic::add_hl_rr(self, target),
            Mnemonic::ADD_hl_sp => arithmetic::add_hl_sp(self),
            Mnemonic::ADD_sp_n => arithmetic::add_sp_n(self),
            Mnemonic::ADC_r(target) => arithmetic::adc_r(self, target),
            Mnemonic::ADC_n => arithmetic::adc_n(self),
            Mnemonic::ADC_hl => arithmetic::adc_hl(self),
            Mnemonic::INC_r(target) => arithmetic::inc_r(self, target),
            Mnemonic::INC_rr(target) => arithmetic::inc_rr(self, target),
            Mnemonic::INC_hl => arithmetic::inc_hl(self),
            Mnemonic::INC_sp => arithmetic::inc_sp(self),
            Mnemonic::DEC_r(target) => arithmetic::dec_r(self, target),
            Mnemonic::DEC_rr(target) => arithmetic::dec_rr(self, target),
            Mnemonic::DEC_sp => arithmetic::dec_sp(self),
            Mnemonic::DEC_hl => arithmetic::dec_hl(self),
            Mnemonic::SUB_n => arithmetic::sub_n(self),
            Mnemonic::SUB_r(target) => arithmetic::sub_r(self, target),
            Mnemonic::SUB_hl => arithmetic::sub_hl(self),
            Mnemonic::SBC_r(target) => arithmetic::sbc_r(self, target),
            Mnemonic::SBC_n => arithmetic::sbc_n(self),
            Mnemonic::SBC_hl => arithmetic::sbc_hl(self),
            Mnemonic::POP_rr(target) => load::pop_rr(self, target),
            Mnemonic::POP_af => load::pop_af(self),
            Mnemonic::OR_r(target) => arithmetic::or_r(self, target),
            Mnemonic::OR_n => arithmetic::or_n(self),
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
            Mnemonic::LD_hl_n => load::ld_hl_n(self),
            Mnemonic::LD_hl_plus_a => load::ld_hl_plus_a(self),
            Mnemonic::LD_hl_minus_a => load::ld_hl_minus_a(self),
            Mnemonic::LD_hl_sp_plus_n => load::ld_hl_sp_plus_n(self),
            Mnemonic::LD_a_hl_plus => load::ld_a_hl_plus(self),
            Mnemonic::LD_a_hl_minus => load::ld_a_hl_minus(self),
            Mnemonic::LD_nn_a => load::ld_nn_a(self),
            Mnemonic::LDH_n_a => load::ldh_n_a(self),
            Mnemonic::LDH_a_n => load::ldh_a_n(self),
            Mnemonic::LDH_a_c => load::ldh_a_c(self),
            Mnemonic::LD_sp_nn => load::ld_sp_nn(self),
            Mnemonic::LD_sp_hl => load::ld_sp_hl(self),
            Mnemonic::LD_nn_sp => load::ld_nn_sp(self),
            Mnemonic::LD_a_nn => load::ld_a_nn(self),
            Mnemonic::LD_c_a => load::ld_c_a(self),
            Mnemonic::JR_c_e(flag) => jump::jr_c_e(self, flag),
            Mnemonic::JR_nc_e(flag) => jump::jr_nc_e(self, flag),
            Mnemonic::JR_e => jump::jr_e(self),
            Mnemonic::PUSH_rr(target) => load::push_rr(self, target),
            Mnemonic::DisableInterrupt => control::disable_interrupt(self),
            Mnemonic::EnableInterrupt => control::enable_interrupt(self),
            Mnemonic::RRCA => rotate::rrca(self),
            Mnemonic::RRA => rotate::rra(self),
            Mnemonic::RLCA => rotate::rlca(self),
            Mnemonic::RLA => rotate::rla(self),
            Mnemonic::RET_c(flag) => jump::ret_c(self, flag),
            Mnemonic::RET_nc(flag) => jump::ret_nc(self, flag),
            Mnemonic::RET => jump::ret(self),
            Mnemonic::RETI => jump::reti(self),
            _ => panic!("Unknown mnemonic: {:?}.", instruction.mnemonic),
        }
    }

    fn execute_prefix(&mut self, instruction: Instruction) -> CycleDuration {
        match instruction.mnemonic {
            Mnemonic::RLC_r(target) => rotate::rlc_r(self, target),
            Mnemonic::RLC_hl => rotate::rlc_hl(self),
            Mnemonic::RRC_r(target) => rotate::rrc_r(self, target),
            Mnemonic::RRC_hl => rotate::rrc_hl(self),
            Mnemonic::RL_r(target) => rotate::rl_r(self, target),
            Mnemonic::RL_hl => rotate::rl_hl(self),
            Mnemonic::RR_r(target) => rotate::rr_r(self, target),
            Mnemonic::RR_hl => rotate::rr_hl(self),
            Mnemonic::SRL_r(target) => shift::srl_r(self, target),
            Mnemonic::SRL_hl => shift::srl_hl(self),
            Mnemonic::SLA_r(target) => shift::sla_r(self, target),
            Mnemonic::SLA_hl => shift::sla_hl(self),
            Mnemonic::SRA_r(target) => shift::sra_r(self, target),
            Mnemonic::SRA_hl => shift::sra_hl(self),
            Mnemonic::SWAP_r(target) => shift::swap_r(self, target),
            Mnemonic::SWAP_hl => shift::swap_hl(self),
            Mnemonic::BIT_r(position, target) => bit_ops::bit_r(self, position, target),
            Mnemonic::BIT_hl(position) => bit_ops::bit_hl(self, position),
            Mnemonic::RES_r(position, target) => bit_ops::res_r(self, position, target),
            Mnemonic::RES_hl(position) => bit_ops::res_hl(self, position),
            Mnemonic::SET_r(position, target) => bit_ops::set_r(self, position, target),
            Mnemonic::SET_hl(position) => bit_ops::set_hl(self, position),
            _ => panic!("Unknown prefix mnemonic: {:?}.", instruction.mnemonic),
        }
    }
}
