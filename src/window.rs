use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::ppu::screen::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_TABLE_HEIGHT, TILE_TABLE_WIDTH, WHITE};

pub fn create_windows(video_subsystem: &VideoSubsystem) -> [Canvas<Window>; 4] {
    let viewport = create_canvas(
        video_subsystem,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SCALE,
        "gemboi",
    );

    let tile_table = create_canvas(
        video_subsystem,
        TILE_TABLE_WIDTH * 8,
        TILE_TABLE_HEIGHT * 8,
        SCALE,
        "tile_table",
    );

    let tile_map_0 = create_canvas(
        video_subsystem,
        TILE_MAP_WIDTH * 8,
        TILE_MAP_HEIGHT * 8,
        SCALE,
        "tile_map_0",
    );

    let tile_map_1 = create_canvas(
        video_subsystem,
        TILE_MAP_WIDTH * 8,
        TILE_MAP_HEIGHT * 8,
        SCALE,
        "tile_map_1",
    );

    [viewport, tile_table, tile_map_0, tile_map_1]
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

pub fn clear_canvases(canvases: &mut [Canvas<Window>; 4]) {
    for canvas in canvases {
        canvas.set_draw_color(WHITE);
        canvas.clear();
    }
}

pub fn present_canvases(canvases: &mut [Canvas<Window>; 4]) {
    for canvas in canvases {
        canvas.present();
    }
}
