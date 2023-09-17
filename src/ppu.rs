use crate::memory_bus::{OAM_END, OAM_START, VRAM_END, VRAM_START};

pub const VRAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 160;

const LCD_CONTROL: u16 = 0xFF40;
const LCD_STATUS: u16 = 0xFF41;
const SCROLL_Y: u16 = 0xFF42;
const SCROLL_X: u16 = 0xFF43;
const LINE_Y: u16 = 0xFF44;
const LINE_Y_COMPARE: u16 = 0xFF45;
const DMA: u16 = 0xFF46;
const BACKGROUND_PALETTE: u16 = 0xFF47;
const OBJECT_PALETTE_0: u16 = 0xFF48;
const OBJECT_PALETTE_1: u16 = 0xFF49;
const WINDOW_Y: u16 = 0xFF4A;
const WINDOW_X: u16 = 0xFF4B;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCALE: u8 = 4;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAM = 2,
    VRam = 3,
}

pub struct Ppu {
    tile_set: [[u8; 8]; 384],
    video_ram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    lcd_control: u8,
    lcd_status: u8,
    scroll_y: u8,
    scroll_x: u8,
    line_y: u8,
    line_y_compare: u8,
    dma: u8,
    background_palette: u8,
    object_palette_0: u8,
    object_palette_1: u8,
    window_y: u8,
    window_x: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            tile_set: [[0; 8]; 384],
            video_ram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcd_control: 0,
            lcd_status: 0,
            scroll_y: 0,
            scroll_x: 0,
            line_y: 0,
            line_y_compare: 0,
            dma: 0,
            background_palette: 0,
            object_palette_0: 0,
            object_palette_1: 0,
            window_y: 0,
            window_x: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize],
            OAM_START..=OAM_END => self.oam[(address - OAM_START) as usize],
            LCD_CONTROL => self.lcd_control,
            LCD_STATUS => self.lcd_status,
            SCROLL_Y => self.scroll_y,
            SCROLL_X => self.scroll_x,
            LINE_Y => self.line_y,
            LINE_Y_COMPARE => self.line_y_compare,
            DMA => self.dma,
            BACKGROUND_PALETTE => self.background_palette,
            OBJECT_PALETTE_0 => self.object_palette_0,
            OBJECT_PALETTE_1 => self.object_palette_1,
            WINDOW_Y => self.window_y,
            WINDOW_X => self.window_x,
            _ => panic!("Unknown address: {:#X} Can't read byte.", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize] = value,
            OAM_START..=OAM_END => self.oam[(address - OAM_START) as usize] = value,
            LCD_CONTROL => self.lcd_control = value,
            LCD_STATUS => self.lcd_status = value,
            SCROLL_Y => self.scroll_y = value,
            SCROLL_X => self.scroll_x = value,
            LINE_Y => self.line_y = value,
            LINE_Y_COMPARE => self.line_y_compare = value,
            DMA => self.dma = value,
            BACKGROUND_PALETTE => self.background_palette = value,
            OBJECT_PALETTE_0 => self.object_palette_0 = value,
            OBJECT_PALETTE_1 => self.object_palette_1 = value,
            WINDOW_Y => self.window_y = value,
            WINDOW_X => self.window_x = value,
            _ => panic!("Unknown address: {:#X} Can't write byte.", address),
        }
    }
}
