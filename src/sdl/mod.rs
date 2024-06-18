pub mod window;

use crate::{
    event_handler::EventHandler,
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH},
    sdl::window::Window,
};
use sdl2::{
    controller::GameController, ttf::Sdl2TtfContext, AudioSubsystem, EventPump, VideoSubsystem,
};

#[allow(clippy::upper_case_acronyms)]
pub struct SDL<'a> {
    pub video_subsystem: VideoSubsystem,
    pub audio_subsystem: AudioSubsystem,
    pub event_pump: EventPump,
    pub window: Window<'a>,
    _gamepad: Option<GameController>,
}

impl<'a> SDL<'a> {
    pub fn new(event_handler: &EventHandler, ttf_context: &'a Sdl2TtfContext) -> Self {
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
            ttf_context,
            "gaemboi",
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            event_handler.window_scale as usize,
        );

        Self {
            video_subsystem,
            audio_subsystem,
            event_pump,
            window,
            _gamepad,
        }
    }
}
