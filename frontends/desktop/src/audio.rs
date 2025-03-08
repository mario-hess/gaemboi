use gaemboi::AudioSamplesObserver;

pub struct Audio;
impl Audio {
    pub fn new() -> Self {
        Self {}
    }

    fn queue_samples(&mut self, audio_samples: &(u8, u8)) {
        let (left, right) = audio_samples;
        //println!("Left: {} | Right: {}", left, right);
    }
}
impl AudioSamplesObserver for Audio {
    fn on_samples_ready(&mut self, audio_samples: &(u8, u8)) {
        self.queue_samples(audio_samples);
    }
}
