use crate::constants::DEFAULT_CONFIG;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Default)]
pub struct RumosConfig {
    pub device_font_color: Option<String>,
    pub device_font_style: Option<String>,
    pub percent_font_color: Option<String>,
    pub percent_font_style: Option<String>,
}

impl RumosConfig {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if !config_path.exists() {
            if let Err(e) = Self::create_default_config(&config_path) {
                eprintln!(
                    "Сould not create default config at {:?}: {}",
                    config_path, e
                );
                return Self::default();
            }
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str::<Self>(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    fn create_default_config(path: &PathBuf) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, DEFAULT_CONFIG)?;

        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let mut path =
            dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        path.push("rumos");
        path.push("config.toml");
        path
    }
}
