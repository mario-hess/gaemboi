use std::time::Instant;

pub struct FpsCounter {
    frame_times: Vec<f32>,
    frame_count: u16,
    last_second: Instant,
    fps: f32,
}
impl FpsCounter {
    pub fn new() -> Self {
        let frame_times = Vec::new();
        let frame_count = 0;
        let last_second = std::time::Instant::now();
        let fps = 0.0;

        Self {
            frame_times,
            frame_count,
            last_second,
            fps,
        }
    }

    pub fn show(&mut self, &frame_start_time: &Instant) {
        let frame_time = frame_start_time.elapsed().as_secs_f32();
        self.frame_times.push(frame_time);
        self.frame_count += 1;

        if self.last_second.elapsed().as_secs() >= 1 {
            self.fps = self.frame_count as f32 / self.frame_times.iter().sum::<f32>();
            println!("{:.2}", self.fps);
            self.frame_times.clear();
            self.frame_count = 0;
            self.last_second = std::time::Instant::now();
        }
    }
}
