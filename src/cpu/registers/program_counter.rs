/*
 * @file    cpu/program_counter.rs
 * @brief   Controls the CPU's program counter.
 * @author  Mario Hess
 * @date    November 11, 2023
 */

const BOOT_ROM_END: u16 = 0x100;

pub struct ProgramCounter {
    pub value: u16,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: BOOT_ROM_END }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn relative_jump(&mut self, value: i8) {
        let new_value = ((self.value as i32).wrapping_add(value as i32)) as u16;
        self.value = new_value;
    }

    pub fn next(&mut self) -> u16 {
        // The program counter should always point to the next instruction
        let old_value = self.value;
        self.value = self.value.wrapping_add(1);

        old_value
    }
}
