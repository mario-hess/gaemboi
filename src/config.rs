/*
 * @file    config.rs
 * @brief   Handles configuration settings.
 * @author  Mario Hess
 * @date    May 23, 2024
 */

pub struct Config {
    pub file_path: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Self {
        let mut file_path = None;

        for arg in args {
            if !arg.contains("gaemboi") {
                file_path = Some("roms/".to_owned() + arg)
            }
        }

        Self { file_path }
    }
}
