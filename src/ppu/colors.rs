use egui_sdl2_gl::egui::Color32;

pub struct Colors {
    pub black: Color32,
    pub dark: Color32,
    pub light: Color32,
    pub white: Color32,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            black: Color32::from_rgb(8, 24, 32),
            dark: Color32::from_rgb(52, 104, 86),
            light: Color32::from_rgb(136, 192, 112),
            white: Color32::from_rgb(224, 248, 208),
        }
    }
}
