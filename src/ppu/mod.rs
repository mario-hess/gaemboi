/**
 * @file    ppu/mod.rs
 * @brief   Handles the Picture Processing Unit for graphics rendering.
 * @author  Mario Hess
 * @date    September 23, 2023
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

const CYCLES_OAM: u16 = 80;
const CYCLES_VRAM: u16 = 172;
const CYCLES_HBLANK: u16 = 204;
const CYCLES_VBLANK: u16 = 456;

const LINES_Y: u8 = 143;
const MAX_LINES_Y: u8 = 153;

pub const VIEWPORT_WIDTH: usize = 20;
pub const VIEWPORT_HEIGHT: usize = 18;

pub const TILE_TABLE_WIDTH: usize = 16;
pub const TILE_TABLE_HEIGHT: usize = 24;

pub const TILE_MAP_WIDTH: usize = 32;
pub const TILE_MAP_HEIGHT: usize = TILE_MAP_WIDTH;

pub const SCALE: usize = 2;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAM = 2,
    Transfer = 3,
}

#[derive(Copy, Clone)]
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
    background_palette: u8,
    object_palette_0: u8,
    object_palette_1: u8,
    window_y: u8,
    window_x: u8,
    counter: u16,
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
            background_palette: 0,
            object_palette_0: 0,
            object_palette_1: 0,
            window_y: 0,
            window_x: 0,
            counter: 0,
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
                        // TODO: render viewport
                        self.interrupts |= VBLANK_MASK;
                        // TODO: lear viewport
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
                    // TODO: render scanline to screen
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
            LCD_CONTROL => self.set_lcd_control(value),
            LCD_STATUS => self.lcd_status.set(value),
            SCROLL_Y => self.scroll_y = value,
            SCROLL_X => self.scroll_x = value,
            LINE_Y => self.line_y = value,
            LINE_Y_COMPARE => self.set_line_y(value),
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
            // TODO: clear screen

            self.set_line_y(0);
            self.lcd_status.mode = Mode::HBlank;
            self.counter = 0;
        }
    }

    pub fn reset_interrupts(&mut self) {
        self.interrupts = 0;
    }

    pub fn debug_draw_tile_map(
        self,
        canvas: &mut Canvas<Window>,
        start_address: u16,
        end_address: u16,
    ) {
        let mut tile_addr = Vec::<u16>::new();

        for i in start_address..=end_address {
            tile_addr.push(self.lcd_control.get_address(self.read_byte(i)));
        }

        let tile_data: Vec<u8> = tile_addr
            .iter()
            .flat_map(|&address| (0..16).map(move |i| self.read_byte(address + i)))
            .collect();

        let tiles = Tile::generate_tiles(tile_data);

        self.draw(canvas, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, tiles);
    }

    pub fn debug_draw_tile_table(&self, canvas: &mut Canvas<Window>) {
        let mut tile_data = Vec::<u8>::new();

        for i in TILE_DATA_START..=TILE_DATA_END {
            tile_data.push(self.read_byte(i));
        }

        let tiles = Tile::generate_tiles(tile_data);

        self.draw(canvas, TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, tiles);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, height: usize, width: usize, tiles: Vec<Tile>) {
        for row in 0..height {
            for col in 0..width {
                let tile_index = row * width + col;

                let tile = &tiles[tile_index];

                let x = col * TILE_WIDTH;
                let y = row * TILE_HEIGHT;

                for (row_index, row_pixel) in tile.data.iter().enumerate() {
                    for (col_index, col_pixel) in row_pixel.iter().enumerate() {
                        let color = match *col_pixel {
                            WHITE => WHITE,
                            LIGHT => LIGHT,
                            DARK => DARK,
                            BLACK => BLACK,
                            _ => unreachable!(),
                        };

                        canvas.set_draw_color(color);

                        canvas
                            .draw_point(Point::new(
                                x as i32 + col_index as i32,
                                y as i32 + row_index as i32,
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
}
