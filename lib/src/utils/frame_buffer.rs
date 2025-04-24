pub enum FrameBuffer {
    U8(Box<[u8]>),
    U16(Box<[u16]>),
}

impl FrameBuffer {
    pub fn set_pixel_u8(
        &mut self,
        offset: usize,
        pixel: u8,
    ) -> Result<(), &'static str> {
        match self {
            FrameBuffer::U8(buffer) => {
                buffer[offset] = pixel;
                Ok(())
            }
            _ => Err("Trying to set u8 pixel on a non-u8 buffer"),
        }
    }

    pub fn set_pixel_u16(&mut self, offset: usize, pixel: u16) -> Result<(), &'static str> {
        match self {
            FrameBuffer::U16(buffer) => {
                buffer[offset] = pixel;
                Ok(())
            }
            _ => Err("Trying to set u16 pixel on a non-u16 buffer"),
        }
    }
}
