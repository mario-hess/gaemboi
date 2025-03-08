use gaemboi::FrameBufferObserver;

pub struct Screen;
impl Screen {
    pub fn new() -> Self {
        Self {}
    }
    fn update(&mut self, frame_buffer: &[u8]) {
        //println!("{:?}", frame_buffer);
    }
    fn render(&self) {}
}
impl FrameBufferObserver for Screen {
    fn on_frame_ready(&mut self, frame_buffer: &[u8]) {
        self.update(frame_buffer);
        self.render();
    }
}

