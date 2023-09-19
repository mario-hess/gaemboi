const TILEMAP_ADDRESS_1: u16 = 0x9800;
const TILEMAP_ADDRESS_2: u16 = 0x9C00;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct LCD_control {
    bg_window_enable: bool,
    object_enable: bool,
    object_size: bool,
    bg_tilemap: bool,
    addressing_mode: bool,
    window_enable: bool,
    window_tilemap: bool,
    lcd_enable: bool,
}

impl LCD_control {
    pub fn new() -> Self {
        Self {
            bg_window_enable: true,
            object_enable: false,
            object_size: false,
            bg_tilemap: false,
            addressing_mode: true,
            window_enable: false,
            window_tilemap: false,
            lcd_enable: true,
        }
    }

    pub fn get_lcd_control(self) -> u8 {
        let bg_window_enable: u8 = if self.bg_window_enable { 0x01 } else { 0 };
        let object_enable: u8 = if self.bg_window_enable { 0x02 } else { 0 };
        let object_size: u8 = if self.bg_window_enable { 0x04 } else { 0 };
        let bg_tilemap: u8 = if self.bg_window_enable { 0x08 } else { 0 };
        let addressing_mode: u8 = if self.bg_window_enable { 0x010 } else { 0 };
        let window_enable: u8 = if self.bg_window_enable { 0x20 } else { 0 };
        let window_tilemap: u8 = if self.bg_window_enable { 0x40 } else { 0 };
        let lcd_enable: u8 = if self.bg_window_enable { 0x80 } else { 0 };

        bg_window_enable
            | object_enable
            | object_size
            | bg_tilemap
            | addressing_mode
            | window_enable
            | window_tilemap
            | lcd_enable
    }

    pub fn set_lcd_control(&mut self, value: u8) {
        self.bg_window_enable = value & 0x01 == 0x01;
        self.object_enable = value & 0x02 == 0x02;
        self.object_size = value & 0x04 == 0x04;
        self.bg_tilemap = value & 0x08 == 0x08;
        self.addressing_mode = value & 0x10 == 0x10;
        self.window_enable = value & 0x20 == 0x20;
        self.window_tilemap = value & 0x40 == 0x40;
        self.lcd_enable = value & 0x80 == 0x80;
    }

    // LCDC.3 — BG tile map area
    // This bit works similarly to LCDC bit 6: if the bit is clear (0),
    // the BG uses tilemap $9800, otherwise tilemap $9C00.
    pub fn bg_tilemap_address(self) -> u16 {
        if !self.bg_tilemap {
            TILEMAP_ADDRESS_1
        } else {
            TILEMAP_ADDRESS_2
        }
    }

    // LCDC.6 — Window tile map area
    // This bit controls which background map the Window uses for rendering.
    // When it’s clear (0), the $9800 tilemap is used, otherwise it’s the $9C00 one.
    pub fn window_tilemap_address(self) -> u16 {
        if !self.window_tilemap {
            TILEMAP_ADDRESS_1
        } else {
            TILEMAP_ADDRESS_2
        }
    }
}
