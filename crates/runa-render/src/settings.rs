#[derive(Clone)]
pub struct RenderSettings {
    pub output_widht: u32,
    pub output_height: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            output_widht: 1280,
            output_height: 720,
        }
    }
}
