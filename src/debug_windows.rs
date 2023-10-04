/**
 * @file    debug_windows.rs
 * @brief   Handles debug windows.
 * @author  Mario Hess
 * @date    October 04, 2023
 */
use sdl2::VideoSubsystem;
use sdl2::ttf::Sdl2TtfContext;

use crate::config::Config;
use crate::ppu::{SCALE, TILEMAP_HEIGHT, TILEMAP_WIDTH, TILETABLE_HEIGHT, TILETABLE_WIDTH, WHITE};
use crate::window::Window;

pub struct DebugWindows<'a> {
    pub tiletable: Option<Window<'a>>,
    pub tilemap_9800: Option<Window<'a>>,
    pub tilemap_9c00: Option<Window<'a>>,
}

impl<'a> DebugWindows<'a> {
    pub fn build(
        video_subsystem: &VideoSubsystem,
        ttf_context: &'a Sdl2TtfContext,
        config: &Config,
    ) -> Self {
        let tiletable = if config.tiletable_enable {
            Some(Window::build(
                video_subsystem,
                ttf_context,
                "Tile Table",
                TILETABLE_WIDTH,
                TILETABLE_HEIGHT,
                SCALE,
            ))
        } else {
            None
        };

        let tilemap_9800 = if config.tilemaps_enable || config.tilemap_9800_enable {
            Some(Window::build(
                video_subsystem,
                ttf_context,
                "Tilemap 9800",
                TILEMAP_WIDTH,
                TILEMAP_HEIGHT,
                SCALE,
            ))
        } else {
            None
        };

        let tilemap_9c00 = if config.tilemaps_enable || config.tilemap_9c00_enable {
            Some(Window::build(
                video_subsystem,
                ttf_context,
                "Tilemap 9C00",
                TILEMAP_WIDTH,
                TILEMAP_HEIGHT,
                SCALE,
            ))
        } else {
            None
        };

        Self {
            tiletable,
            tilemap_9800,
            tilemap_9c00,
        }
    }

    pub fn clear(&mut self) {
        let windows: Vec<&mut Option<Window>> = vec![
            &mut self.tiletable,
            &mut self.tilemap_9800,
            &mut self.tilemap_9c00,
        ];

        for window in windows.into_iter().flatten() {
            window.canvas.set_draw_color(WHITE);
            window.canvas.clear();
        }
    }

    pub fn present(&mut self) {
        let windows: Vec<&mut Option<Window>> = vec![
            &mut self.tiletable,
            &mut self.tilemap_9800,
            &mut self.tilemap_9c00,
        ];

        for window in windows.into_iter().flatten() {
            window.canvas.present();
        }
    }
}
