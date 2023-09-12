use crate::machine::FPS;

// 4.194304 MHz
pub const CLOCK_SPEED: u32 = 4194304;

pub struct Clock {
    pub cycles_per_frame: u32,
    pub cycles_passed: u32,
}

impl Clock {
    pub fn new() -> Self {
        let cycles_per_frame = CLOCK_SPEED / FPS as u32;

        Self {
            cycles_per_frame,
            cycles_passed: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        self.cycles_passed += (m_cycles * 4) as u32;
    }

    pub fn reset(&mut self) {
        self.cycles_passed -= self.cycles_per_frame;
    }
}
