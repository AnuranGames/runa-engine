use std::sync::Arc;

pub struct AudioSource {
    pub sound_data: Arc<Vec<u8>>,
    /// min = 0.0, max = 1.0
    pub volume: f32,
    pub is_3d: bool,
    pub looped: bool,
}

impl AudioSource {
    pub fn new(sound_data: Vec<u8>) -> Self {
        Self {
            sound_data: Arc::new(sound_data),
            volume: 1.0,
            is_3d: false,
            looped: false,
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    pub fn with_loop(mut self, looped: bool) -> Self {
        self.looped = looped;
        self
    }
}
