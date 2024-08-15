pub struct FPSCounter {
    fps: f32,
    frame_times: Vec<f32>,
    frame_count: u32,
    last_second: std::time::Instant,
}
impl FPSCounter {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            frame_times: Vec::new(),
            frame_count: 0,
            last_second: std::time::Instant::now()
        }
    }

    pub fn tick(
        &mut self,
        frame_start_time: std::time::Instant,
    ) {
        let frame_time = frame_start_time.elapsed().as_secs_f32();
        self.frame_times.push(frame_time);
        self.frame_count += 1;

        if self.last_second.elapsed().as_secs() >= 1 {
            self.fps = self.frame_count as f32 / self.frame_times.iter().sum::<f32>();
            self.frame_times.clear();
            self.frame_count = 0;
            self.last_second = std::time::Instant::now();
            println!("{}", self.fps);
        }
    }
}
