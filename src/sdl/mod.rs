/*
pub mod window;

use crate::{
    config::Config,
    event_handler::EventHandler,
    ppu::{TILETABLE_HEIGHT, TILETABLE_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    sdl::window::Window,
};
use sdl2::{controller::GameController, AudioSubsystem, EventPump, VideoSubsystem};

#[allow(clippy::upper_case_acronyms)]
pub struct SDL {
    pub video_subsystem: VideoSubsystem,
    pub audio_subsystem: AudioSubsystem,
    pub event_pump: EventPump,
    pub window: Window,
    pub tiletable: Option<Window>,
    _gamepad: Option<GameController>,
}

impl SDL {
    pub fn new(event_handler: &EventHandler, config: &Config) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();
        let controller_subsystem = sdl_context.game_controller().unwrap();

        // Initialize gamepad
        let available = controller_subsystem
            .num_joysticks()
            .map_err(|e| format!("can't enumerate joysticks: {}", e))
            .unwrap();

        let _gamepad = (0..available).find_map(|id| {
            if !controller_subsystem.is_game_controller(id) {
                println!("{} is not a gamepad", id);
                return None;
            }

            println!("Attempting to open gamepad {}", id);

            match controller_subsystem.open(id) {
                Ok(gamepad) => {
                    println!("Success: opened \"{}\"", gamepad.name());
                    Some(gamepad)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        });

        let event_pump = sdl_context.event_pump().unwrap();

        let window = Window::build(
            &video_subsystem,
            "gaemboi",
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            event_handler.window_scale as usize,
        );

        let tiletable = if config.tiletable_enable {
            Some(Window::build(
                &video_subsystem,
                "tiletable",
                TILETABLE_WIDTH,
                TILETABLE_HEIGHT,
                event_handler.window_scale as usize,
            ))
        } else {
            None
        };

        Self {
            video_subsystem,
            audio_subsystem,
            event_pump,
            window,
            tiletable,
            _gamepad,
        }
    }
}
*/
