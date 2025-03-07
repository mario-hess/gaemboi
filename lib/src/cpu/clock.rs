pub const CPU_CLOCK_SPEED: u32 = 4194304;
const FPS: f32 = 59.7275;

// CPU_CLOCK_SPEED / FPS = ~70224.00066970826
// Cycles are represented in integers, truncating
// the floating point values is correct
pub const CYCLES_PER_FRAME: u32 = (CPU_CLOCK_SPEED as f32 / FPS) as u32; 

pub struct Clock {
    pub cycles_passed: u32,
}

impl Clock {
    pub fn new() -> Self {
        Self { cycles_passed: 0 }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        let t_cycles = m_cycles * 4;
        self.cycles_passed += (t_cycles) as u32;
    }

    pub fn reset(&mut self) {
        self.cycles_passed = 0;
    }
}
