pub const SAMPLING_FREQUENCY: u16 = 44100;
pub const FRAME_DURATION_MILLIS: f64 = 16.742706458499015;
pub const FRAME_DURATION_MICROS: u64 = (FRAME_DURATION_MILLIS * 1_000.0) as u64;
pub const FRAME_DURATION: std::time::Duration =
    std::time::Duration::from_micros(FRAME_DURATION_MICROS);
pub const FPS: f32 = 59.7275;
