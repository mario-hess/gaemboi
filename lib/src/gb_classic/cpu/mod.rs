pub mod clock;
pub mod instruction;
pub mod interrupt;
mod registers;
mod stack;

use std::error::Error;

use crate::gb_classic::{
    bus::Bus,
    cpu::{
        instruction::*,
        interrupt::Interrupt,
        registers::{program_counter::ProgramCounter, Registers},
        stack::Stack,
    },
};

pub struct Cpu {
    pub registers: Registers,
    pub program_counter: ProgramCounter,
    pub stack: Stack,
    interrupt: Interrupt,
    ime: bool,
    ime_scheduled: bool,
    halted: bool,
}

impl Cpu {
    pub fn new(flags_enabled: bool) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            registers: Registers::new(flags_enabled),
            program_counter: ProgramCounter::new(),
            stack: Stack::new(),
            interrupt: Interrupt::new(),
            ime: false,
            ime_scheduled: false,
            halted: false,
        })
    }

    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let interrupt_enabled = bus.get_interrupt_enabled();
        let interrupt_flag = bus.get_interrupt_flag();

        // The HALT instruction gives the game the ability to stop the CPU
        // from executing any more instructions until an interrupt gets enabled
        if self.halted
            && self
                .interrupt
                .interrupt_enabled(interrupt_enabled, interrupt_flag)
        {
            self.halted = false;
        } else if self.halted {
            // Halt consumes 1 m_cycle
            return 1;
        }

        if self.ime {
            self.halted = false;
            if let Some(m_cycles) = self.handle_interrupts(bus) {
                return m_cycles;
            }
        }

        let byte = bus.read_byte(self.program_counter.next());
        let instruction = Instruction::from_byte(byte);

        // Check if mnemonic refers to the prefix table
        let cycles = match instruction.mnemonic {
            Mnemonic::Prefix => self.prefix_step(bus),
            _ => {
                let cycle_duration = self.execute_instruction(instruction, bus);
                match cycle_duration {
                    CycleDuration::Default => instruction.m_cycles,
                    CycleDuration::Optional => instruction.opt_m_cycles.unwrap(),
                }
            }
        };

        if self.ime_scheduled {
            self.ime = true;
            self.ime_scheduled = false;
        }

        cycles
    }

    // Handle next instruction from prefix table
    fn prefix_step(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(self.program_counter.next());
        let instruction = Instruction::from_prefix_byte(byte);
        let cycle_duration = self.execute_prefix(instruction, bus);

        match cycle_duration {
            CycleDuration::Default => instruction.m_cycles,
            CycleDuration::Optional => instruction.opt_m_cycles.unwrap(),
        }
    }

    // Multi-byte data is handled in little-endian format
    fn get_nn_little_endian(&mut self, bus: &Bus) -> u16 {
        let low_byte = bus.read_byte(self.program_counter.next()) as u16;
        let high_byte = bus.read_byte(self.program_counter.next()) as u16;

        (high_byte << 8) | low_byte
    }

    // https://gbdev.io/pandocs/Interrupts.html#interrupt-handling
    pub fn handle_interrupts(&mut self, bus: &mut Bus) -> Option<u8> {
        for (interrupt, isr_address) in self.interrupt.get_interrupts() {
            if self
                .interrupt
                .handle_interrupt(bus.interrupt_enabled, bus.interrupt_flag, interrupt)
            {
                self.interrupt_service_routine(bus, isr_address, interrupt);
                return Some(5);
            }
        }

        None
    }

    pub fn interrupt_service_routine(&mut self, bus: &mut Bus, isr_address: u16, value: u8) {
        self.ime = false;
        self.stack.push(bus, self.program_counter.get());
        self.program_counter.set(isr_address);
        bus.interrupt_flag &= !value;
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
        bus: &mut Bus,
    ) -> CycleDuration {
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
            Mnemonic::RST(address) => jump::rst(self, bus, address),
            Mnemonic::JP_nn => jump::jp_nn(self, bus),
            Mnemonic::JP_c_nn(flag) => jump::jp_c_nn(self, bus, flag),
            Mnemonic::JP_nc_nn(flag) => jump::jp_nc_nn(self, bus, flag),
            Mnemonic::JP_hl => jump::jp_hl(self),
            Mnemonic::CP_n => arithmetic::cp_n(self, bus),
            Mnemonic::CP_r(target) => arithmetic::cp_r(self, target),
            Mnemonic::CP_hl => arithmetic::cp_hl(self, bus),
            Mnemonic::CALL_nn => jump::call_nn(self, bus),
            Mnemonic::CALL_c_nn(flag) => jump::call_c_nn(self, bus, flag),
            Mnemonic::CALL_nc_nn(flag) => jump::call_nc_nn(self, bus, flag),
            Mnemonic::AND_r(target) => arithmetic::and_r(self, target),
            Mnemonic::AND_n => arithmetic::and_n(self, bus),
            Mnemonic::AND_hl => arithmetic::and_hl(self, bus),
            Mnemonic::ADD_r(target) => arithmetic::add_r(self, target),
            Mnemonic::ADD_n => arithmetic::add_n(self, bus),
            Mnemonic::ADD_a_hl => arithmetic::add_a_hl(self, bus),
            Mnemonic::ADD_hl_rr(target) => arithmetic::add_hl_rr(self, target),
            Mnemonic::ADD_hl_sp => arithmetic::add_hl_sp(self),
            Mnemonic::ADD_sp_n => arithmetic::add_sp_n(self, bus),
            Mnemonic::ADC_r(target) => arithmetic::adc_r(self, target),
            Mnemonic::ADC_n => arithmetic::adc_n(self, bus),
            Mnemonic::ADC_hl => arithmetic::adc_hl(self, bus),
            Mnemonic::INC_r(target) => arithmetic::inc_r(self, target),
            Mnemonic::INC_rr(target) => arithmetic::inc_rr(self, target),
            Mnemonic::INC_hl => arithmetic::inc_hl(self, bus),
            Mnemonic::INC_sp => arithmetic::inc_sp(self),
            Mnemonic::DEC_r(target) => arithmetic::dec_r(self, target),
            Mnemonic::DEC_rr(target) => arithmetic::dec_rr(self, target),
            Mnemonic::DEC_sp => arithmetic::dec_sp(self),
            Mnemonic::DEC_hl => arithmetic::dec_hl(self, bus),
            Mnemonic::SUB_n => arithmetic::sub_n(self, bus),
            Mnemonic::SUB_r(target) => arithmetic::sub_r(self, target),
            Mnemonic::SUB_hl => arithmetic::sub_hl(self, bus),
            Mnemonic::SBC_r(target) => arithmetic::sbc_r(self, target),
            Mnemonic::SBC_n => arithmetic::sbc_n(self, bus),
            Mnemonic::SBC_hl => arithmetic::sbc_hl(self, bus),
            Mnemonic::POP_rr(target) => load::pop_rr(self, bus, target),
            Mnemonic::POP_af => load::pop_af(self, bus),
            Mnemonic::OR_r(target) => arithmetic::or_r(self, target),
            Mnemonic::OR_n => arithmetic::or_n(self, bus),
            Mnemonic::OR_hl => arithmetic::or_hl(self, bus),
            Mnemonic::XOR_r(target) => arithmetic::xor_r(self, target),
            Mnemonic::XOR_n => arithmetic::xor_n(self, bus),
            Mnemonic::XOR_hl => arithmetic::xor_hl(self, bus),
            Mnemonic::LD_r_r(to, from) => load::ld_r_r(self, to, from),
            Mnemonic::LD_rr_r(pair_target, reg_target) => {
                load::ld_rr_r(self, bus, pair_target, reg_target)
            }
            Mnemonic::LD_rr_nn(target) => load::ld_rr_nn(self, bus, target),
            Mnemonic::LD_r_rr(reg_target, pair_target) => {
                load::ld_r_rr(self, bus, reg_target, pair_target)
            }
            Mnemonic::LD_r_n(target) => load::ld_r_n(self, bus, target),
            Mnemonic::LD_hl_n => load::ld_hl_n(self, bus),
            Mnemonic::LD_hl_plus_a => load::ld_hl_plus_a(self, bus),
            Mnemonic::LD_hl_minus_a => load::ld_hl_minus_a(self, bus),
            Mnemonic::LD_hl_sp_plus_n => load::ld_hl_sp_plus_n(self, bus),
            Mnemonic::LD_a_hl_plus => load::ld_a_hl_plus(self, bus),
            Mnemonic::LD_a_hl_minus => load::ld_a_hl_minus(self, bus),
            Mnemonic::LD_nn_a => load::ld_nn_a(self, bus),
            Mnemonic::LDH_n_a => load::ldh_n_a(self, bus),
            Mnemonic::LDH_a_n => load::ldh_a_n(self, bus),
            Mnemonic::LDH_a_c => load::ldh_a_c(self, bus),
            Mnemonic::LD_sp_nn => load::ld_sp_nn(self, bus),
            Mnemonic::LD_sp_hl => load::ld_sp_hl(self),
            Mnemonic::LD_nn_sp => load::ld_nn_sp(self, bus),
            Mnemonic::LD_a_nn => load::ld_a_nn(self, bus),
            Mnemonic::LD_c_a => load::ld_c_a(self, bus),
            Mnemonic::JR_c_e(flag) => jump::jr_c_e(self, bus, flag),
            Mnemonic::JR_nc_e(flag) => jump::jr_nc_e(self, bus, flag),
            Mnemonic::JR_e => jump::jr_e(self, bus),
            Mnemonic::PUSH_rr(target) => load::push_rr(self, bus, target),
            Mnemonic::DisableInterrupt => control::disable_interrupt(self),
            Mnemonic::EnableInterrupt => control::enable_interrupt(self),
            Mnemonic::RRCA => rotate::rrca(self),
            Mnemonic::RRA => rotate::rra(self),
            Mnemonic::RLCA => rotate::rlca(self),
            Mnemonic::RLA => rotate::rla(self),
            Mnemonic::RET_c(flag) => jump::ret_c(self, bus, flag),
            Mnemonic::RET_nc(flag) => jump::ret_nc(self, bus, flag),
            Mnemonic::RET => jump::ret(self, bus),
            Mnemonic::RETI => jump::reti(self, bus),
            _ => panic!("Unknown mnemonic: {:?}.", instruction.mnemonic),
        }
    }

    fn execute_prefix(&mut self, instruction: Instruction, bus: &mut Bus) -> CycleDuration {
        match instruction.mnemonic {
            Mnemonic::RLC_r(target) => rotate::rlc_r(self, target),
            Mnemonic::RLC_hl => rotate::rlc_hl(self, bus),
            Mnemonic::RRC_r(target) => rotate::rrc_r(self, target),
            Mnemonic::RRC_hl => rotate::rrc_hl(self, bus),
            Mnemonic::RL_r(target) => rotate::rl_r(self, target),
            Mnemonic::RL_hl => rotate::rl_hl(self, bus),
            Mnemonic::RR_r(target) => rotate::rr_r(self, target),
            Mnemonic::RR_hl => rotate::rr_hl(self, bus),
            Mnemonic::SRL_r(target) => shift::srl_r(self, target),
            Mnemonic::SRL_hl => shift::srl_hl(self, bus),
            Mnemonic::SLA_r(target) => shift::sla_r(self, target),
            Mnemonic::SLA_hl => shift::sla_hl(self, bus),
            Mnemonic::SRA_r(target) => shift::sra_r(self, target),
            Mnemonic::SRA_hl => shift::sra_hl(self, bus),
            Mnemonic::SWAP_r(target) => shift::swap_r(self, target),
            Mnemonic::SWAP_hl => shift::swap_hl(self, bus),
            Mnemonic::BIT_r(position, target) => bit_ops::bit_r(self, position, target),
            Mnemonic::BIT_hl(position) => bit_ops::bit_hl(self, bus, position),
            Mnemonic::RES_r(position, target) => bit_ops::res_r(self, position, target),
            Mnemonic::RES_hl(position) => bit_ops::res_hl(self, bus, position),
            Mnemonic::SET_r(position, target) => bit_ops::set_r(self, position, target),
            Mnemonic::SET_hl(position) => bit_ops::set_hl(self, bus, position),
            _ => panic!("Unknown prefix mnemonic: {:?}.", instruction.mnemonic),
        }
    }
}
