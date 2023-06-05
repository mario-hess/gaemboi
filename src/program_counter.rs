pub struct ProgramCounter {
    pub value: u16,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: 0x0100 }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn increment(&mut self, value: u16) {
        self.value = self.value.wrapping_add(value);
    }

    pub fn increment_signed(&mut self, value: i8) {
        self.value = ((self.value as i32).wrapping_add(value as i32)) as u16;
    }

    pub fn next(&mut self) -> u16 {
        let old = self.value;
        self.value = self.value.wrapping_add(1);
        old
    }
    
    pub fn step(&mut self) {
        self.value += 1;
    }
}
