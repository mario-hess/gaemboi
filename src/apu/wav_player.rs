use std::{error::Error, sync::{Arc, Mutex}};

use egui_sdl2_gl::sdl2::{audio::{AudioCallback, AudioDevice, AudioSpecDesired}, AudioSubsystem};

pub struct WavPlayer {
    data: Arc<Mutex<Vec<i16>>>,
    position: Arc<Mutex<usize>>,
}

impl AudioCallback for WavPlayer {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        let data = self.data.lock().unwrap();
        let mut pos = self.position.lock().unwrap();

        let len = data.len();
        for x in out.iter_mut() {
            if *pos < len {
                *x = data[*pos] / 10;
                *pos += 1;
            } else {
                *pos = 0; // Loop back to the start
                *x = data[*pos] / 10;
                *pos += 1;
            }
        }
    }
}

pub fn create_audio_theme(
    audio_subsystem: &AudioSubsystem,
) -> Result<AudioDevice<WavPlayer>, Box<dyn Error>> {
    let mut reader = hound::WavReader::open("./audio/splash.wav")
        .map_err(|e| format!("Failed to open WAV file: {}", e))?;
    let spec = reader.spec();

    let mut audio_data = Vec::new();
    match spec.bits_per_sample {
        16 => {
            // Directly load 16-bit samples
            for sample in reader.samples::<i16>() {
                audio_data.push(sample.map_err(|e| format!("Error reading WAV samples: {}", e))?);
            }
        }
        24 => {
            // Convert 24-bit samples to i16
            for sample in reader.samples::<i32>() {
                let sample = sample.map_err(|e| format!("Error reading WAV samples: {}", e))?;
                audio_data.push((sample >> 8) as i16); // Scale down 24-bit to 16-bit
            }
        }
        32 => {
            // Convert 32-bit samples to i16
            for sample in reader.samples::<i32>() {
                let sample = sample.map_err(|e| format!("Error reading WAV samples: {}", e))?;
                audio_data.push((sample >> 16) as i16); // Scale down 32-bit to 16-bit
            }
        }
        _ => {}
    }

    if audio_data.is_empty() {
        // Handle error
    }

    let data = Arc::new(Mutex::new(audio_data));
    let position = Arc::new(Mutex::new(0));

    // Set up audio playback
    let desired_spec = AudioSpecDesired {
        freq: Some(spec.sample_rate as i32),
        channels: Some(spec.channels as u8),
        samples: Some(1024), // Buffer size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |_spec| WavPlayer {
            data: Arc::clone(&data),
            position: Arc::clone(&position),
        })
        .map_err(|e| format!("Failed to open audio device: {}", e))?;

    Ok(device)
}
