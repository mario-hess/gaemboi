use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::config::Config;
use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, WHITE};

pub fn create_windows(
    config: &Option<Config>,
    video_subsystem: &VideoSubsystem,
) -> Vec<Canvas<Window>> {
    let mut windows = Vec::<Canvas<Window>>::new();

    windows.push(create_canvas(
        video_subsystem,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SCALE,
        "gemboi",
    ));

    if let Some(config) = config {
        if config.tiletable_enable {
            windows.push(create_canvas(
                video_subsystem,
                TILE_TABLE_WIDTH * 8,
                TILE_TABLE_HEIGHT * 8,
                SCALE,
                "tile_table",
            ));
        }

        if config.tilemaps_enable {
            windows.push(create_canvas(
                video_subsystem,
                TILE_MAP_WIDTH * 8,
                TILE_MAP_HEIGHT * 8,
                SCALE,
                "tile_map_0",
            ));

            windows.push(create_canvas(
                video_subsystem,
                TILE_MAP_WIDTH * 8,
                TILE_MAP_HEIGHT * 8,
                SCALE,
                "tile_map_1",
            ));
        }
    }

    windows
}

fn create_canvas(
    video_subsystem: &VideoSubsystem,
    width: usize,
    height: usize,
    scale: usize,
    title: &str,
) -> Canvas<Window> {
    let window = video_subsystem
        .window(title, (width * scale) as u32, (height * scale) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_logical_size(width as u32, height as u32)
        .unwrap();

    canvas
}

pub fn clear_canvases(canvases: &mut Vec<Canvas<Window>>) {
    for canvas in canvases {
        canvas.set_draw_color(WHITE);
        canvas.clear();
    }
}

pub fn present_canvases(canvases: &mut Vec<Canvas<Window>>) {
    for canvas in canvases {
        canvas.present();
    }
}
