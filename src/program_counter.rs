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
        self.value += value;
    }
}
