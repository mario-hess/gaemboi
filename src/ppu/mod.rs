/**
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    May 19, 2024
 */
mod lcd_control;
mod lcd_status;
mod oam;
pub mod tile;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    interrupt::{LCD_STAT_MASK, VBLANK_MASK},
    memory_bus::{OAM_END, OAM_START, VRAM_END, VRAM_START},
    ppu::{
        lcd_control::LCD_control,
        lcd_status::LCD_status,
        oam::OAM,
        tile::{Tile, TILE_HEIGHT, TILE_WIDTH},
    },
};

pub const VRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 40;
const PRIORITY_MAP_SIZE: usize = 256 * 256 + 256;

const TILE_DATA_START: u16 = VRAM_START;
const TILE_DATA_END: u16 = 0x97FF;

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

pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;

pub const TILETABLE_WIDTH: usize = 128;
pub const TILETABLE_HEIGHT: usize = 192;

pub const TILEMAP_WIDTH: usize = 256;
pub const TILEMAP_HEIGHT: usize = TILEMAP_WIDTH;

pub const SCALE: usize = 2;
pub const BUFFER_SIZE: usize = VIEWPORT_WIDTH * VIEWPORT_HEIGHT;

const FULL_WIDTH: usize = TILEMAP_WIDTH;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Mode {
    OAM = 2,
    Transfer = 3,
    HBlank = 0,
    VBlank = 1,
}

#[derive(Copy, Clone)]
enum Priority {
    None,
    Overlap,
}

pub struct Ppu {
    video_ram: [u8; VRAM_SIZE],
    oam: [OAM; OAM_SIZE],
    lcd_control: LCD_control,
    lcd_status: LCD_status,
    scroll_y: u8,
    scroll_x: u8,
    line_y: u8,
    line_y_compare: u8,
    line_y_window: u8,
    bg_palette: [u8; 4],
    tile_palette0: [u8; 4],
    tile_palette1: [u8; 4],
    window_y: u8,
    window_x: u8,
    counter: u16,
    priority_map: [Priority; PRIORITY_MAP_SIZE],
    pub screen_buffer: [Color; BUFFER_SIZE],
    enabled: bool,
    pub interrupts: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            video_ram: [0; VRAM_SIZE],
            oam: [OAM::new(); OAM_SIZE],
            lcd_control: LCD_control::new(),
            lcd_status: LCD_status::new(),
            scroll_y: 0,
            scroll_x: 0,
            line_y: 0,
            line_y_compare: 0,
            line_y_window: 0,
            bg_palette: [0, 1, 2, 3],
            tile_palette0: [0, 1, 2, 3],
            tile_palette1: [0, 1, 2, 3],
            window_y: 0,
            window_x: 0,
            counter: 0,
            priority_map: [Priority::None; PRIORITY_MAP_SIZE],
            screen_buffer: [WHITE; BUFFER_SIZE],
            enabled: true,
            interrupts: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8, canvas: &mut Canvas<Window>) {
        if !self.lcd_control.lcd_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;
        self.counter += t_cycles;

        match self.lcd_status.mode {
            Mode::OAM => {
                if self.counter >= CYCLES_OAM {
                    self.lcd_status
                        .set_mode(Mode::Transfer, &mut self.interrupts);
                    self.counter %= CYCLES_OAM;
                }
            }
            Mode::Transfer => {
                if self.counter >= CYCLES_TRANSFER {
                    self.render_scanline();
                    self.lcd_status.set_mode(Mode::HBlank, &mut self.interrupts);
                    self.counter %= CYCLES_TRANSFER;
                }
            }
            Mode::HBlank => {
                if self.counter >= CYCLES_HBLANK {
                    self.counter %= CYCLES_HBLANK;

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
                            self.line_y_window += 1;
                        }
                        self.set_line_y(self.line_y + 1);
                        self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                    }
                }
            }
            Mode::VBlank => {
                if self.counter >= CYCLES_VBLANK {
                    self.set_line_y(self.line_y + 1);
                    self.counter %= CYCLES_VBLANK;

                    if self.line_y > MAX_LINES_Y {
                        self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                        self.line_y_window = 0;
                        self.set_line_y(0);
                    }
                }
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
            TILE_PALETTE_0 => get_palette(&self.tile_palette0),
            TILE_PALETTE_1 => get_palette(&self.tile_palette1),
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
            LINE_Y => {}, // Not used
            LINE_Y_COMPARE => self.set_line_y_compare(value),
            BG_PALETTE => set_palette(&mut self.bg_palette, value),
            TILE_PALETTE_0 => set_palette(&mut self.tile_palette0, value),
            TILE_PALETTE_1 => set_palette(&mut self.tile_palette1, value),
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
            self.line_y_window = 0;
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
            self.render_tile_line();
        }
    }

    fn render_bg_line(&mut self) {
        let bg_offset_y = self.line_y.wrapping_add(self.scroll_y);
        let row_is_window = self.lcd_control.window_enabled && self.line_y >= self.window_y;

        for x in 0..VIEWPORT_WIDTH as u8 {
            let bg_offset_x = x.wrapping_add(self.scroll_x);
            let col_is_window =
                self.lcd_control.window_enabled && x >= self.window_x.wrapping_sub(7);

            // Determine the address of the tile data based on whether it's in the window or background
            let tile_address = if row_is_window && col_is_window {
                let address = self.lcd_control.get_window_address();
                let y_offset = self.line_y_window;
                let x_offset = x.wrapping_sub(self.window_x.wrapping_sub(7));

                calculate_address(address, y_offset, x_offset)
            } else {
                let address = self.lcd_control.get_bg_address();
                calculate_address(address, bg_offset_y, bg_offset_x)
            };

            let tile_index = self.read_byte(tile_address);
            let tile_address = self.lcd_control.get_address(tile_index);

            // Calculate the offset within the tile data for the current row
            let y_tile_address_offset = if row_is_window && col_is_window {
                (self.line_y - self.window_y) % TILE_HEIGHT as u8 * 2
            } else {
                bg_offset_y % TILE_HEIGHT as u8 * 2
            } as u16;

            let tile_data_address = tile_address + y_tile_address_offset;
            let tile_color_address = tile_data_address + 1;

            let tile_data = self.read_byte(tile_data_address);
            let tile_color = self.read_byte(tile_color_address);

            // Calculate the pixel index based on whether it's part of the window or background
            let pixel_index = if col_is_window && row_is_window {
                self.window_x.wrapping_sub(x) % 8
            } else {
                7 - (bg_offset_x % 8)
            };

            let color_index = get_color_index(tile_data, tile_color, pixel_index);
            let priority_offset = self.line_y as usize + FULL_WIDTH * x as usize;

            if color_index == 0 {
                self.priority_map[priority_offset] = Priority::Overlap
            }

            let pixel = get_pixel_color(&self.bg_palette, color_index);

            // Calculate the offset for the current pixel and update the screen buffer
            let offset = x as usize + self.line_y as usize * VIEWPORT_WIDTH;
            self.screen_buffer[offset] = pixel;
        }
    }

    fn render_tile_line(&mut self) {
        // Convert line_y to an i16 and determine the height of the sprite (8x8 or 8x16)
        let line_y = self.line_y as i16;
        let tile_height = if self.lcd_control.object_size { 16 } else { 8 };

        let mut sorted_sprites: Vec<(usize, i16)> = Vec::new();

        for i in 0..OAM_SIZE {
            let oam_entry = self.oam[i];
            let object_y = oam_entry.y_pos as i16 - 16;
            let object_x = oam_entry.x_pos as i16 - 8;

            if line_y >= object_y && line_y < object_y + tile_height {
                sorted_sprites.push((i, object_x));
            }
        }

        // Stable sort sprites based on X coordinate and index
        sorted_sprites.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        // 10 Objects per line limit, reversed for correct overlapping
        for (index, x_offset) in sorted_sprites.iter().take(10).rev() {
            let i = *index;

            let oam_entry = self.oam[i];
            let object_y = oam_entry.y_pos as i16 - 16;

            let mut object_index = oam_entry.tile_index;

            // Ignore last bit for 8x16 sprites
            if tile_height == 16 {
                object_index &= 0b1111_1110;
            }

            let tile_begin_address = TILE_DATA_START + (object_index as u16 * 16);

            // Check if tile is verticalline_y mirrored
            let line_offset = if oam_entry.y_flip_enabled() {
                tile_height - 1 - (line_y - object_y)
            } else {
                line_y - object_y
            };

            let tile_data_address = tile_begin_address + (line_offset * 2) as u16;
            let tile_color_address = tile_begin_address + (line_offset * 2) as u16 + 1;

            let tile_data = self.read_byte(tile_data_address);
            let tile_color = self.read_byte(tile_color_address);

            for x in 0..8 {
                let x_offset = *x_offset + x as i16;

                // Skip pixels outside of viewport
                if !(0..VIEWPORT_WIDTH as i16).contains(&x_offset) {
                    continue;
                }

                // Check if pixel is horizontalline_y mirrored
                let pixel_index = if oam_entry.x_flip_enabled() { x } else { 7 - x };

                let sprite_palette = if oam_entry.palette_enabled() {
                    self.tile_palette1
                } else {
                    self.tile_palette0
                };

                let color_index = get_color_index(tile_data, tile_color, pixel_index);

                // Skip rendering transparent pixels
                if color_index == 0 {
                    continue;
                }

                let pixel = get_pixel_color(&sprite_palette, color_index);

                // Skip rendering pixel if background overlaps
                let priority_offset = line_y as usize + FULL_WIDTH * x_offset as usize;
                if self.bg_has_priority(&oam_entry, priority_offset) {
                    continue;
                }

                // Calculate the offset for the current pixel and update the screen buffer
                let offset = x_offset + line_y * VIEWPORT_WIDTH as i16;
                self.screen_buffer[offset as usize] = pixel;
            }
        }
    }

    fn bg_has_priority(&self, oam_entry: &OAM, offset: usize) -> bool {
        if !oam_entry.overlap_enabled() {
            return false;
        }

        match self.priority_map[offset] {
            Priority::Overlap => false,
            Priority::None => true,
        }
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
        for i in 0..PRIORITY_MAP_SIZE {
            if i < BUFFER_SIZE {
                self.screen_buffer[i] = WHITE;
            }
            self.priority_map[i] = Priority::None;
        }
    }

    pub fn reset_interrupts(&mut self) {
        self.interrupts = 0;
    }

    pub fn debug_draw_tile_map(
        &self,
        canvas: &mut Canvas<Window>,
        start_address: u16,
        end_address: u16,
    ) {
        let tiles = (start_address..=end_address)
            .map(|i| self.lcd_control.get_address(self.read_byte(i)))
            .flat_map(|address| (0..16).map(move |j| self.read_byte(address + j)))
            .collect::<Vec<u8>>()
            .chunks(16)
            .map(Tile::new)
            .collect::<Vec<Tile>>();

        self.debug_draw(canvas, TILEMAP_WIDTH, &tiles);
    }

    pub fn debug_draw_tile_table(&self, canvas: &mut Canvas<Window>) {
        let tiles = (TILE_DATA_START..=TILE_DATA_END)
            .map(|i| self.read_byte(i))
            .collect::<Vec<u8>>()
            .chunks(16)
            .map(Tile::new)
            .collect::<Vec<Tile>>();

        self.debug_draw(canvas, TILETABLE_WIDTH, &tiles);
    }

    fn debug_draw(&self, canvas: &mut Canvas<Window>, width: usize, tiles: &[Tile]) {
        let tiles_per_row = width / TILE_WIDTH;

        for (index, tile) in tiles.iter().enumerate() {
            let row = index / tiles_per_row;
            let col = index % tiles_per_row;

            let x = col as i32 * TILE_WIDTH as i32;
            let y = row as i32 * TILE_HEIGHT as i32;

            for (row_index, row_pixel) in tile.data.iter().enumerate() {
                let y_offset = y + row_index as i32;
                for (col_index, col_pixel) in row_pixel.iter().enumerate() {
                    let x_offset = x + col_index as i32;
                    let color = match *col_pixel {
                        WHITE => WHITE,
                        LIGHT => LIGHT,
                        DARK => DARK,
                        BLACK => BLACK,
                        _ => unreachable!(),
                    };

                    canvas.set_draw_color(color);
                    canvas.draw_point(Point::new(x_offset, y_offset)).unwrap();
                }
            }
        }
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

fn calculate_address(base_address: u16, row: u8, col: u8) -> u16 {
    let tiles_per_row: u16 = 32;
    let tile_row = (row as u16) / TILE_WIDTH as u16;
    let tile_col = (col as u16) / TILE_HEIGHT as u16;

    let offset = tile_row * tiles_per_row + tile_col;

    base_address + offset
}

fn get_color_index(first_byte: u8, second_byte: u8, pixel_index: u8) -> u8 {
    ((first_byte >> pixel_index) & 1) | ((second_byte >> pixel_index) & 1) << 1
}

fn set_palette(palette: &mut [u8], value: u8) {
    for (i, color_data) in (0..4).map(|i| (i, (value >> (i * 2) & 0x03))) {
        palette[i] = color_data.min(3);
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
