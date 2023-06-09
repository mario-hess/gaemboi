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
            Mnemonic::Rst(address) => self.rst(address),
            Mnemonic::JPnn => self.jp_nn(),
            Mnemonic::CPn => self.cp_n(),
            Mnemonic::CALLnn => self.call_nn(),
            Mnemonic::CALLFnn(flag) => self.call_f_nn(flag),
            Mnemonic::ANDn => self.and_n(),
            Mnemonic::AddReg(target) => self.add_reg(target),
            Mnemonic::INCReg(target) => self.inc_reg(target),
            Mnemonic::INCPair(target) => self.inc_pair(target),
            Mnemonic::DECPair(target) => self.dec_pair(target),
            Mnemonic::Subn => self.sub_n(),
            Mnemonic::POPPair(target) => self.pop_pair(target),
            Mnemonic::POPaf => self.pop_af(),
            Mnemonic::XORReg(target) => self.xor_reg(target),
            Mnemonic::LDRegReg(to, from) => self.ld_rr(to, from),
            Mnemonic::LDPairReg(pair_target, reg_target) => {
                self.ld_pair_reg(pair_target, reg_target)
            }
            Mnemonic::LDPairNN(target) => self.ld_pair_nn(target),
            Mnemonic::LDRegPair(reg_target, pair_target) => {
                self.ld_reg_pair(reg_target, pair_target)
            }
            Mnemonic::LDRegN(target) => self.ld_reg_n(target),
            Mnemonic::LDaHLp => self.ld_a_hl_p(),
            Mnemonic::LDnnA => self.ld_nn_a(),
            Mnemonic::LDHnA => self.ldh_n_a(),
            Mnemonic::LDHAn => self.ldh_a_n(),
            Mnemonic::LDSPnn => self.ld_sp_nn(),
            Mnemonic::LDSPhl => self.ld_sp_hl(),
            Mnemonic::LDaNN => self.ld_a_nn(),
            Mnemonic::JRce(flag) => self.jr_c_e(flag),
            Mnemonic::JRnce(flag) => self.jr_nc_e(flag),
            Mnemonic::JRe => self.jr_e(),
            Mnemonic::PushPair(target) => self.push_pair(target),
            Mnemonic::DisableInterrupt => self.disable_interrupt(),
            Mnemonic::Retc(flag) => self.ret_c(flag),
            Mnemonic::Retnc(flag) => self.ret_nc(flag),
            Mnemonic::Ret => self.ret(),
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

    // --- Misc instructions ---
    fn rst(&mut self, address: u16) {
        // Unconditional function call to the absolute
        // fixed address defined by the opcode

        self.push_stack(self.program_counter.get());
        self.program_counter.set(address);
    }

    fn disable_interrupt(&mut self) {
        // Disables interrupt handling by setting IME=0
        // and cancelling any scheduled effects of the EI
        // instruction if any

        self.interrupt_enabled = false;
    }

    // --- Sub / Add Instructions ---
    fn add_reg(&mut self, target: Target) {
        // Adds to the 8-bit A register, the 8-bit register r,
        // and stores the result back into the A register

        let r = self.registers.get_register_value(&target);
        let a = self.registers.get_a();

        let result = a.wrapping_add(r);
        self.registers.set_a(result);

        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(true);
        self.registers.f.set_half_carry((result & 0x0F) == 0);
        self.registers.f.set_carry(a < r);
    }

    fn sub_n(&mut self) {
        // Subtracts from the 8-bit A register, the
        // immediate data n, and stores the result
        // back into the A register.

        let n = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();
        let result = a.wrapping_sub(n);
        self.registers.set_a(result);

        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(true);
        self.registers.f.set_half_carry((result & 0x0F) == 0);
        self.registers.f.set_carry(a < n);
    }

    fn ret(&mut self) {
        // Unconditional return from a function

        let address = self.pop_stack();
        self.program_counter.set(address);
    }

    fn ret_c(&mut self, flag: Flag) {
        // Conditional return from a function,
        // depending on the condition c

        let flag = self.get_flag_value(flag);

        if flag {
            let address = self.pop_stack();
            self.program_counter.set(address);
        }
    }

    fn ret_nc(&mut self, flag: Flag) {
        // Conditional return from a function,
        // depending on the condition nc

        let flag = self.get_flag_value(flag);

        if !flag {
            let address = self.pop_stack();
            self.program_counter.set(address);
        }
    }

    // --- AND / OR instructions ---
    fn and_n(&mut self) {
        // Performs a bitwise AND operation between the
        // 8-bit A register and immediate data n, and
        // stores the result back into the A register

        let n = self.memory_bus.read_byte(self.program_counter.next());
        let a = self.registers.get_a();
        let result = a & n;
        self.registers.set_a(result);

        self.registers.f.set_flags(result == 0, false, true, false);
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

    fn jr_c_e(&mut self, flag: Flag) {
        // Conditional jump to the relative address specified
        // by the signed 8-bit immediate value, depending on the
        // flag condition

        let address = self.memory_bus.read_byte(self.program_counter.next()) as i8;
        let flag = self.get_flag_value(flag);

        if flag {
            self.program_counter.relative_jump(address);
        }
    }

    fn jr_nc_e(&mut self, flag: Flag) {
        // Conditional jump to the relative address specified
        // by the signed 8-bit immediate value, depending on the
        // flag condition

        let address = self.memory_bus.read_byte(self.program_counter.next()) as i8;
        let flag = self.get_flag_value(flag);

        if !flag {
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
    fn inc_reg(&mut self, target: Target) {
        // Increments data in the 8-bit register r

        let reg = self.registers.get_register_value(&target);
        let set_reg = self.registers.get_register_setter(&target);

        let result = reg.wrapping_add(1);
        set_reg(&mut self.registers, result);

        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(false);
        self.registers.f.set_half_carry((result & 0x0F) == 0);
    }

    fn inc_pair(&mut self, target: Target) {
        // Increments data in the 16-bit target register by 1

        let value = self.registers.get_pair_value(&target);
        let set_reg = self.registers.get_pair_setter(&target);
        set_reg(&mut self.registers, value.wrapping_add(1));
    }

    fn dec_pair(&mut self, target: Target) {
        // Decrements data in the 16-bittarget register

        let reg = self.registers.get_pair_value(&target);
        let set_reg = self.registers.get_pair_setter(&target);

        let result = reg.wrapping_sub(1);
        set_reg(&mut self.registers, result);
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

    fn ld_pair_nn(&mut self, target: Target) {
        // Load to the 16-bit register rr, the
        // immediate 16-bit data nn

        let value = self.get_nn_little_endian();
        let set_pair = self.registers.get_pair_setter(&target);
        set_pair(&mut self.registers, value);
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

    fn ld_a_hl_p(&mut self) {
        // Load to the 8-bit A register, data from the absolute
        // address specified by the 16-bit register HL. The value
        // of HL is incremented after the memory read

        let hl = self.registers.get_hl();
        let value = self.memory_bus.read_byte(hl);

        self.registers.set_a(value);
        self.registers.set_hl(hl.wrapping_add(1));
    }

    fn ld_a_nn(&mut self) {
        // Load to the 8-bit A register, data from the absolute
        // address specified by the 16-bit operand nn

        let address = self.get_nn_little_endian();
        let value = self.memory_bus.read_byte(address);

        self.registers.set_a(value);
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
        // n, so the possible range is 0xFF00-0xFFFF

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
        // range is 0xFF00-0xFFFF

        let n = self.memory_bus.read_byte(self.program_counter.next()) as u16;
        let address = 0xFF00 | n;

        let value = self.memory_bus.read_byte(address);
        self.registers.set_a(value);
    }

    fn ld_sp_nn(&mut self) {
        // loads the immediate 16-bit value into the stack pointer register

        let value = self.get_nn_little_endian();
        self.stack_pointer = value;
    }

    fn ld_sp_hl(&mut self) {
        // Load to the 16-bit SP register, data from the 16-bit HL register

        let hl = self.registers.get_hl();
        self.stack_pointer = hl;
    }

    fn push_pair(&mut self, target: Target) {
        // Push to the stack memory, data from the 16-bit register rr

        let value = self.registers.get_pair_value(&target);
        self.push_stack(value);
    }

    fn pop_pair(&mut self, target: Target) {
        // Pops to the 16-bit register rr, data from the stack memory

        let set_pair = self.registers.get_pair_setter(&target);
        let value = self.pop_stack();

        set_pair(&mut self.registers, value);
    }

    fn pop_af(&mut self) {
        // Pops to the 16-bit register rr, data from the stack memory.
        // Completely replaces the F register value, so all
        // flags are changed based on the 8-bit data that is read from memory

        let value = self.pop_stack();
        self.registers.set_af(value);
    }

    // --- Call instructions ---
    fn call_nn(&mut self) {
        // Unconditional function call to the absolute address
        // specified by the 16-bit operand nn

        let address = self.get_nn_little_endian();
        self.push_stack(self.program_counter.get());
        self.program_counter.set(address);
    }

    fn call_f_nn(&mut self, flag: Flag) {
        // conditional call to a subroutine at the absolute
        // 16-bit memory address a16 if the flag is set.

        let flag = self.get_flag_value(flag);

        if flag {
            let address = self.get_nn_little_endian();
            self.push_stack(self.program_counter.get());
            self.program_counter.set(address)
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