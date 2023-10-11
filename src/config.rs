/**
 * @file    config.rs
 * @brief   Handles configuration settings.
 * @author  Mario Hess
 * @date    October 11, 2023
 */

use crate::Mode;

pub struct Config {
    pub file_path: Option<String>,
    pub tiletable_enable: bool,
    pub tilemaps_enable: bool,
    pub tilemap_9800_enable: bool,
    pub tilemap_9c00_enable: bool,
    pub mode: Mode,
}

impl Config {
    pub fn build(args: &[String]) -> Self {
        let mut file_path = None;

        for arg in args {
            if !arg.starts_with('-') && !arg.contains("target/release/gemboi") {
                file_path = Some("roms/".to_owned() + arg)
            }
        }

        let tiletable_enable = args.contains(&"-t".to_string());
        let tilemaps_enable = args.contains(&"-m".to_string());
        let tilemap_9800_enable = args.contains(&"-m1".to_string());
        let tilemap_9c00_enable = args.contains(&"-m2".to_string());
        let mode = Mode::Splash;

        Self {
            file_path,
            tiletable_enable,
            tilemaps_enable,
            tilemap_9800_enable,
            tilemap_9c00_enable,
            mode,
        }
    }
}
