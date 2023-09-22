/**
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    September 22, 2023
 */
mod lcd_control;
mod lcd_status;
mod oam;
pub mod screen;
pub mod tile;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::memory_bus::{OAM_END, OAM_START, VRAM_END, VRAM_START};
use crate::ppu::lcd_control::LCD_control;
use crate::ppu::lcd_status::LCD_status;
use crate::ppu::oam::OAM;
use crate::ppu::tile::{Tile, TILE_HEIGHT, TILE_WIDTH};

pub const VRAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 40; // 40 * 4 = 160 byte

const TILE_DATA_START: u16 = VRAM_START;
const TILE_DATA_END: u16 = 0x97FF;

pub const TILE_MAP_START_0: u16 = 0x9800;
pub const TILE_MAP_END_0: u16 = 0x9BFF;

pub const TILE_MAP_START_1: u16 = 0x9C00;
pub const TILE_MAP_END_1: u16 = VRAM_END;

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

pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const DARK: Color = Color::RGB(96, 96, 96);
pub const LIGHT: Color = Color::RGB(192, 192, 192);
pub const WHITE: Color = Color::RGB(255, 255, 255);

const WIDTH: usize = 32;
const HEIGHT: usize = 12;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAM = 2,
    VRam = 3,
}

#[derive(Copy, Clone)]
pub struct Ppu {
    video_ram: [u8; VRAM_SIZE],
    oam: [OAM; OAM_SIZE],
    lcd_control: LCD_control,
    lcd_status: LCD_status,
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
            video_ram: [0; VRAM_SIZE],
            oam: [OAM::new(); OAM_SIZE],
            lcd_control: LCD_control::new(),
            lcd_status: LCD_status::new(),
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
            OAM_START..=OAM_END => self.read_oam(address - OAM_START),
            LCD_CONTROL => self.lcd_control.get(),
            LCD_STATUS => self.lcd_status.get(),
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
            _ => panic!("Unknown address: {:#X}. Can't read byte.", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => self.video_ram[(address - VRAM_START) as usize] = value,
            OAM_START..=OAM_END => self.write_oam(address - OAM_START, value),
            LCD_CONTROL => self.lcd_control.set(value),
            LCD_STATUS => self.lcd_status.set(value),
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
            _ => panic!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
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

    pub fn debug_draw_tile_map(
        &mut self,
        tile_map_canvas: &mut Canvas<Window>,
        start_address: u16,
        end_address: u16,
    ) {
        let mut tile_indexes = Vec::<u8>::new();

        for i in start_address..=end_address {
            tile_indexes.push(self.read_byte(i));
        }

        let mut tile_data = Vec::<u8>::new();

        for tile_index in tile_indexes {
            let address = self.lcd_control.get_address(tile_index);
            tile_data.push(self.read_byte(address));
        }

        let mut tile_map = Vec::<Tile>::new();

        for chunk in tile_data.chunks(16) {
            let mut tile_bytes = [0; 16];
            tile_bytes.copy_from_slice(chunk);

            let tile = Tile::new(tile_bytes);
            tile_map.push(tile);
        }

        for row in 0..WIDTH {
            for col in 0..WIDTH {
                let tile_index = row * WIDTH + col;

                if tile_index < tile_map.len() {
                    let tile = &tile_map[tile_index];

                    let x = col * TILE_WIDTH;
                    let y = row * TILE_HEIGHT;

                    for (tile_row, row_pixels) in tile.data.iter().enumerate() {
                        for (tile_col, pixel) in row_pixels.iter().enumerate() {
                            let color = match *pixel {
                                WHITE => WHITE,
                                LIGHT => LIGHT,
                                DARK => DARK,
                                BLACK => BLACK,
                                _ => unreachable!(),
                            };

                            tile_map_canvas.set_draw_color(color);

                            tile_map_canvas
                                .draw_point(Point::new(
                                    x as i32 + tile_col as i32,
                                    y as i32 + tile_row as i32,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

    pub fn debug_draw_tile_table(&mut self, tile_data_canvas: &mut Canvas<Window>) {
        let mut tile_data = Vec::<u8>::new();

        // Tile data is stored in VRAM in the memory area at 0x8000-0x97FF.
        for i in TILE_DATA_START..=TILE_DATA_END {
            tile_data.push(self.read_byte(i));
        }

        let mut tile_table = Vec::<Tile>::new();

        for chunk in tile_data.chunks(16) {
            let mut tile_bytes = [0; 16];
            tile_bytes.copy_from_slice(chunk);

            let tile = Tile::new(tile_bytes);
            tile_table.push(tile);
        }

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let tile_index = row * WIDTH + col;

                if tile_index < tile_table.len() {
                    let tile = &tile_table[tile_index];

                    let x = col * TILE_WIDTH;
                    let y = row * TILE_HEIGHT;

                    for (tile_row, row_pixels) in tile.data.iter().enumerate() {
                        for (tile_col, pixel) in row_pixels.iter().enumerate() {
                            let color = match *pixel {
                                WHITE => WHITE,
                                LIGHT => LIGHT,
                                DARK => DARK,
                                BLACK => BLACK,
                                _ => unreachable!(),
                            };

                            tile_data_canvas.set_draw_color(color);

                            tile_data_canvas
                                .draw_point(Point::new(
                                    x as i32 + tile_col as i32,
                                    y as i32 + tile_row as i32,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}
