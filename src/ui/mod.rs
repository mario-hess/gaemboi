mod central_panel;
mod top_panel;

use central_panel::CentralPanel;
use egui_sdl2_gl::{
    egui::{Align, Color32, Context, FullOutput, Grid, Pos2, Rect, Stroke, Ui, Vec2},
    painter::Painter,
    sdl2::video::Window,
    EguiStateHandler,
};
use top_panel::TopPanel;

use crate::{
    apu::channel::square_channel::{SquareChannel, DUTY_TABLE},
    cpu::Cpu,
    event_handler::EventHandler,
    ppu::{
        TILEMAP_HEIGHT, TILEMAP_WIDTH, TILETABLE_HEIGHT, TILETABLE_WIDTH, VIEWPORT_HEIGHT,
        VIEWPORT_WIDTH,
    },
    State, View,
};

pub struct UIManager {
    top_panel: TopPanel,
    central_panel: CentralPanel,
    pub current_view: View,
    pub previous_view: View,
}

impl UIManager {
    pub fn new(painter: &mut Painter) -> Self {
        Self {
            top_panel: TopPanel::new(),
            central_panel: CentralPanel::new(painter),
            current_view: View::Viewport,
            previous_view: View::Viewport,
        }
    }

    pub fn draw_splash(
        &mut self,
        egui_ctx: &Context,
        state: &mut EguiStateHandler,
        painter: &mut Painter,
        window: &mut Window,
        event_handler: &mut EventHandler,
    ) {
        self.update_window_size(window, event_handler);
        self.top_panel.draw(
            egui_ctx,
            event_handler,
            &0.0,
            None,
            &mut self.current_view,
            State::Splash,
        );
        self.central_panel
            .draw(egui_ctx, event_handler, None, &self.current_view, painter);
        self.finish_frame(egui_ctx, window, state, painter);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        egui_ctx: &Context,
        state: &mut EguiStateHandler,
        painter: &mut Painter,
        window: &mut Window,
        event_handler: &mut EventHandler,
        cpu: &mut Cpu,
        fps: &f32,
    ) {
        self.update_window_size(window, event_handler);
        if event_handler.fast_forward > 1 {
            event_handler.performance_mode = false;
            event_handler.volume = 0;
            event_handler.volume_slider = false;
        } else {
            event_handler.volume_slider = true;
        }

        self.top_panel.draw(
            egui_ctx,
            event_handler,
            fps,
            Some(cpu),
            &mut self.current_view,
            State::Play,
        );
        self.central_panel.draw(
            egui_ctx,
            event_handler,
            Some(cpu),
            &self.current_view,
            painter,
        );
        self.draw_windows(egui_ctx, cpu, event_handler);
        self.finish_frame(egui_ctx, window, state, painter);
    }

    fn draw_windows(&self, egui_ctx: &Context, cpu: &mut Cpu, event_handler: &mut EventHandler) {
        egui_sdl2_gl::egui::Window::new("Square Waves")
            .open(&mut event_handler.show_square_waves)
            .min_width(280.0)
            .max_width(280.0)
            .collapsible(true)
            .show(egui_ctx, |ui| {
                ui.label("Channel 1");
                draw_square(ui, &cpu.memory_bus.apu.ch1);
                ui.separator();
                ui.label("Channel 2");
                draw_square(ui, &cpu.memory_bus.apu.ch2);
            });

        egui_sdl2_gl::egui::Window::new("Waveform")
            .open(&mut event_handler.show_waveform)
            .show(egui_ctx, |ui| {
                draw_wave(
                    ui,
                    &cpu.memory_bus.apu.ch3.wave_ram,
                    &cpu.memory_bus.apu.ch3.wave_ram_position,
                    &cpu.memory_bus.apu.ch3.volume,
                    &cpu.memory_bus.apu.ch3.frequency,
                );
            });

        egui_sdl2_gl::egui::Window::new("Color Scheme")
            .open(&mut event_handler.color_scheme_opened)
            .show(egui_ctx, |ui| {
                Grid::new("color_scheme_grid")
                    .num_columns(2)
                    .spacing([10.0, 10.0]) // Set horizontal and vertical spacing
                    .show(ui, |ui| {
                        color_picker_row(ui, "Black:", &mut event_handler.black);
                        color_picker_row(ui, "Dark:", &mut event_handler.dark);
                        color_picker_row(ui, "Light:", &mut event_handler.light);
                        color_picker_row(ui, "White:", &mut event_handler.white);

                        if ui.button("Reset").clicked() {
                            // Reset colors
                        }
                    });
            });

        fn color_picker_row(ui: &mut Ui, label: &str, color: &mut Color32) {
            ui.with_layout(
                egui_sdl2_gl::egui::Layout::left_to_right(Align::Center),
                |ui| {
                    ui.label(label);
                },
            );
            ui.color_edit_button_srgba(color);
            ui.end_row();
        }

        egui_sdl2_gl::egui::Window::new("CPU Status")
            .open(&mut event_handler.cpu_status_opened)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("AF");
                    ui.label(format!("0x{:02X}", cpu.registers.get_a()));
                    let f: u8 = (&cpu.registers.flags).into();
                    ui.label(format!("0x{:02X}", f));
                });

                ui.horizontal(|ui| {
                    ui.label("BC");
                    ui.label(format!("0x{:02X}", cpu.registers.get_b()));
                    ui.label(format!("0x{:02X}", cpu.registers.get_c()));
                });

                ui.horizontal(|ui| {
                    ui.label("DE");
                    ui.label(format!("0x{:02X}", cpu.registers.get_d()));
                    ui.label(format!("0x{:02X}", cpu.registers.get_e()));
                });

                ui.horizontal(|ui| {
                    ui.label("HL");
                    ui.label(format!("0x{:02X}", cpu.registers.get_h()));
                    ui.label(format!("0x{:02X}", cpu.registers.get_l()));
                });

                ui.horizontal(|ui| {
                    ui.label("SP");
                    ui.label(format!("0x{:02X}", cpu.stack_pointer));
                });

                ui.horizontal(|ui| {
                    ui.label("PC");
                    ui.label(format!("0x{:02X}", cpu.program_counter.get()));
                });
            });
    }

    #[allow(unused_variables)]
    fn finish_frame(
        &self,
        egui_ctx: &Context,
        window: &mut Window,
        egui_state: &mut EguiStateHandler,
        painter: &mut Painter,
    ) {
        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_frame();

        // Process output
        egui_state.process_output(window, &platform_output);

        // Paint jobs
        let paint_jobs = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_jobs(None, textures_delta, paint_jobs);

        // Swap the window
        window.gl_swap_window();
    }

    /*
    fn draw_menu_button(&self, ui: &mut Ui, label: &str, items: Vec<MenuItem>) {
        ui.menu_button(label, |ui| {
            for item in items {
                if ui.button(item.label).clicked() {
                    item.action();
                    ui.close_menu();
                }
            }
        });
    }
    */
    pub fn update_window_size(&mut self, window: &mut Window, event_handler: &mut EventHandler) {
        if self.current_view != self.previous_view
            || event_handler.window_scale != event_handler.previous_scale
        {
            match self.current_view {
                View::Viewport => {
                    window
                        .set_size(
                            VIEWPORT_WIDTH as u32 * event_handler.window_scale,
                            VIEWPORT_HEIGHT as u32 * event_handler.window_scale
                                + self.top_panel.menu_bar_height as u32,
                        )
                        .unwrap();
                }
                View::Tiletable => {
                    window
                        .set_size(
                            TILETABLE_WIDTH as u32 * event_handler.window_scale,
                            TILETABLE_HEIGHT as u32 * event_handler.window_scale
                                + self.top_panel.menu_bar_height as u32,
                        )
                        .unwrap();
                }
                _ => {
                    window
                        .set_size(
                            TILEMAP_WIDTH as u32 * event_handler.window_scale,
                            TILEMAP_HEIGHT as u32 * event_handler.window_scale
                                + self.top_panel.menu_bar_height as u32,
                        )
                        .unwrap();
                }
            }

            window.set_position(
                egui_sdl2_gl::sdl2::video::WindowPos::Centered,
                egui_sdl2_gl::sdl2::video::WindowPos::Centered,
            );

            event_handler.previous_scale = event_handler.window_scale;
            self.previous_view = self.current_view;
        }
    }
}

pub fn draw_wave(
    ui: &mut egui_sdl2_gl::egui::Ui,
    wave_ram: &[u8; 32],
    wave_ram_position: &u8,
    volume: &u8,
    frequency: &u16,
) {
    let (response, painter) =
        ui.allocate_painter(Vec2::new(300.0, 100.0), egui_sdl2_gl::egui::Sense::hover());
    let rect = response.rect;

    // Draw grid
    for i in 0..=32 {
        let x = egui_sdl2_gl::egui::remap(i as f32, 0.0..=16.0, rect.left()..=rect.right());
        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(0.5, Color32::DARK_GRAY),
        );
    }
    for i in 0..=16 {
        let y = egui_sdl2_gl::egui::remap(i as f32, 0.0..=16.0, rect.bottom()..=rect.top());
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(0.5, Color32::DARK_GRAY),
        );
    }

    // Draw wave
    let mut points = Vec::with_capacity(16);
    for i in (0..32).step_by(2) {
        let sample = (wave_ram[i] as u16) | ((wave_ram[i + 1] as u16) << 4);
        let x = egui_sdl2_gl::egui::remap(i as f32, 0.0..=15.0, rect.left()..=rect.right());
        let y = egui_sdl2_gl::egui::remap(
            sample as f32,
            0.0..=255.0,
            rect.bottom()..=(rect.top() + 1.0),
        );
        points.push(Pos2::new(x, y));
    }
    painter.add(egui_sdl2_gl::egui::Shape::line(
        points,
        Stroke::new(1.0, Color32::GREEN),
    ));

    // Highlight current position
    let current_x = egui_sdl2_gl::egui::remap(
        *wave_ram_position as f32,
        0.0..=31.0,
        rect.left()..=rect.right(),
    );
    painter.line_segment(
        [
            Pos2::new(current_x, rect.top()),
            Pos2::new(current_x, rect.bottom()),
        ],
        Stroke::new(2.0, Color32::RED),
    );

    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            // Display channel information
            ui.label(format!("Position: {:02}", wave_ram_position));
            ui.separator();
            ui.label(format!("Frequency: {:04}", frequency));
            ui.separator();
            ui.label(format!("Volume: {:02}", volume));
        });
    });
}

pub fn draw_square(ui: &mut egui_sdl2_gl::egui::Ui, square_channel: &SquareChannel) {
    ui.vertical_centered(|ui| {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(240.0, 40.0), egui_sdl2_gl::egui::Sense::hover());
        let rect = response.rect;

        // Draw grid
        for i in 0..=64 {
            let x = egui_sdl2_gl::egui::remap(i as f32, 0.0..=64.0, rect.left()..=rect.right());
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(0.5, Color32::DARK_GRAY),
            );
        }
        for i in 0..=16 {
            let y = egui_sdl2_gl::egui::remap(i as f32, 0.0..=16.0, rect.bottom()..=rect.top());
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(0.5, Color32::DARK_GRAY),
            );
        }

        let points = generate_square_wave_points(
            rect,
            square_channel.wave_duty as usize,
            square_channel.sequence as usize,
            square_channel.volume_envelope.volume,
        );

        painter.add(egui_sdl2_gl::egui::Shape::line(
            points,
            Stroke::new(1.0, Color32::GREEN),
        ));
    });

    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            // Display channel information
            ui.label(format!(
                "Duty: {}/8",
                DUTY_TABLE[square_channel.wave_duty as usize]
                    .iter()
                    .sum::<u8>()
            ));
            ui.separator();
            ui.label(format!("Frequency: {:04}", square_channel.frequency));
            ui.separator();
            ui.label(format!(
                "Volume: {:02}",
                square_channel.volume_envelope.volume
            ));
        });
    });
}

fn generate_square_wave_points(
    rect: Rect,
    duty_cycle: usize,
    sequence: usize,
    volume: u8,
) -> Vec<Pos2> {
    let mut points = Vec::new();
    let amplitude = (rect.height() / 2.0) - 1.0;
    let mid_y = (rect.top() + rect.bottom()) / 2.0;

    let samples = 64; // Number of samples to generate
    let duty_pattern = DUTY_TABLE[duty_cycle];

    let y_high = mid_y - (volume as f32 / 15.0) * amplitude;
    let y_low = mid_y + (volume as f32 / 15.0) * amplitude;

    let mut last_y = if duty_pattern[sequence % 8] == 1 {
        y_high
    } else {
        y_low
    };
    let mut last_x = rect.left();

    for i in 0..=samples {
        let x =
            egui_sdl2_gl::egui::remap(i as f32, 0.0..=samples as f32, rect.left()..=rect.right());
        let cycle_position = (i + sequence * (samples / 8)) % 8;
        let is_high = duty_pattern[cycle_position] == 1;
        let y = if is_high { y_high } else { y_low };

        if y != last_y {
            // Transition point: add two points for the vertical line
            if last_x != x {
                points.push(Pos2::new(last_x, last_y));
            }
            points.push(Pos2::new(x, last_y));
            points.push(Pos2::new(x, y));
        } else if points.is_empty() {
            // Include the first point
            points.push(Pos2::new(x, y));
        } else {
            points.push(Pos2::new(x, y));
        }

        last_y = y;
        last_x = x;
    }

    points
}
