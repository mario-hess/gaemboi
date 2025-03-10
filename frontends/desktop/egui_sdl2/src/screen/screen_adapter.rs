use gaemboi::FrameBufferListener;

pub struct ScreenAdapter;
impl ScreenAdapter {
    pub fn new() -> Self {
        Self {}
    }

    fn render_u8(&mut self, _frame_buffer: &[u8]) {
        //println!("{:?}", frame_buffer);
    }

    fn render_u16(&mut self, _frame_buffer: &[u16]) {
        //println!("{:?}", frame_buffer);
    }
}

impl FrameBufferListener<u8> for ScreenAdapter {
    fn on_frame_ready(&mut self, frame_buffer: &[u8]) {
        self.render_u8(frame_buffer);
    }
}

impl FrameBufferListener<u16> for ScreenAdapter {
    fn on_frame_ready(&mut self, frame_buffer: &[u16]) {
        self.render_u16(frame_buffer);
    }
}
