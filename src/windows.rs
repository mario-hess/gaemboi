use sdl2::render::Canvas;
use sdl2::video::Window as SDL_Window;
use sdl2::VideoSubsystem;

use crate::config::Config;
use crate::ppu::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, WHITE};

pub const VIEWPORT_WIDTH: u16 = 160;
pub const VIEWPORT_HEIGHT: u16 = 144;
pub const SCALE: u8 = 4;

pub struct Windows {
    viewport: Canvas<SDL_Window>,
    pub tiletable: Option<Canvas<SDL_Window>>,
    pub tilemap_9800: Option<Canvas<SDL_Window>>,
    pub tilemap_9c00: Option<Canvas<SDL_Window>>,
}

impl Windows {
    pub fn build(config: &Option<Config>, video_subsystem: &VideoSubsystem) -> Self {
        let viewport = create_canvas(
            video_subsystem,
            "Viewport",
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            SCALE,
        );

        let (tiletable, tilemap_9800, tilemap_9c00) = match config {
            Some(config) => (
                if config.tiletable_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tile Table",
                        TILE_TABLE_WIDTH as u16 * 8,
                        TILE_TABLE_HEIGHT as u16 * 8,
                        SCALE,
                    ))
                } else {
                    None
                },
                if config.tilemaps_enable || config.tilemap_9800_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tilemap 9800",
                        TILE_MAP_WIDTH as u16 * 8,
                        TILE_MAP_HEIGHT as u16 * 8,
                        SCALE,
                    ))
                } else {
                    None
                },
                if config.tilemaps_enable || config.tilemap_9c00_enable {
                    Some(create_canvas(
                        video_subsystem,
                        "Tilemap 9C00",
                        TILE_MAP_WIDTH as u16 * 8,
                        TILE_MAP_HEIGHT as u16 * 8,
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

        let canvases: Vec<&mut Option<Canvas<SDL_Window>>> = vec![
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

        let canvases: Vec<&mut Option<Canvas<SDL_Window>>> = vec![
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
    width: u16,
    height: u16,
    scale: u8,
) -> Canvas<SDL_Window> {
    let window = video_subsystem
        .window(
            title,
            width as u32 * scale as u32,
            height as u32 * scale as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_logical_size(width as u32, height as u32)
        .unwrap();

    canvas
}
