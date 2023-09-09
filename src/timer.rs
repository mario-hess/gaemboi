use crate::machine::FPS;

pub const CLOCK_SPEED: usize = 4194304;

pub struct Timer {
    pub cycles_per_frame: usize,
    pub cycles_passed: usize,
}

impl Timer {
    pub fn new() -> Timer {
        let cycles_per_frame = CLOCK_SPEED / FPS as usize;

        Timer {
            cycles_per_frame,
            cycles_passed: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        self.cycles_passed += (m_cycles * 4) as usize;
    }

    pub fn reset(&mut self) {
        self.cycles_passed -= self.cycles_per_frame;
    }
}
