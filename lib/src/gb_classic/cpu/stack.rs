use crate::gb_classic::bus::Bus;

const STACK_POINTER_START: u16 = 0xFFFE;

pub struct Stack {
    pointer: u16,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            pointer: STACK_POINTER_START,
        }
    }

    pub fn get_pointer(&self) -> u16 {
        self.pointer
    }

    pub fn set_pointer(&mut self, value: u16) {
        self.pointer = value;
    }

    pub fn push(&mut self, bus: &mut Bus, address: u16) {
        let high_byte = (address >> 8) as u8;
        let low_byte = address as u8;

        self.set_pointer(self.get_pointer().wrapping_sub(1));
        bus.write_byte(self.get_pointer(), high_byte);

        self.set_pointer(self.get_pointer().wrapping_sub(1));
        bus.write_byte(self.get_pointer(), low_byte);
    }

    pub fn pop(&mut self, bus: &Bus) -> u16 {
        let low_byte = bus.read_byte(self.get_pointer()) as u16;
        self.set_pointer(self.get_pointer().wrapping_add(1));

        let high_byte = bus.read_byte(self.get_pointer()) as u16;
        self.set_pointer(self.get_pointer().wrapping_add(1));

        (high_byte << 8) | low_byte
    }
}
