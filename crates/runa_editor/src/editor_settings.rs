use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = ".runa_editor/settings.ron";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub external_editor_executable: String,
    pub external_editor_args: String,
    pub ui_scale: f32,
    pub show_hidden_files: bool,
    pub content_icon_size: f32,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            external_editor_executable: "zed".to_string(),
            external_editor_args: "{file}".to_string(),
            ui_scale: 1.0,
            show_hidden_files: false,
            content_icon_size: 48.0,
        }
    }
}

impl EditorSettings {
    pub fn load() -> Self {
        let path = settings_path();
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };
        ron::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = settings_path();
        let Some(parent) = path.parent() else {
            return Err("Invalid settings path".to_string());
        };
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let content = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|error| error.to_string())
    }
}

fn settings_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join(SETTINGS_PATH)
}
