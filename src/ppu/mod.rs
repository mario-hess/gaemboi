/**
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    May 24, 2024
 */
mod lcd_control;
mod lcd_status;
mod oam;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    interrupt::{LCD_STAT_MASK, VBLANK_MASK},
    memory_bus::{OAM_END, OAM_START, VRAM_END, VRAM_START},
    ppu::{lcd_control::LCD_control, lcd_status::LCD_status, oam::OAM},
};

pub const VRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 40;

const TILE_DATA_START: u16 = VRAM_START;
pub const TILEMAP_START_0: u16 = 0x9800;
pub const TILEMAP_START_1: u16 = 0x9C00;

const LCD_CONTROL: u16 = 0xFF40;
const LCD_STATUS: u16 = 0xFF41;
const SCROLL_Y: u16 = 0xFF42;
const SCROLL_X: u16 = 0xFF43;
const LINE_Y: u16 = 0xFF44;
const LINE_Y_COMPARE: u16 = 0xFF45;
const BG_PALETTE: u16 = 0xFF47;
const TILE_PALETTE_0: u16 = 0xFF48;
const TILE_PALETTE_1: u16 = 0xFF49;
const WINDOW_Y: u16 = 0xFF4A;
const WINDOW_X: u16 = 0xFF4B;

pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const DARK: Color = Color::RGB(96, 96, 96);
pub const LIGHT: Color = Color::RGB(192, 192, 192);
pub const WHITE: Color = Color::RGB(255, 255, 255);

const CYCLES_OAM: u16 = 80;
const CYCLES_TRANSFER: u16 = 172;
const CYCLES_HBLANK: u16 = 204;
const CYCLES_VBLANK: u16 = 456;

const LINES_Y: u8 = 143;
const MAX_LINES_Y: u8 = 153;

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = TILE_WIDTH;

pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;

const FULL_WIDTH: usize = 256;

const OVERLAP_MAP_SIZE: usize = FULL_WIDTH * FULL_WIDTH;
pub const BUFFER_SIZE: usize = VIEWPORT_WIDTH * VIEWPORT_HEIGHT;

// https://gbdev.io/pandocs/Graphics.html
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Mode {
    OAM = 2,
    Transfer = 3,
    HBlank = 0,
    VBlank = 1,
}

pub struct Ppu {
    enabled: bool,
    pub interrupts: u8,
    video_ram: [u8; VRAM_SIZE],
    oam: [OAM; OAM_SIZE],
    lcd_control: LCD_control,
    lcd_status: LCD_status,
    scroll_x: u8,
    scroll_y: u8,
    window_x: u8,
    window_y: u8,
    line_y: u8,
    line_y_compare: u8,
    window_line_counter: u8,
    bg_palette: [u8; 4],
    sprite_palette0: [u8; 4],
    sprite_palette1: [u8; 4],
    counter: u16,
    overlap_map: [bool; OVERLAP_MAP_SIZE],
    pub screen_buffer: [Color; BUFFER_SIZE],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            enabled: true,
            interrupts: 0,
            video_ram: [0; VRAM_SIZE],
            oam: [OAM::new(); OAM_SIZE],
            lcd_control: LCD_control::new(),
            lcd_status: LCD_status::new(),
            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,
            line_y: 0,
            line_y_compare: 0,
            window_line_counter: 0,
            bg_palette: [0, 1, 2, 3],
            sprite_palette0: [0, 1, 2, 3],
            sprite_palette1: [0, 1, 2, 3],
            counter: 0,
            overlap_map: [false; OVERLAP_MAP_SIZE],
            screen_buffer: [WHITE; BUFFER_SIZE],
        }
    }

    pub fn tick(&mut self, m_cycles: u8, canvas: &mut Canvas<Window>) {
        if !self.lcd_control.lcd_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;
        self.counter += t_cycles;

        // https://gbdev.io/pandocs/Rendering.html
        match self.lcd_status.mode {
            Mode::OAM => {
                if self.counter < CYCLES_OAM {
                    return;
                }

                self.lcd_status
                    .set_mode(Mode::Transfer, &mut self.interrupts);

                self.counter -= CYCLES_OAM;
            }
            Mode::Transfer => {
                if self.counter < CYCLES_TRANSFER {
                    return;
                }

                self.render_scanline();
                self.lcd_status.set_mode(Mode::HBlank, &mut self.interrupts);

                self.counter -= CYCLES_TRANSFER;
            }
            Mode::HBlank => {
                if self.counter < CYCLES_HBLANK {
                    return;
                }

                if self.line_y >= LINES_Y {
                    self.lcd_status.set_mode(Mode::VBlank, &mut self.interrupts);
                    self.draw_viewport(canvas);
                    self.interrupts |= VBLANK_MASK;
                    self.clear_screen();
                } else {
                    if self.lcd_control.window_enabled
                        && self.window_x - 7 < VIEWPORT_WIDTH as u8
                        && self.window_y < VIEWPORT_HEIGHT as u8
                        && self.line_y >= self.window_y
                    {
                        self.window_line_counter += 1;
                    }
                    self.set_line_y(self.line_y + 1);
                    self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                }

                self.counter -= CYCLES_HBLANK;
            }
            Mode::VBlank => {
                if self.counter < CYCLES_VBLANK {
                    return;
                }

                self.set_line_y(self.line_y + 1);

                if self.line_y > MAX_LINES_Y {
                    self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                    self.window_line_counter = 0;
                    self.set_line_y(0);
                }

                self.counter -= CYCLES_VBLANK;
            }
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize],
            OAM_START..=OAM_END => self.read_oam(address - OAM_START),
            LCD_CONTROL => self.lcd_control.get(),
            LCD_STATUS => self.lcd_status.get(),
            SCROLL_Y => self.scroll_y,
            SCROLL_X => self.scroll_x,
            LINE_Y => self.line_y,
            LINE_Y_COMPARE => self.line_y_compare,
            BG_PALETTE => get_palette(&self.bg_palette),
            TILE_PALETTE_0 => get_palette(&self.sprite_palette0),
            TILE_PALETTE_1 => get_palette(&self.sprite_palette1),
            WINDOW_Y => self.window_y,
            WINDOW_X => self.window_x,
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize] = value,
            OAM_START..=OAM_END => self.write_oam(address - OAM_START, value),
            LCD_CONTROL => self.set_lcd_control(value),
            LCD_STATUS => self.lcd_status.set(value),
            SCROLL_Y => self.scroll_y = value,
            SCROLL_X => self.scroll_x = value,
            LINE_Y => {} // Not used
            LINE_Y_COMPARE => self.set_line_y_compare(value),
            BG_PALETTE => set_palette(&mut self.bg_palette, value),
            TILE_PALETTE_0 => set_palette(&mut self.sprite_palette0, value),
            TILE_PALETTE_1 => set_palette(&mut self.sprite_palette1, value),
            WINDOW_Y => self.window_y = value,
            WINDOW_X => {
                if value < 7 {
                    return;
                }
                self.window_x = value
            }
            _ => eprintln!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn read_oam(&self, address: u16) -> u8 {
        let index = (address / 4) as usize;
        let offset = (address % 4) as usize;

        match offset {
            0 => self.oam[index].y_pos,
            1 => self.oam[index].x_pos,
            2 => self.oam[index].tile_index,
            3 => self.oam[index].attributes,
            _ => unreachable!(),
        }
    }

    fn write_oam(&mut self, address: u16, value: u8) {
        let index = (address / 4) as usize;
        let offset = (address % 4) as usize;

        match offset {
            0 => self.oam[index].y_pos = value,
            1 => self.oam[index].x_pos = value,
            2 => self.oam[index].tile_index = value,
            3 => self.oam[index].attributes = value,
            _ => unreachable!(),
        }
    }

    fn set_line_y(&mut self, value: u8) {
        self.line_y = value;
        self.compare_line();
    }

    pub fn set_line_y_compare(&mut self, value: u8) {
        self.line_y_compare = value;
        self.compare_line();
    }

    fn compare_line(&mut self) {
        self.lcd_status.compare_flag = false;

        if self.line_y_compare == self.line_y {
            self.lcd_status.compare_flag = true;

            if self.lcd_status.interrupt_stat {
                self.interrupts |= LCD_STAT_MASK;
            }
        }
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control.set(value);

        if !self.lcd_control.lcd_enabled {
            self.clear_screen();
            self.window_line_counter = 0;
            self.set_line_y(0);
            self.lcd_status.mode = Mode::HBlank;
            self.counter = 0;
            self.enabled = false;
        }
    }

    fn render_scanline(&mut self) {
        if self.lcd_control.bg_enabled {
            self.render_bg_line();
        }

        if self.lcd_control.object_enabled {
            self.render_object_line();
        }
    }

    fn render_bg_line(&mut self) {
        let bg_offset_y = self.line_y.wrapping_add(self.scroll_y);
        let row_is_window = self.lcd_control.window_enabled && self.line_y >= self.window_y;

        for x in 0..VIEWPORT_WIDTH as u8 {
            let bg_offset_x = x.wrapping_add(self.scroll_x);
            let col_is_window =
                self.lcd_control.window_enabled && x >= self.window_x.wrapping_sub(7);
            let is_window = row_is_window && col_is_window;

            // Determine the sprite data based on whether it's in the window or background
            let (sprite_index_address, line_offset, pixel_index) =
                self.get_bg_tile_data(is_window, x, bg_offset_x, bg_offset_y);

            let sprite_index = self.read_byte(sprite_index_address);
            let sprite_address = self.lcd_control.get_address(sprite_index);

            let (first_byte, second_byte) =
                self.get_tile_bytes(sprite_address + line_offset as u16);

            let color_index = get_color_index(first_byte, second_byte, pixel_index);

            // Calculate the offset for the current pixel based on
            // the background width and update the overlap map
            let overlap_offset = self.line_y as usize + FULL_WIDTH * x as usize;
            if color_index == 0 {
                self.overlap_map[overlap_offset] = true;
            }

            let pixel = get_pixel_color(&self.bg_palette, color_index);

            // Calculate the offset for the current pixel and update the screen buffer
            let offset = x as usize + self.line_y as usize * VIEWPORT_WIDTH;
            self.screen_buffer[offset] = pixel;
        }
    }

    fn render_object_line(&mut self) {
        // Convert line_y to an i16 and determine the height of the sprite (8x8 or 8x16)
        let line_y = self.line_y as i16;
        let tile_height = if self.lcd_control.object_size {
            TILE_HEIGHT as i16 * 2
        } else {
            TILE_HEIGHT as i16
        };

        let mut sorted_objects: Vec<(usize, i16)> = Vec::new();

        for i in 0..OAM_SIZE {
            let oam_entry = self.oam[i];
            // First byte in OAM (oam_entry.y_pos) is the object’s vertical position on the screen + 16
            let object_y = oam_entry.y_pos as i16 - 16;
            // Second byte in OAM (oam_entry.x_pos) is the object’s horizontal position on the screen + 8
            let object_x = oam_entry.x_pos as i16 - 8;

            // Determine if the current scanline intersects with the vertical span of the object
            if line_y >= object_y && line_y < object_y + tile_height {
                sorted_objects.push((i, object_x));
            }
        }

        // Stable sort sprites based on X coordinate and index
        sorted_objects.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        // 10 Objects per line limit, reversed for correct overlapping
        for (index, x_offset) in sorted_objects.iter().take(10).rev() {
            let oam_entry = self.oam[*index];
            let object_y = oam_entry.y_pos as i16 - 16;

            let mut object_index = oam_entry.tile_index;

            // Ignore last bit for 8x16 sprites
            if tile_height == TILE_HEIGHT as i16 * 2 {
                object_index &= 0b1111_1110;
            }

            // An object sprite consists of 16 bytes
            let tile_begin_address = TILE_DATA_START + (object_index as u16 * 16);

            // Calculate line offset based on if the sprite is vertically mirrored
            let line_offset = if oam_entry.y_flip_enabled() {
                tile_height - 1 - (line_y - object_y)
            } else {
                line_y - object_y
            };

            // Since each line consists of 2 bytes, the offset has to be multiplied by 2
            let tile_address = tile_begin_address + (line_offset * 2) as u16;
            let (first_byte, second_byte) = self.get_tile_bytes(tile_address);

            for x in 0..8 {
                let x_offset = *x_offset + x as i16;

                // Skip pixels outside of viewport
                if !(0..VIEWPORT_WIDTH as i16).contains(&x_offset) {
                    continue;
                }

                // Skip rendering pixel if background overlaps
                let overlap_offset = line_y as usize + FULL_WIDTH * x_offset as usize;
                if self.bg_has_priority(&oam_entry, overlap_offset) {
                    continue;
                }

                // Check if pixel is horizontally mirrored
                let pixel_index = if oam_entry.x_flip_enabled() { x } else { 7 - x };

                let color_index = get_color_index(first_byte, second_byte, pixel_index);

                // Skip rendering transparent pixels
                if color_index == 0 {
                    continue;
                }

                let sprite_palette = if oam_entry.palette_enabled() {
                    self.sprite_palette1
                } else {
                    self.sprite_palette0
                };

                let pixel = get_pixel_color(&sprite_palette, color_index);

                // Calculate the offset for the current pixel and update the screen buffer
                let offset = x_offset + line_y * VIEWPORT_WIDTH as i16;
                self.screen_buffer[offset as usize] = pixel;
            }
        }
    }

    fn get_bg_tile_data(
        &self,
        is_window: bool,
        x: u8,
        bg_offset_x: u8,
        bg_offset_y: u8,
    ) -> (u16, u8, u8) {
        let address = if is_window {
            let address = self.lcd_control.get_window_address();
            let x_offset = x.wrapping_sub(self.window_x.wrapping_sub(7));
            let y_offset = self.window_line_counter;

            calculate_address(address, x_offset, y_offset)
        } else {
            let address = self.lcd_control.get_bg_address();

            calculate_address(address, bg_offset_x, bg_offset_y)
        };

        // Since each line consists of 2 bytes, the offset has to be multiplied by 2
        let line_offset = if is_window {
            (self.line_y - self.window_y) % TILE_HEIGHT as u8 * 2
        } else {
            bg_offset_y % TILE_HEIGHT as u8 * 2
        };

        let pixel_index = if is_window {
            self.window_x.wrapping_sub(x) % TILE_WIDTH as u8
        } else {
            7 - (bg_offset_x % TILE_WIDTH as u8)
        };

        (address, line_offset, pixel_index)
    }

    fn get_tile_bytes(&self, address: u16) -> (u8, u8) {
        let first_byte = self.read_byte(address);
        let second_byte = self.read_byte(address + 1);

        (first_byte, second_byte)
    }

    fn bg_has_priority(&self, oam_entry: &OAM, offset: usize) -> bool {
        if !oam_entry.overlap_enabled() {
            return false;
        }

        !self.overlap_map[offset]
    }

    pub fn draw_viewport(&mut self, canvas: &mut Canvas<Window>) {
        for (index, pixel) in self.screen_buffer.iter().enumerate() {
            let x_coord = (index % VIEWPORT_WIDTH) as i32;
            let y_coord = (index / VIEWPORT_WIDTH) as i32;

            canvas.set_draw_color(*pixel);
            canvas.draw_point(Point::new(x_coord, y_coord)).unwrap();
        }
    }

    fn clear_screen(&mut self) {
        for i in 0..OVERLAP_MAP_SIZE {
            if i < BUFFER_SIZE {
                self.screen_buffer[i] = WHITE;
            }
            self.overlap_map[i] = false;
        }
    }

    pub fn reset_interrupts(&mut self) {
        self.interrupts = 0;
    }
}

fn get_pixel_color(palette: &[u8], color_index: u8) -> Color {
    match palette[color_index as usize] {
        0 => WHITE,
        1 => LIGHT,
        2 => DARK,
        3 => BLACK,
        _ => BLACK,
    }
}

fn calculate_address(base_address: u16, x: u8, y: u8) -> u16 {
    let sprites_per_row: u16 = (FULL_WIDTH / TILE_WIDTH) as u16;
    let sprite_x = (x as u16) / TILE_WIDTH as u16;
    let sprite_y = (y as u16) / TILE_HEIGHT as u16;

    let offset = sprite_y * sprites_per_row + sprite_x;

    base_address + offset
}

fn get_color_index(first_byte: u8, second_byte: u8, pixel_index: u8) -> u8 {
    // The first byte specifies the least significant bit of the color ID of
    // each pixel, and the second byte specifies the most significant bit
    ((first_byte >> pixel_index) & 1) | ((second_byte >> pixel_index) & 1) << 1
}

fn set_palette(palette: &mut [u8], value: u8) {
    for (i, color_data) in (0..4).map(|i| (i, (value >> (i * 2) & 0x03))) {
        palette[i] = color_data;
    }
}

fn get_palette(palette: &[u8]) -> u8 {
    palette.iter().enumerate().fold(0u8, |acc, (i, &color)| {
        let color_data = match color {
            0..=3 => color,
            _ => 3,
        };

        acc | (color_data & 0x03) << (i * 2)
    })
}
