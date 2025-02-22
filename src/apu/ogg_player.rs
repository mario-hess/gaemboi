use egui_sdl2_gl::sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};
use lewton::inside_ogg::OggStreamReader;
use lewton::samples::InterleavedSamples;
use std::{
    error::Error,
    io::{BufReader, Cursor},
    sync::{Arc, Mutex},
};

pub struct OggPlayer<'a> {
    data: Arc<Mutex<Vec<i16>>>,
    position: Arc<Mutex<usize>>,
    volume: &'a u8,
}


impl AudioCallback for OggPlayer<'_> {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        let data = self.data.lock().unwrap();
        let mut pos = self.position.lock().unwrap();
        let volume = *self.volume as f32 / 255.0;

        let len = data.len();
        for x in out.iter_mut() {
            if *pos < len {
                *x = (data[*pos] as f32 * volume) as i16;
                *pos += 1;
            } else {
                *pos = 0; // Loop back to the start
                *x = (data[*pos] as f32 * volume) as i16;
                *pos += 1;
            }
        }
    }
}

pub fn create_audio_theme<'a>(
    audio_subsystem: &AudioSubsystem,
    volume: &'a u8,
) -> Result<AudioDevice<OggPlayer<'a>>, Box<dyn Error>> {
    let file_bytes = include_bytes!("../../audio/splash.ogg");
    let file = BufReader::new(Cursor::new(file_bytes));
    let mut ogg_stream = OggStreamReader::new(file)?;

    let mut audio_data = Vec::new();
    while let Some(packet) = ogg_stream.read_dec_packet_generic::<InterleavedSamples<i16>>()? {
        audio_data.extend(packet.samples);
    }

    if audio_data.is_empty() {
        return Err("No audio data found in OGG file".into());
    }

    let data = Arc::new(Mutex::new(audio_data));
    let position = Arc::new(Mutex::new(0));

    // Set up audio playback
    let desired_spec = AudioSpecDesired {
        freq: Some(ogg_stream.ident_hdr.audio_sample_rate as i32),
        channels: Some(ogg_stream.ident_hdr.audio_channels),
        samples: Some(1024), // Buffer size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |_spec| OggPlayer {
            data: Arc::clone(&data),
            position: Arc::clone(&position),
            volume,
        })
        .map_err(|e| format!("Failed to open audio device: {}", e))?;

    Ok(device)
}
