/**
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    October 15, 2023
 */
mod lcd_control;
mod lcd_status;
mod oam;
pub mod tile;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::interrupt::{LCD_STAT_MASK, VBLANK_MASK};
use crate::memory_bus::{OAM_END, OAM_START, VRAM_END, VRAM_START};
use crate::ppu::lcd_control::LCD_control;
use crate::ppu::lcd_status::LCD_status;
use crate::ppu::oam::OAM;
use crate::ppu::tile::{Tile, TILE_HEIGHT, TILE_WIDTH};

pub const VRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 40;

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
const DMA: u16 = 0xFF46;
const BG_PALETTE: u16 = 0xFF47;
const OBJ_PALETTE_0: u16 = 0xFF48;
const OBJ_PALETTE_1: u16 = 0xFF49;
const WINDOW_Y: u16 = 0xFF4A;
const WINDOW_X: u16 = 0xFF4B;

pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const DARK: Color = Color::RGB(96, 96, 96);
pub const LIGHT: Color = Color::RGB(192, 192, 192);
pub const WHITE: Color = Color::RGB(255, 255, 255);

const CYCLES_OAM: u16 = 80;
const CYCLES_VRAM: u16 = 172;
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

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAM = 2,
    Transfer = 3,
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
    pub line_y: u8,
    line_y_compare: u8,
    dma: u8,
    bg_palette: u8,
    obj_palette0: u8,
    obj_palette1: u8,
    window_y: u8,
    window_x: u8,
    counter: u16,
    priority_map: [Priority; 65792],
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
            dma: 0,
            bg_palette: 0,
            obj_palette0: 0,
            obj_palette1: 0,
            window_y: 0,
            window_x: 0,
            counter: 0,
            priority_map: [Priority::None; 65792],
            screen_buffer: [WHITE; BUFFER_SIZE],
            enabled: true,
            interrupts: 0,
        }
    }

    pub fn tick(&mut self, m_cycles: u8) {
        if !self.lcd_control.lcd_enabled {
            return;
        }

        let t_cycles = (m_cycles * 4) as u16;
        self.counter += t_cycles;

        match self.lcd_status.mode {
            Mode::HBlank => {
                if self.counter >= CYCLES_HBLANK {
                    self.counter %= CYCLES_HBLANK;

                    if self.line_y >= LINES_Y {
                        self.lcd_status.set_mode(Mode::VBlank, &mut self.interrupts);
                        //self.render_screen();
                        self.interrupts |= VBLANK_MASK;
                        //self.clear_screen();
                    } else {
                        self.line_y = self.line_y.wrapping_add(1);
                        self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                    }
                }
            }
            Mode::VBlank => {
                if self.counter >= CYCLES_VBLANK {
                    self.line_y = self.line_y.wrapping_add(1);
                    self.counter %= CYCLES_VBLANK;

                    if self.line_y > MAX_LINES_Y {
                        self.lcd_status.set_mode(Mode::OAM, &mut self.interrupts);
                        self.line_y = 0;
                    }
                }
            }
            Mode::OAM => {
                if self.counter >= CYCLES_OAM {
                    self.lcd_status
                        .set_mode(Mode::Transfer, &mut self.interrupts);
                    self.counter %= CYCLES_OAM;
                }
            }
            Mode::Transfer => {
                if self.counter >= CYCLES_VRAM {
                    self.render_scanline();
                    self.lcd_status.set_mode(Mode::HBlank, &mut self.interrupts);
                    self.counter %= CYCLES_VRAM;
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
            DMA => self.dma,
            BG_PALETTE => self.bg_palette,
            OBJ_PALETTE_0 => self.obj_palette0,
            OBJ_PALETTE_1 => self.obj_palette1,
            WINDOW_Y => self.window_y,
            WINDOW_X => self.window_x,
            _ => panic!("Unknown address: {:#X}. Can't read byte.", address),
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
            LINE_Y => self.line_y = value,
            LINE_Y_COMPARE => self.set_line_y(value),
            DMA => self.dma = value,
            BG_PALETTE => self.bg_palette = value,
            OBJ_PALETTE_0 => self.obj_palette0 = value,
            OBJ_PALETTE_1 => self.obj_palette1 = value,
            WINDOW_Y => self.window_y = value,
            WINDOW_X => self.window_x = value,
            _ => panic!(
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
            //self.clear_screen();
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

        if self.lcd_control.obj_enabled {
            //self.render_obj_line();
        }
    }

    fn render_bg_line(&mut self) {
        let bg_offset_y = self.line_y.wrapping_add(self.scroll_y);
        let row_is_window = self.lcd_control.window_enabled && self.line_y >= self.window_y;

        for x in 0..VIEWPORT_WIDTH {
            let bg_offset_x = x.wrapping_add(self.scroll_x as usize);
            let column_is_window =
                self.lcd_control.window_enabled && x >= self.window_x.wrapping_sub(7) as usize;

            let tile_address = if row_is_window && column_is_window {
                let address = self.lcd_control.get_window_address();
                let y_offset = self.line_y.wrapping_sub(self.window_y);
                let x_offset = x.wrapping_sub(self.window_x.wrapping_sub(7) as usize);

                calculate_address(address, y_offset, x_offset as u8)
            } else {
                let address = self.lcd_control.get_bg_address();
                calculate_address(address, bg_offset_y, bg_offset_x as u8)
            };

            let tile_index = self.read_byte(tile_address);
            let tile_address = self.lcd_control.get_address(tile_index);

            let y_tile_address_offset = if row_is_window && column_is_window {
                (self.line_y - self.window_y) % 8 * 2
            } else {
                bg_offset_y % 8 * 2
            } as u16;

            let first_byte_address = tile_address + y_tile_address_offset;
            let second_byte_address = first_byte_address + 1;

            let first_byte = self.read_byte(first_byte_address);
            let second_byte = self.read_byte(second_byte_address);

            let pixel_index = if column_is_window && row_is_window {
                self.window_x.wrapping_sub(x as u8) % 8
            } else {
                7 - (bg_offset_x % 8) as u8
            };

            let bit1 = (first_byte >> pixel_index) & 1;
            let bit2 = (second_byte >> pixel_index) & 1;

            let pixel = match (bit2, bit1) {
                (0, 0) => WHITE,
                (0, 1) => LIGHT,
                (1, 0) => DARK,
                (1, 1) => BLACK,
                _ => unreachable!(),
            };

            let offset = self.line_y as usize + 256 * x;

            if pixel == WHITE {
                self.priority_map[offset] = Priority::Overlap
            }

            self.draw_to_buffer(x, self.line_y as usize, pixel);
        }
    }

    fn draw_to_buffer(&mut self, x: usize, y: usize, color: Color) {
        let offset = x + y * VIEWPORT_WIDTH;

        self.screen_buffer[offset] = color;
    }

    pub fn draw_viewport(&self, canvas: &mut Canvas<Window>) {
        for (index, pixel) in self.screen_buffer.iter().enumerate() {
            let color = *pixel;
            let x_coord = (index % VIEWPORT_WIDTH) as i32;
            let y_coord = (index / VIEWPORT_WIDTH) as i32;

            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(x_coord, y_coord)).unwrap();
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

fn calculate_address(address: u16, y: u8, x: u8) -> u16 {
    address + (y as u16 / 8 * 32) + (x as u16 / 8)
}
