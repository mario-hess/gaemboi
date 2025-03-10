use gaemboi::{FrameBuffer, FrameBufferListener};

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

impl FrameBufferListener for ScreenAdapter {
    fn on_frame_ready(&mut self, frame_buffer: &FrameBuffer) {
        match frame_buffer {
            FrameBuffer::U8(buffer) => self.render_u8(buffer),
            FrameBuffer::U16(buffer) => self.render_u16(buffer),
        }
    }
}
