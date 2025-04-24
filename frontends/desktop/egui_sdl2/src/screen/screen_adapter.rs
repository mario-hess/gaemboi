use std::{cell::RefCell, rc::Rc};

use egui_sdl2_gl::{
    egui::{Color32, Image, TextureId, Vec2, load::SizedTexture},
    painter::Painter,
};

use gaemboi::{
    ADVANCE_SCREEN_HEIGHT, ADVANCE_SCREEN_WIDTH, CLASSIC_SCREEN_HEIGHT, CLASSIC_SCREEN_WIDTH,
    FrameBuffer, FrameBufferObserver, GameBoyType,
};

const CLASSIC_PALETTE: [Color32; 4] = [
    Color32::WHITE,                   // 0 = White
    Color32::from_rgb(170, 170, 170), // 1 = Light
    Color32::from_rgb(85, 85, 85),    // 2 = Dark
    Color32::BLACK,                   // 3 = Black
];

pub struct ScreenAdapter {
    painter: Rc<RefCell<Painter>>,
    texture_id: TextureId,
    width: usize,
    height: usize,
    gb_type: GameBoyType,
}

impl ScreenAdapter {
    pub fn new(painter: Rc<RefCell<Painter>>, gb_type: &GameBoyType) -> Self {
        let gb_type = gb_type.clone();

        let (width, height) = match gb_type {
            GameBoyType::Classic | GameBoyType::Color => {
                (CLASSIC_SCREEN_WIDTH, CLASSIC_SCREEN_HEIGHT)
            }
            GameBoyType::Advance => (ADVANCE_SCREEN_WIDTH, ADVANCE_SCREEN_HEIGHT),
        };

        let texture_id = painter.borrow_mut().new_user_texture(
            (width, height),
            &vec![Color32::from_rgb(0, 0, 0); width * height],
            false,
        );

        Self {
            painter,
            texture_id,
            width,
            height,
            gb_type,
        }
    }

    fn render_u8(&mut self, frame_buffer: &[u8]) {
        let buffer = match self.gb_type {
            GameBoyType::Classic => convert_u8_classic(frame_buffer),
            GameBoyType::Color => todo!("convert_u8_color not implemented"),
            _ => unreachable!(),
        };

        self.painter
            .borrow_mut()
            .update_user_texture_data(self.texture_id, &buffer);
    }

    fn render_u16(&mut self, frame_buffer: &[u16]) {
        todo!("render_u16 not implemented")
    }

    pub fn get_image(&self) -> Image {
        Image::new(SizedTexture::new(
            self.texture_id,
            Vec2::new(self.width as f32, self.height as f32),
        ))
    }
}

pub struct FrameBufferObserverWrapper {
    inner: Rc<RefCell<ScreenAdapter>>,
}

impl FrameBufferObserverWrapper {
    pub fn new(inner: Rc<RefCell<ScreenAdapter>>) -> Self {
        Self { inner }
    }
}

impl FrameBufferObserver for FrameBufferObserverWrapper {
    fn on_frame_ready(&mut self, frame_buffer: &FrameBuffer) {
        match frame_buffer {
            FrameBuffer::U8(buffer) => self.inner.borrow_mut().render_u8(buffer),
            FrameBuffer::U16(buffer) => self.inner.borrow_mut().render_u16(buffer),
        }
    }
}

fn classic_palette(index: u8) -> Color32 {
    CLASSIC_PALETTE[index as usize]
}

fn convert_u8_classic(pixels: &[u8]) -> Vec<Color32> {
    pixels.iter().map(|&p| classic_palette(p)).collect()
}
