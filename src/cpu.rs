use crate::instruction::{Flag, Instruction, Mnemonic, Target};
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
        println!("-------------------------------------------------------------");
        self.execute_instruction(instruction);
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.mnemonic {
            Mnemonic::Nop => {}
            Mnemonic::Rst(address) => self.rst(address),
            Mnemonic::JPnn => self.jp_nn(),
            Mnemonic::CPn => self.cp_n(),
            Mnemonic::CALLnn => self.call_nn(),
            Mnemonic::INCPair(target) => self.inc_pair(target),
            Mnemonic::XORReg(target) => self.xor_reg(target),
            Mnemonic::LDRegReg(to, from) => self.ld_rr(to, from),
            Mnemonic::LDPairReg(pair_target, reg_target) => {
                self.ld_pair_reg(pair_target, reg_target)
            }
            Mnemonic::LDRegPair(reg_target, pair_target) => {
                self.ld_reg_pair(reg_target, pair_target)
            }
            Mnemonic::LDRegN(target) => self.ld_reg_n(target),
            Mnemonic::LDnnA => self.ld_nn_a(),
            Mnemonic::LDHnA => self.ldh_n_a(),
            Mnemonic::LDHAn => self.ldh_a_n(),
            Mnemonic::JRcce(flag) => self.jr_cc_e(flag),
            Mnemonic::JRe => self.jr_e(),
            Mnemonic::DI => self.di(),
            _ => panic!("Unknown mnemonic."),
        }
    }

    // --- Misc instructions ---
    fn rst(&mut self, address: u16) {
        // Unconditional function call to the absolute
        // fixed address defined by the opcode

        self.stack_pointer = self.stack_pointer.wrapping_sub(2);

        let pc = self.program_counter.next();
        let high_byte = (pc >> 8) as u8;
        let low_byte = pc as u8;

        self.memory_bus.write_byte(self.stack_pointer, low_byte);
        self.memory_bus
            .write_byte(self.stack_pointer.wrapping_add(1), high_byte);

        self.program_counter.set(address);
    }

    fn di(&mut self) {
        // Disables interrupt handling by setting IME=0
        // and cancelling any scheduled effects of the EI
        // instruction if any.

        self.interrupt_enabled = false;
    }

    // --- Jump instructions ---
    fn jp_nn(&mut self) {
        // Unconditional jump to the absolute address
        // specified by the 16-bit immediate values

        let address = self.get_nn_little_endian();

        self.program_counter.set(address);
    }

    fn jr_e(&mut self) {
        // Unconditional jump to the relative address
        // specified by the signed 8-bit immediate value

        let address = self.memory_bus.read_byte(self.program_counter.next()) as i8;

        self.program_counter.relative_jump(address);
    }

    fn jr_cc_e(&mut self, flag: Flag) {
        // Conditional jump to the relative address specified
        // by the signed 8-bit immediate value, depending on the
        // flag condition

        let address = self.memory_bus.read_byte(self.program_counter.next()) as i8;

        let flag = match flag {
            Flag::Z => self.registers.f.get_zero(),
            Flag::N => self.registers.f.get_subtract(),
            Flag::H => self.registers.f.get_half_carry(),
            Flag::C => self.registers.f.get_carry(),
        };

        if flag {
            self.program_counter.relative_jump(address);
        }
    }

    // --- Compare instructions ---
    fn cp_n(&mut self) {
        // Subtracts from the 8-bit A register, the immediate
        // data n, and updates flags based on the result.
        // This instructions basically identical to SUB n,
        // but does not update the A register

        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();

        let zero = a.wrapping_sub(byte) == 0;
        let half_carry = (a & 0x0F) < (byte & 0x0F);
        let carry = a < byte;

        self.registers.f.set_flags(zero, true, half_carry, carry);
    }

    // --- Increment instructions ---
    fn inc_pair(&mut self, target: Target) {
        // Increments data in the 16-bit target register by 1

        let value = self.registers.get_pair_value(&target);
        let set_reg = self.registers.get_pair_setter(&target);
        set_reg(&mut self.registers, value.wrapping_add(1));
    }

    // --- XOR instructions ---
    fn xor_reg(&mut self, target: Target) {
        // Performs a bitwise XOR operation between the
        // 8-bit A register and the 8-bit target register,
        // and stores the result back into the A register

        let a = self.registers.get_a();
        let value = self.registers.get_register_value(&target);

        let result = a ^ value;
        let flag = result == 0;

        self.registers.set_a(result);
        self.registers.f.set_flags(flag, false, false, false);
    }

    // --- Load instructions ---
    fn ld_rr(&mut self, to: Target, from: Target) {
        // 8-bit load instructions transfer one byte of data
        // between two 8-bit registers, or between one 8-bit
        // register and location in memory

        let set_reg = self.registers.get_register_setter(&to);
        let value = self.registers.get_register_value(&from);

        set_reg(&mut self.registers, value);
    }

    fn ld_pair_reg(&mut self, pair_target: Target, reg_target: Target) {
        // Load data from the 8-bit target register to the
        // absolute address specified by the 16-bit register

        let address = self.registers.get_pair_value(&pair_target);
        let value = self.registers.get_register_value(&reg_target);
        self.memory_bus.write_byte(address, value);
    }

    fn ld_reg_pair(&mut self, reg_target: Target, pair_target: Target) {
        // Load data from the absolute address specified
        // by the 16-bit register to the 8-bit register

        let address = self.registers.get_pair_value(&pair_target);
        let set_reg = self.registers.get_register_setter(&reg_target);
        let value = self.memory_bus.read_byte(address);
        set_reg(&mut self.registers, value);
    }

    fn ld_reg_n(&mut self, target: Target) {
        // Load the immediate 8-bit value to the 8-bit target register

        let byte = self.memory_bus.read_byte(self.program_counter.next());
        let set_reg = self.registers.get_register_setter(&target);
        set_reg(&mut self.registers, byte);
    }

    fn ld_nn_a(&mut self) {
        // Load data from the 8-bit A register to the absolute
        // address specified by the 16-bit immediate values

        let address = self.get_nn_little_endian();
        let a = self.registers.get_a();

        self.memory_bus.write_byte(address, a);
    }

    fn ldh_n_a(&mut self) {
        // Load to the address specified by the 8-bit immediate
        // data n, data from the 8-bit A register. The full 16-bit
        // absolute address is obtained by setting the most significant
        // byte to 0xFF and the least significant byte to the value of
        // n, so the possible range is 0xFF00-0xFFFF.

        let n = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let address = 0xFF00 | n;

        let value = self.registers.get_a();

        self.memory_bus.write_byte(address, value)
    }

    fn ldh_a_n(&mut self) {
        // Load to the 8-bit A register, data from the address specified
        // by the 8-bit immediate data n. The full 16-bit absolute address
        // is obtained by setting the most significant byte to 0xFF and
        // the least significant byte to the value of n, so the possible
        // range is 0xFF00-0xFFFF.

        let n = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let address = 0xFF00 | n;

        let value = self.memory_bus.read_byte(address);
        self.registers.set_a(value);
    }

    // --- Call instructions ---
    fn call_nn(&mut self) {
        // Unconditional function call to the absolute address
        // specified by the 16-bit operand nn

        let new_pc = self.get_nn_little_endian();
        let old_pc = self.program_counter.get();

        self.stack_pointer = self.stack_pointer.wrapping_sub(2);

        let msb = (old_pc >> 8) as u8;
        let lsb = old_pc as u8;

        self.memory_bus.write_byte(self.stack_pointer, msb);
        self.memory_bus
            .write_byte(self.stack_pointer.wrapping_add(1), lsb);

        self.program_counter.set(new_pc);
    }

    // --- Util ---
    fn get_nn_little_endian(&mut self) -> u16 {
        let low_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let high_byte = self.memory_bus.read_byte(self.program_counter.next()) as u16;

        (high_byte << 8) | low_byte
    }
}
