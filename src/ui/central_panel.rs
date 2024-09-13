/*
 * @file    ui/central_panel.rs
 * @brief   Main panel responsible for displaying the viewport and UI elements.
 * @author  Mario Hess
 * @date    September 13, 2024
 */

use egui_sdl2_gl::{
    egui::{load::SizedTexture, Color32, Image, TextureId, Vec2},
    painter::Painter,
};

use crate::{
    cpu::Cpu,
    event_handler::EventHandler,
    ppu::{
        TILEMAP_END_0, TILEMAP_END_1, TILEMAP_HEIGHT, TILEMAP_START_0, TILEMAP_START_1, TILEMAP_WIDTH, TILETABLE_HEIGHT, TILETABLE_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE
    },
    View,
};

pub struct CentralPanel {
    pub game_background: Vec<Color32>,
    pub game_texture_id: TextureId,
    pub tiletable_texture_id: TextureId,
    pub tilemap_texture_id: TextureId,
    splash_texture_id: TextureId,
    pub splash_background: Vec<Color32>,
}

impl CentralPanel {
    pub fn new(painter: &mut Painter) -> Self {
        let mut game_background: Vec<Color32> =
            Vec::with_capacity(VIEWPORT_WIDTH * VIEWPORT_HEIGHT);
        game_background.fill(Color32::from_rgb(WHITE.r, WHITE.g, WHITE.b));

        let game_texture_id = painter.new_user_texture(
            (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            &vec![Color32::BLACK; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            false,
        );

        let tiletable_texture_id = painter.new_user_texture(
            (TILETABLE_WIDTH, TILETABLE_HEIGHT),
            &vec![Color32::BLACK; TILETABLE_WIDTH * TILETABLE_HEIGHT],
            false,
        );

        let tilemap_texture_id = painter.new_user_texture(
            (TILEMAP_WIDTH, TILEMAP_HEIGHT),
            &vec![Color32::BLACK; TILEMAP_WIDTH * TILEMAP_HEIGHT],
            false,
        );

        let img = image::load_from_memory(include_bytes!("../../images/splash.png"))
            .expect("Failed to load image");

        let image_data: Vec<u8> = img.to_rgba8().into_raw();
        let splash_background = image_data
            .chunks_exact(4)
            .map(|chunk| Color32::from_rgba_unmultiplied(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect();

        let splash_texture_id = painter.new_user_texture(
            (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            &vec![Color32::BLACK; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            false,
        );

        Self {
            game_background,
            game_texture_id,
            tiletable_texture_id,
            tilemap_texture_id,
            splash_texture_id,
            splash_background,
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
                                .map(|color| Color32::from_rgb(color.r, color.g, color.b))
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
                                .map(|color| Color32::from_rgb(color.r, color.g, color.b))
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
                                .map(|color| Color32::from_rgb(color.r, color.g, color.b))
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
                                .map(|color| Color32::from_rgb(color.r, color.g, color.b))
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
                    painter
                        .update_user_texture_data(self.splash_texture_id, &self.splash_background);
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
