/*
 * @file    ui/top_panel.rs
 * @brief   Top panel of the user interface.
 * @author  Mario Hess
 * @date    September 13, 2024
 */

use egui_sdl2_gl::egui::{menu, Context, TopBottomPanel, Ui};
use rfd::FileDialog;

use crate::{event_handler::EventHandler, State, View};

pub struct TopPanel {
    pub menu_bar_height: f32,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            menu_bar_height: 20.0,
        }
    }

    pub fn draw(
        &mut self,
        egui_ctx: &Context,
        event_handler: &mut EventHandler,
        fps: &f32,
        current_view: &mut View,
        current_state: State,
    ) {
        TopBottomPanel::top("top_panel")
            .exact_height(self.menu_bar_height)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open").clicked() {
                                ui.close_menu();
                                event_handler.volume = 0;

                                let file = FileDialog::new()
                                    .add_filter("gb", &["gb"])
                                    .set_directory("../")
                                    .pick_file();

                                if let Some(file) = file {
                                    event_handler.file_path =
                                        Some(file.into_os_string().into_string().unwrap());
                                }

                                event_handler.volume = event_handler.last_volume;
                            }
                        });

                        ui.menu_button("View", |ui| {
                            ui.menu_button("Window Scale                >", |ui| {
                                if ui
                                    .radio_value(&mut event_handler.window_scale, 1, "1x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };

                                if ui
                                    .radio_value(&mut event_handler.window_scale, 2, "2x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };

                                if ui
                                    .radio_value(&mut event_handler.window_scale, 3, "3x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };

                                if ui
                                    .radio_value(&mut event_handler.window_scale, 4, "4x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };

                                if ui
                                    .radio_value(&mut event_handler.window_scale, 5, "5x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };

                                if ui
                                    .radio_value(&mut event_handler.window_scale, 6, "6x")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                            });

                            match current_state {
                                State::Splash => ui.set_enabled(false),
                                State::Play => {}
                            }

                            ui.menu_button("Video RAM                      >", |ui| {
                                if ui
                                    .radio_value(current_view, View::Viewport, "Viewport")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                                if ui
                                    .radio_value(current_view, View::Tiletable, "Tiletable")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                                if ui
                                    .radio_value(current_view, View::Tilemap0, "Tilemap0")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                                if ui
                                    .radio_value(current_view, View::Tilemap1, "Tilemap1")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                            });

                            ui.menu_button("Audio Visualizer            >", |ui| {
                                if ui
                                    .checkbox(&mut event_handler.show_square_waves, "Square Waves")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                                if ui
                                    .checkbox(&mut event_handler.show_waveform, "Waveform")
                                    .clicked()
                                {
                                    ui.close_menu();
                                };
                            });

                            if ui.button("CPU Status").clicked() {
                                event_handler.cpu_status_opened = true;
                                ui.close_menu();
                            };
                        });

                        ui.menu_button("Settings", |ui| {
                            match current_state {
                                State::Splash => ui.set_enabled(false),
                                State::Play => {}
                            }

                            if ui.button("Keybindings").clicked() {
                                event_handler.keybindings_opened =
                                    !event_handler.keybindings_opened;
                                ui.close_menu();
                            }

                            if ui.button("Color Scheme").clicked() {
                                event_handler.color_scheme_opened =
                                    !event_handler.color_scheme_opened;
                                ui.close_menu();
                            }

                            ui.add_enabled(
                                *event_handler.fast_forward.borrow() == 1,
                                |ui: &mut Ui| {
                                    ui.checkbox(
                                        &mut event_handler.performance_mode,
                                        "Performance Mode",
                                    )
                                },
                            );

                            ui.menu_button("Fast Forward                   >", |ui| {
                                ui.add(
                                    egui_sdl2_gl::egui::Slider::new(
                                        &mut *event_handler.fast_forward.as_ref().borrow_mut(),
                                        1..=4,
                                    )
                                    .prefix("Speed: ")
                                    .suffix("x"),
                                )
                            });
                        });
                        ui.menu_button("Help", |ui| {
                            if ui.button("About").clicked() {
                                event_handler.about_opened = !event_handler.about_opened;
                                ui.close_menu();
                            }

                            if ui.button("Bug report").clicked() {
                                event_handler.bug_report_opened = !event_handler.bug_report_opened;
                                ui.close_menu();
                            }
                        });
                    });

                    // Tabs on the right
                    ui.with_layout(
                        egui_sdl2_gl::egui::Layout::right_to_left(
                            egui_sdl2_gl::egui::Align::Center,
                        ),
                        |ui| {
                            ui.add_space(6.0);
                            ui.label(format!("FPS: {:.2}", fps));

                            ui.style_mut().spacing.slider_width = 75.0;
                            ui.separator();
                            ui.add_enabled(event_handler.volume_slider, |ui: &mut Ui| {
                                ui.add(
                                    egui_sdl2_gl::egui::Slider::new(
                                        &mut event_handler.volume,
                                        0..=100,
                                    )
                                    .show_value(false)
                                    .prefix("VOL: ")
                                    .suffix("%"),
                                )
                            });

                            ui.add_enabled_ui(event_handler.volume_slider, |ui| {
                                let label = if event_handler.volume == 0 {
                                    "Muted".to_string()
                                } else {
                                    format!("VOL: {}%", event_handler.volume)
                                };
                                if ui.button(label).clicked() {
                                    if event_handler.volume != 0 {
                                        event_handler.last_volume = event_handler.volume;
                                        event_handler.volume = 0;
                                    } else {
                                        event_handler.volume = event_handler.last_volume;
                                    }
                                }
                            });
                        },
                    );
                });
            });
    }
}
