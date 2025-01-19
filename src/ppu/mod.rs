/*
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    June 19, 2024
 */

mod background;
mod lcd_control;
mod lcd_status;
mod oam;
mod tile;
mod window;

use egui_sdl2_gl::egui::Color32;

use crate::{
    event_handler::EventHandler,
    interrupt::{LCD_STAT_MASK, VBLANK_MASK},
    memory_bus::{MemoryAccess, OAM_END, OAM_START, VRAM_END, VRAM_START},
    ppu::{
        background::Background,
        lcd_control::LCD_control,
        lcd_status::{LCD_status, MODE_HBLANK, MODE_OAM, MODE_TRANSFER, MODE_VBLANK},
        oam::OAM,
        tile::Tile,
        window::Window,
    },
};

pub const VRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 40;

const TILETABLE_DATA_START: u16 = VRAM_START;
const TILETABLE_DATA_END: u16 = 0x97FF;

pub const TILEMAP_START_0: u16 = 0x9800;
pub const TILEMAP_END_0: u16 = 0x9BFF;
pub const TILEMAP_START_1: u16 = 0x9C00;
pub const TILEMAP_END_1: u16 = VRAM_END;

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

const CYCLES_OAM: u16 = 80;
const CYCLES_TRANSFER: u16 = 172;
const CYCLES_HBLANK: u16 = 204;
const CYCLES_VBLANK: u16 = 456;

const LINES_Y: u8 = 143;
const MAX_LINES_Y: u8 = 153;

const TILE_WIDTH: u8 = 8;
const TILE_HEIGHT: u8 = TILE_WIDTH;
const TILE_HEIGHT_BIG: u8 = TILE_HEIGHT * 2;

pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;

pub const TILETABLE_WIDTH: usize = 128 + 17;
pub const TILETABLE_HEIGHT: usize = 192 + 26;
pub const TILEMAP_WIDTH: usize = 256;
pub const TILEMAP_HEIGHT: usize = TILEMAP_WIDTH;

const GRID_COLOR: Color32 = Color32::from_rgb(128, 128, 128);

const FULL_WIDTH: usize = 256;

pub const OVERLAP_MAP_SIZE: usize = FULL_WIDTH * FULL_WIDTH;
pub const BUFFER_SIZE: usize = VIEWPORT_WIDTH * VIEWPORT_HEIGHT;

// https://gbdev.io/pandocs/Graphics.html
// https://hacktix.github.io/GBEDG/ppu/
pub struct Ppu {
    enabled: bool,
    pub interrupts: u8,
    video_ram: [u8; VRAM_SIZE],
    oam: [OAM; OAM_SIZE],
    oam_buffer: Vec<(usize, i16)>,
    lcd_control: LCD_control,
    lcd_status: LCD_status,
    window: Window,
    background: Background,
    // Indicates the current horizontal line, which might be about to be drawn, being
    // drawn, or just been drawn.
    scan_y: u8,
    // The system constantly compares the value of the LY and LYC registers. When both
    // values are identical, the compare flag in the STAT register is set, and (if enabled)
    // a STAT interrupt is requested.
    scan_y_compare: u8,
    // This register assigns gray shades to the color IDs of the Background and Window tiles.
    bg_palette: u8,
    // These registers assign gray shades to the color IDs of the Objects that use the
    // corresponding palette. Color index 0 is transparent for Objects.
    sprite_palette0: u8,
    sprite_palette1: u8,
    tile_height: u8,
    counter: u16,
    pub overlap_map: [bool; OVERLAP_MAP_SIZE],
    pub viewport_buffer: [Color32; BUFFER_SIZE],
    pub should_draw: bool,
}

impl MemoryAccess for Ppu {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize],
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.read_oam(address - OAM_START),
            // 0xFF40 (LCD Control)
            LCD_CONTROL => (&self.lcd_control).into(),
            // 0xFF41 (LCD Status)
            LCD_STATUS => (&self.lcd_status).into(),
            // 0xFF42 (Scroll Y)
            SCROLL_Y => self.background.get_y_scroll(),
            // 0xFF43 (Scroll X)
            SCROLL_X => self.background.get_x_scroll(),
            // 0xFF44 (Line Y Coordinate)
            LINE_Y => self.scan_y,
            // 0xFF45 (Line Y Compare)
            LINE_Y_COMPARE => self.scan_y_compare,
            // 0xFF47 (BG Palette)
            BG_PALETTE => self.bg_palette,
            // 0xFF48 (Object Palette 0)
            TILE_PALETTE_0 => self.sprite_palette0,
            // 0xFF49 (Object Palette 1)
            TILE_PALETTE_1 => self.sprite_palette1,
            // 0xFF4A (Window Y Position)
            WINDOW_Y => self.window.get_y_coord(),
            // 0xFF4B (Window X Position)
            WINDOW_X => self.window.get_x_coord(),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // 0x8000 - 0x9FFF (Video Ram)
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize] = value,
            // 0xFE00 - 0xFE9F (Object Attribute Memory)
            OAM_START..=OAM_END => self.write_oam(address - OAM_START, value),
            // 0xFF40 (LCD Control)
            LCD_CONTROL => self.set_lcd_control(value),
            // 0xFF41 (LCD Status)
            LCD_STATUS => self.lcd_status = value.into(),
            // 0xFF42 (Scroll Y)
            SCROLL_Y => self.background.set_y_scroll(value),
            // 0xFF43 (Scroll X)
            SCROLL_X => self.background.set_x_scroll(value),
            // 0xFF44 (Line Y Coordinate - Not used)
            LINE_Y => {}
            // 0xFF45 (Line Y Compare)
            LINE_Y_COMPARE => self.set_scan_y_compare(value),
            // 0xFF47 (BG Palette)
            BG_PALETTE => self.bg_palette = value,
            // 0xFF48 (Object Palette 0)
            TILE_PALETTE_0 => self.sprite_palette0 = value,
            // 0xFF49 (Object Palette 1)
            TILE_PALETTE_1 => self.sprite_palette1 = value,
            // 0xFF4A (Window Y Position)
            WINDOW_Y => self.window.set_y_coord(value),
            // 0xFF4B (Window X Position)
            WINDOW_X => self.window.set_x_coord(value),
            _ => unreachable!(),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            enabled: true,
            interrupts: 0,
            video_ram: [0; VRAM_SIZE],
            oam: [OAM::new(); OAM_SIZE],
            oam_buffer: Vec::new(),
            lcd_control: LCD_control::default(),
            lcd_status: LCD_status::default(),
            window: Window::new(),
            background: Background::new(),
            scan_y: 0,
            scan_y_compare: 0,
            bg_palette: 0,
            sprite_palette0: 0,
            sprite_palette1: 0,
            tile_height: TILE_HEIGHT,
            counter: 0,
            overlap_map: [false; OVERLAP_MAP_SIZE],
            viewport_buffer: [Color32::from_rgb(224, 248, 208); BUFFER_SIZE],
            should_draw: false,
        }
    }

    pub fn tick(&mut self, m_cycles: u8, event_handler: &EventHandler) {
        if !self.lcd_control.lcd_enabled() {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;
        self.counter += t_cycles;

        // https://gbdev.io/pandocs/Rendering.html
        match self.lcd_status.get_mode() {
            // During this mode the PPU searches OAM memory for sprites that should
            // be rendered on the current scanline and stores them in a buffer.
            MODE_OAM => {
                if self.counter < CYCLES_OAM {
                    return;
                }

                self.oam_buffer.clear();
                let scan_y = self.scan_y as i16;

                // Determine the height of the sprite (8x8 or 8x16)
                self.tile_height = if self.lcd_control.object_size() {
                    TILE_HEIGHT_BIG
                } else {
                    TILE_HEIGHT
                };

                for i in 0..OAM_SIZE {
                    let oam_entry = self.oam[i];
                    // First byte in OAM (oam_entry.y_pos) is the
                    // object’s vertical position on the screen + 16
                    let object_y = oam_entry.get_y_pos() as i16 - 16;
                    // Second byte in OAM (oam_entry.x_pos) is the
                    // object’s horizontal position on the screen + 8
                    let object_x = oam_entry.get_x_pos() as i16 - 8;

                    // Determine if the current scanline intersects with the vertical span of the object
                    if scan_y >= object_y && scan_y < object_y + self.tile_height as i16 {
                        self.oam_buffer.push((i, object_x));
                    }
                }

                // Stable sort sprites based on X coordinate and index
                self.oam_buffer
                    .sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

                // Take the first 10 items and reverse the order (10 Objects per line limit)
                self.oam_buffer.truncate(10);
                self.oam_buffer.reverse();

                self.lcd_status
                    .set_mode(MODE_TRANSFER, &mut self.interrupts);

                self.counter -= CYCLES_OAM;
            }
            // In this mode the PPU transfers pixels of the current scanline to the LCD.
            MODE_TRANSFER => {
                if self.counter < CYCLES_TRANSFER {
                    return;
                }

                self.render_scanline(event_handler);
                self.lcd_status.set_mode(MODE_HBLANK, &mut self.interrupts);

                self.counter -= CYCLES_TRANSFER;
            }
            // This mode takes up the remainder of the scanline after the Drawing Mode
            // finishes, more or less “padding” the duration of the scanline to a total
            // of 456 T-Cycles.
            MODE_HBLANK => {
                if self.counter < CYCLES_HBLANK {
                    return;
                }

                if self.scan_y >= LINES_Y {
                    // Draw the current frame to the screen
                    self.should_draw = true;
                    self.interrupts |= VBLANK_MASK;
                    self.lcd_status.set_mode(MODE_VBLANK, &mut self.interrupts);
                } else {
                    // Increase internal window line counter alongside scan_y if window is visible on
                    // the viewport
                    self.window
                        .increase_line_counter(self.lcd_control.window_enabled(), self.scan_y);

                    self.set_scan_y(self.scan_y + 1);
                    self.lcd_status.set_mode(MODE_OAM, &mut self.interrupts);
                }

                self.counter -= CYCLES_HBLANK;
            }
            // V-Blank mode is the same as H-Blank in the way that the PPU does not draw
            // any pixels to the LCD during its duration. However, instead of it taking
            // place at the end of every scanline, it’s a much longer period at the end
            // of every frame.
            MODE_VBLANK => {
                if self.counter < CYCLES_VBLANK {
                    return;
                }

                self.set_scan_y(self.scan_y + 1);

                // Next frame
                if self.scan_y > MAX_LINES_Y {
                    self.lcd_status.set_mode(MODE_OAM, &mut self.interrupts);
                    self.window.reset_line_counter();
                    self.set_scan_y(0);
                }

                self.counter -= CYCLES_VBLANK;
            }
            _ => unreachable!(),
        }
    }

    fn read_oam(&self, address: u16) -> u8 {
        let index = (address / 4) as usize;
        let offset = (address % 4) as usize;

        match offset {
            0 => self.oam[index].get_y_pos(),
            1 => self.oam[index].get_x_pos(),
            2 => self.oam[index].get_tile_index(),
            3 => self.oam[index].get_attributes(),
            _ => unreachable!(),
        }
    }

    fn write_oam(&mut self, address: u16, value: u8) {
        let index = (address / 4) as usize;
        let offset = (address % 4) as usize;

        match offset {
            0 => self.oam[index].set_y_pos(value),
            1 => self.oam[index].set_x_pos(value),
            2 => self.oam[index].set_tile_index(value),
            3 => self.oam[index].set_attributes(value),
            _ => unreachable!(),
        }
    }

    fn set_scan_y(&mut self, value: u8) {
        self.scan_y = value;
        self.compare_line();
    }

    pub fn set_scan_y_compare(&mut self, value: u8) {
        self.scan_y_compare = value;
        self.compare_line();
    }

    fn compare_line(&mut self) {
        self.lcd_status.set_compare_flag(false);

        if self.scan_y_compare != self.scan_y {
            return;
        }

        self.lcd_status.set_compare_flag(true);

        if self.lcd_status.get_interrupt_stat() {
            self.interrupts |= LCD_STAT_MASK;
        }
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control = value.into();

        if !self.lcd_control.lcd_enabled() {
            self.clear_screen();
            self.window.reset_line_counter();
            self.set_scan_y(0);
            self.lcd_status.mode = MODE_HBLANK;
            self.counter = 0;
            self.enabled = false;
        }
    }

    fn render_scanline(&mut self, event_handler: &EventHandler) {
        if self.lcd_control.bg_enabled() {
            self.render_bg_window_line(event_handler);
        }

        if self.lcd_control.object_enabled() {
            self.render_object_line(event_handler);
        }
    }

    fn render_bg_window_line(&mut self, event_handler: &EventHandler) {
        let base_offset = self.scan_y as usize * VIEWPORT_WIDTH;
        for scan_x in 0..VIEWPORT_WIDTH as u8 {
            let (tile_index_address, x_offset, y_offset) = self.get_bg_window_tile_data(scan_x);

            let tile_index = self.read_byte(tile_index_address);
            let tile_address = self.lcd_control.get_address(tile_index);

            let (first_byte, second_byte) = self.get_tile_bytes(tile_address + y_offset as u16);

            let color_index = color_index(first_byte, second_byte, x_offset);

            let overlap_offset = self.scan_y as usize + FULL_WIDTH * scan_x as usize;
            self.overlap_map[overlap_offset] = color_index != 0;

            let pixel = self.pixel_color(&self.bg_palette, &color_index, event_handler);

            // Calculate the offset for the current pixel and update the viewport buffer
            let offset = scan_x as usize + base_offset;
            self.viewport_buffer[offset] = pixel;
        }
    }

    fn render_object_line(&mut self, event_handler: &EventHandler) {
        let scan_y = self.scan_y as i16;
        let base_offset = scan_y as usize * VIEWPORT_WIDTH;

        for (oam_index, x_offset) in self.oam_buffer.iter() {
            let oam_entry = self.oam[*oam_index];
            let y_offset = oam_entry.get_y_pos() as i16 - 16;

            let mut tile_index = oam_entry.get_tile_index();

            // Ignore last bit for 8x16 sprites
            if self.tile_height == TILE_HEIGHT_BIG {
                tile_index &= 0b1111_1110;
            }

            // A tile consists of 16 bytes
            let tile_start_address = TILETABLE_DATA_START + (tile_index as u16 * 16);

            // Calculate line offset based on if the sprite is vertically mirrored
            let line_offset = if oam_entry.attributes.y_flip_enabled() {
                self.tile_height as i16 - 1 - (scan_y - y_offset)
            } else {
                self.scan_y as i16 - y_offset
            };

            // Since each line consists of 2 bytes, the offset has to be multiplied by 2
            let tile_address = tile_start_address + line_offset as u16 * 2;
            let (first_byte, second_byte) = self.get_tile_bytes(tile_address);

            for pixel_index in 0..TILE_WIDTH {
                let x_offset = x_offset + pixel_index as i16;

                // Skip rendering pixel outside of viewport
                if !(0..VIEWPORT_WIDTH).contains(&(x_offset as usize)) {
                    continue;
                }

                // Skip rendering pixel if background overlaps
                let overlap_offset = scan_y as usize + FULL_WIDTH * x_offset as usize;
                if self.is_overlapping(&oam_entry, overlap_offset) {
                    continue;
                }

                // Check if pixel is horizontally mirrored
                let pixel_index = if oam_entry.attributes.x_flip_enabled() {
                    pixel_index
                } else {
                    7 - pixel_index
                };

                let color_index = color_index(first_byte, second_byte, pixel_index);

                // Skip rendering transparent pixels
                if color_index == 0 {
                    continue;
                }

                let sprite_palette = if oam_entry.attributes.dmg_palette_enabled() {
                    self.sprite_palette1
                } else {
                    self.sprite_palette0
                };

                let pixel = self.pixel_color(&sprite_palette, &color_index, event_handler);

                // Calculate the offset for the current pixel and update the viewport buffer
                let offset = x_offset as usize + base_offset;
                self.viewport_buffer[offset] = pixel;
            }
        }
    }

    fn get_bg_window_tile_data(&self, scan_x: u8) -> (u16, u8, u8) {
        let in_window =
            self.window
                .is_pixel_in_window(self.lcd_control.window_enabled(), scan_x, self.scan_y);

        if in_window {
            let base_address = self.lcd_control.get_window_address();
            let (x_coord, y_coord) = self.window.tilemap_coordinates(scan_x);

            let tile_index_address = calculate_address(base_address, x_coord, y_coord);
            let (x_offset, y_offset) = self.window.pixel_offsets(scan_x, self.scan_y);

            (tile_index_address, x_offset, y_offset)
        } else {
            let base_address = self.lcd_control.get_bg_address();
            let (x_coord, y_coord) = self.background.tilemap_coordinates(scan_x, self.scan_y);

            let tile_index_address = calculate_address(base_address, x_coord, y_coord);
            let (x_offset, y_offset) = self.background.pixel_offsets(x_coord, y_coord);

            (tile_index_address, x_offset, y_offset)
        }
    }

    // Each tile occupies 16 bytes, where each line is represented by 2 bytes
    fn get_tile_bytes(&self, address: u16) -> (u8, u8) {
        let first_byte = self.read_byte(address);
        let second_byte = self.read_byte(address + 1);

        (first_byte, second_byte)
    }

    fn is_overlapping(&self, oam_entry: &OAM, offset: usize) -> bool {
        if !oam_entry.attributes.priority_enabled() {
            return false;
        }

        self.overlap_map[offset]
    }

    pub fn clear_screen(&mut self) {
        self.overlap_map.fill(false);
    }

    pub fn reset_interrupts(&mut self) {
        self.interrupts = 0;
    }

    fn pixel_color(&self, palette: &u8, color_index: &u8, event_handler: &EventHandler) -> Color32 {
        match (palette >> (color_index << 1)) & 0b11 {
            0b00 => event_handler.white,
            0b01 => event_handler.light,
            0b10 => event_handler.dark,
            0b11 => event_handler.black,
            _ => unreachable!(),
        }
    }

    pub fn tiletable(
        &self,
        event_handler: &EventHandler,
    ) -> [Color32; TILETABLE_WIDTH * TILETABLE_HEIGHT] {
        let mut tiletable_buffer = [event_handler.white; TILETABLE_WIDTH * TILETABLE_HEIGHT];

        // Grid vertical lines
        for i in 0..=16 {
            let line_pos = i * 9;
            for y in 0..TILETABLE_HEIGHT {
                let index = y * TILETABLE_WIDTH + line_pos;
                tiletable_buffer[index] = GRID_COLOR;
            }
        }

        // Grid horizontal lines
        for i in 0..=24 {
            let line_pos = i * 9;
            for x in 0..TILETABLE_WIDTH {
                let index = line_pos * TILETABLE_WIDTH + x;
                tiletable_buffer[index] = GRID_COLOR;
            }
        }

        let tiles = (TILETABLE_DATA_START..=TILETABLE_DATA_END)
            .map(|i| self.read_byte(i))
            .collect::<Vec<u8>>()
            .chunks_exact(16)
            .map(|chunk| Tile::new(chunk, event_handler.black, event_handler.dark, event_handler.light, event_handler.white))
            .collect::<Vec<Tile>>();

        let tiles_per_row = 16;

        for (index, tile) in tiles.iter().enumerate() {
            let x = (index % tiles_per_row) * 9 + 1;
            let y = (index / tiles_per_row) * 9 + 1;

            for (row_index, row_pixel) in tile.data.iter().enumerate() {
                let y_offset = y + row_index;
                for (col_index, color) in row_pixel.iter().enumerate() {
                    let x_offset = x + col_index;
                    let buffer_index = y_offset * TILETABLE_WIDTH + x_offset;
                    tiletable_buffer[buffer_index] = *color;
                }
            }
        }

        tiletable_buffer
    }

    pub fn tilemap(
        &self,
        start_address: u16,
        end_address: u16,
        event_handler: &EventHandler,
    ) -> [Color32; TILEMAP_WIDTH * TILEMAP_HEIGHT] {
        let mut tilemap_buffer = [event_handler.white; TILEMAP_WIDTH * TILEMAP_HEIGHT];

        let tiles = (start_address..=end_address)
            .map(|i| self.lcd_control.get_address(self.read_byte(i)))
            .flat_map(|address| (0..16).map(move |j| self.read_byte(address + j)))
            .collect::<Vec<u8>>()
            .chunks_exact(16)
            .map(|chunk| {
                Tile::new(
                    chunk,
                    event_handler.black,
                    event_handler.dark,
                    event_handler.light,
                    event_handler.white,
                )
            })
            .collect::<Vec<Tile>>();

        let tiles_per_row = TILEMAP_WIDTH / TILE_WIDTH as usize;

        for (index, tile) in tiles.iter().enumerate() {
            let x = (index % tiles_per_row) * TILE_WIDTH as usize;
            let y = (index / tiles_per_row) * TILE_WIDTH as usize;

            for (row_index, row_pixel) in tile.data.iter().enumerate() {
                let y_offset = y + row_index;
                for (col_index, color) in row_pixel.iter().enumerate() {
                    let x_offset = x + col_index;
                    let buffer_index = y_offset * TILEMAP_WIDTH + x_offset;
                    tilemap_buffer[buffer_index] = *color;
                }
            }
        }

        tilemap_buffer
    }
}

fn calculate_address(base_address: u16, x: u8, y: u8) -> u16 {
    let sprites_per_row: u16 = (FULL_WIDTH / TILE_WIDTH as usize) as u16;
    let sprite_x = (x as u16) / TILE_WIDTH as u16;
    let sprite_y = (y as u16) / TILE_HEIGHT as u16;

    let offset = sprite_y * sprites_per_row + sprite_x;

    base_address + offset
}

// The first byte specifies the least significant bit of the color ID of
// each pixel, and the second byte specifies the most significant bit
fn color_index(first_byte: u8, second_byte: u8, pixel_index: u8) -> u8 {
    let lsb = (first_byte >> pixel_index) & 0b1;
    let msb = ((second_byte >> pixel_index) & 0b1) << 1;

    msb | lsb
}
