/*
 * @file    ui/central_panel.rs
 * @brief   Main panel responsible for displaying the viewport and UI elements.
 * @author  Mario Hess
 * @date    September 13, 2024
 */

use std::f32;
use std::{cell::RefCell, rc::Rc};

use egui_sdl2_gl::{
    egui::{load::SizedTexture, Color32, Image, TextureId, Vec2},
    painter::Painter,
};

use image::GenericImageView;
use std::time::{Duration, Instant};

use crate::{
    cpu::Cpu,
    event_handler::EventHandler,
    ppu::{
        colors::Colors, TILEMAP_END_0, TILEMAP_END_1, TILEMAP_HEIGHT, TILEMAP_START_0,
        TILEMAP_START_1, TILEMAP_WIDTH, TILETABLE_HEIGHT, TILETABLE_WIDTH, VIEWPORT_HEIGHT,
        VIEWPORT_WIDTH,
    },
    View,
};

pub struct CentralPanel {
    pub game_background: Vec<Color32>,
    pub game_texture_id: TextureId,
    pub tiletable_texture_id: TextureId,
    pub tilemap_texture_id: TextureId,
    pub splash_frames: Vec<Vec<Color32>>, // Store each frame as Color32 data
    splash_texture_id: TextureId,
    current_frame: usize,
    last_frame_time: Instant,
    frame_duration: Duration,
}

impl CentralPanel {
    pub fn new(painter: &mut Painter, colors: Rc<RefCell<Colors>>) -> Self {
        let mut splash_frames = Vec::new();
        let mut width = 0;
        let mut height = 0;

        // Load all frames into memory
        for frame_data in get_splash_frames() {
            let img = image::load_from_memory(frame_data).expect("Failed to load frame image");
            let (img_width, img_height) = img.dimensions();
            width = img_width as usize; // Assuming all frames have the same dimensions
            height = img_height as usize;

            let image_data: Vec<Color32> = img
                .to_rgba8()
                .pixels()
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect();
            splash_frames.push(image_data);
        }

        // Use the width and height of the loaded frames to define the texture size
        let splash_texture_id = painter.new_user_texture(
            (width, height),   // Use the dimensions of the images directly
            &splash_frames[0], // Use the first frame initially
            false,
        );

        let mut game_background: Vec<Color32> =
            Vec::with_capacity(VIEWPORT_WIDTH * VIEWPORT_HEIGHT);
        let borrowed_colors = colors.as_ref().borrow();
        game_background.fill(Color32::from_rgb(
            borrowed_colors.white.r(),
            borrowed_colors.white.g(),
            borrowed_colors.white.b(),
        ));

        let game_texture_id = painter.new_user_texture(
            (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            &vec![borrowed_colors.black; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            false,
        );

        let tiletable_texture_id = painter.new_user_texture(
            (TILETABLE_WIDTH, TILETABLE_HEIGHT),
            &vec![borrowed_colors.black; TILETABLE_WIDTH * TILETABLE_HEIGHT],
            false,
        );

        let tilemap_texture_id = painter.new_user_texture(
            (TILEMAP_WIDTH, TILEMAP_HEIGHT),
            &vec![borrowed_colors.black; TILEMAP_WIDTH * TILEMAP_HEIGHT],
            false,
        );

        Self {
            game_background,
            game_texture_id,
            tiletable_texture_id,
            tilemap_texture_id,
            splash_frames,
            splash_texture_id,
            current_frame: 0,
            last_frame_time: Instant::now(),
            frame_duration: Duration::from_millis(100),
        }
    }

    pub fn draw(
        &mut self,
        egui_ctx: &egui_sdl2_gl::egui::Context,
        event_handler: &EventHandler,
        cpu: Option<&mut Cpu>,
        current_view: &View,
        painter: &mut Painter,
    ) {
        egui_sdl2_gl::egui::CentralPanel::default()
            .frame(egui_sdl2_gl::egui::Frame::none())
            .show(egui_ctx, |ui| {
                if let Some(cpu) = cpu {
                    match current_view {
                        View::Viewport => {
                            self.game_background = cpu
                                .memory_bus
                                .ppu
                                .viewport_buffer
                                .iter()
                                .map(|color| Color32::from_rgb(color.r(), color.g(), color.b()))
                                .collect();

                            painter.update_user_texture_data(
                                self.game_texture_id,
                                &self.game_background,
                            );
                            let game_image = Image::new(SizedTexture::new(
                                self.game_texture_id,
                                Vec2::new(
                                    VIEWPORT_WIDTH as f32 * event_handler.window_scale as f32,
                                    VIEWPORT_HEIGHT as f32 * event_handler.window_scale as f32,
                                ),
                            ))
                            .maintain_aspect_ratio(true);
                            ui.add(game_image);

                            cpu.memory_bus.ppu.clear_screen();
                        }
                        View::Tiletable => {
                            let tiletable_background: Vec<Color32> = cpu
                                .memory_bus
                                .ppu
                                .tiletable()
                                .iter()
                                .map(|color| Color32::from_rgb(color.r(), color.g(), color.b()))
                                .collect();
                            painter.update_user_texture_data(
                                self.tiletable_texture_id,
                                &tiletable_background,
                            );
                            let tiletable_image = Image::new(SizedTexture::new(
                                self.tiletable_texture_id,
                                Vec2::new(
                                    TILETABLE_WIDTH as f32 * event_handler.window_scale as f32,
                                    TILETABLE_HEIGHT as f32 * event_handler.window_scale as f32,
                                ),
                            ))
                            .maintain_aspect_ratio(true);
                            ui.add(tiletable_image);
                        }
                        View::Tilemap0 => {
                            let tilemap_background: Vec<Color32> = cpu
                                .memory_bus
                                .ppu
                                .tilemap(TILEMAP_START_0, TILEMAP_END_0)
                                .iter()
                                .map(|color| Color32::from_rgb(color.r(), color.g(), color.b()))
                                .collect();
                            painter.update_user_texture_data(
                                self.tilemap_texture_id,
                                &tilemap_background,
                            );
                            let tilemap_image = Image::new(SizedTexture::new(
                                self.tilemap_texture_id,
                                Vec2::new(
                                    TILEMAP_WIDTH as f32 * event_handler.window_scale as f32,
                                    TILEMAP_HEIGHT as f32 * event_handler.window_scale as f32,
                                ),
                            ))
                            .maintain_aspect_ratio(true);
                            ui.add(tilemap_image);
                        }
                        View::Tilemap1 => {
                            let tilemap_background: Vec<Color32> = cpu
                                .memory_bus
                                .ppu
                                .tilemap(TILEMAP_START_1, TILEMAP_END_1)
                                .iter()
                                .map(|color| Color32::from_rgb(color.r(), color.g(), color.b()))
                                .collect();
                            painter.update_user_texture_data(
                                self.tilemap_texture_id,
                                &tilemap_background,
                            );
                            let tilemap_image = Image::new(SizedTexture::new(
                                self.tilemap_texture_id,
                                Vec2::new(
                                    TILEMAP_WIDTH as f32 * event_handler.window_scale as f32,
                                    TILEMAP_HEIGHT as f32 * event_handler.window_scale as f32,
                                ),
                            ))
                            .maintain_aspect_ratio(true);
                            ui.add(tilemap_image);
                        }
                    }
                } else {
                    // Update the current frame if enough time has passed
                    if self.last_frame_time.elapsed() >= self.frame_duration {
                        self.current_frame = (self.current_frame + 1) % self.splash_frames.len();
                        self.last_frame_time = Instant::now();

                        // Update the texture with the current frame
                        painter.update_user_texture_data(
                            self.splash_texture_id,
                            &self.splash_frames[self.current_frame],
                        );
                    }

                    // Display the current frame
                    let splash_image = Image::new(SizedTexture::new(
                        self.splash_texture_id,
                        Vec2::new(
                            VIEWPORT_WIDTH as f32 * event_handler.window_scale as f32,
                            VIEWPORT_HEIGHT as f32 * event_handler.window_scale as f32,
                        ),
                    ))
                    .maintain_aspect_ratio(true);

                    ui.add(splash_image);
                }
            });
    }
}

macro_rules! include_splash_frames {
    ($($i:expr),+) => {
        vec![
            $(include_bytes!(concat!("../../images/splash/gaemboi", $i, ".png"))),+
        ]
    };
}

fn get_splash_frames() -> Vec<&'static [u8]> {
    include_splash_frames!(2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14)
}
