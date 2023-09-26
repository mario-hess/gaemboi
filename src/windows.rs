/**
 * @file    windows.rs
 * @brief   Handles windows for rendering graphics based on the configuration.
 * @author  Mario Hess
 * @date    September 26, 2023
 */
use sdl2::render::{Canvas, CanvasBuilder};
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::config::Config;
use crate::ppu::tile::{TILE_HEIGHT, TILE_WIDTH};
use crate::ppu::{
    SCALE, TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, VIEWPORT_HEIGHT,
    VIEWPORT_WIDTH, WHITE,
};

pub struct Windows {
    viewport: Canvas<Window>,
    pub tiletable: Option<Canvas<Window>>,
    pub tilemap_9800: Option<Canvas<Window>>,
    pub tilemap_9c00: Option<Canvas<Window>>,
}

impl Windows {
    pub fn build(config: &Option<Config>, video_subsystem: &VideoSubsystem) -> Self {
        let viewport = create_canvas(
            video_subsystem,
            "Viewport",
            VIEWPORT_WIDTH * TILE_WIDTH,
            VIEWPORT_HEIGHT * TILE_HEIGHT,
            SCALE,
        );

        let (tiletable, tilemap_9800, tilemap_9c00) = match config {
            Some(config) => (
                if config.tiletable_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tile Table",
                        TILE_TABLE_WIDTH * TILE_WIDTH,
                        TILE_TABLE_HEIGHT * TILE_HEIGHT,
                        SCALE,
                    ))
                } else {
                    None
                },
                if config.tilemaps_enable || config.tilemap_9800_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tilemap 9800",
                        TILE_MAP_WIDTH * TILE_WIDTH,
                        TILE_MAP_HEIGHT * TILE_HEIGHT,
                        SCALE,
                    ))
                } else {
                    None
                },
                if config.tilemaps_enable || config.tilemap_9c00_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tilemap 9C00",
                        TILE_MAP_WIDTH * TILE_WIDTH,
                        TILE_MAP_HEIGHT * TILE_HEIGHT,
                        SCALE,
                    ))
                } else {
                    None
                },
            ),
            None => (None, None, None),
        };

        Self {
            viewport,
            tiletable,
            tilemap_9800,
            tilemap_9c00,
        }
    }

    pub fn clear(&mut self) {
        self.viewport.set_draw_color(WHITE);
        self.viewport.clear();

        let canvases: Vec<&mut Option<Canvas<Window>>> = vec![
            &mut self.tiletable,
            &mut self.tilemap_9800,
            &mut self.tilemap_9c00,
        ];

        for canvas in canvases.into_iter().flatten() {
            canvas.set_draw_color(WHITE);
            canvas.clear();
        }
    }

    pub fn present(&mut self) {
        self.viewport.present();

        let canvases: Vec<&mut Option<Canvas<Window>>> = vec![
            &mut self.tiletable,
            &mut self.tilemap_9800,
            &mut self.tilemap_9c00,
        ];

        for canvas in canvases.into_iter().flatten() {
            canvas.present();
        }
    }
}

fn create_canvas(
    video_subsystem: &VideoSubsystem,
    title: &str,
    width: usize,
    height: usize,
    scale: usize,
) -> Canvas<Window> {
    let window = video_subsystem
        .window(
            title,
            width as u32 * scale as u32,
            height as u32 * scale as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = CanvasBuilder::new(window).accelerated().build().unwrap();

    canvas
        .set_logical_size(width as u32, height as u32)
        .unwrap();

    canvas
}
