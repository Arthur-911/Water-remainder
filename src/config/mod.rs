pub mod defaults;

use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "defaults::default_water_interval")]
    pub water_interval_mins: u64,

    #[serde(default = "defaults::default_break_interval")]
    pub break_interval_mins: u64,

    #[serde(default = "defaults::default_stagger_minutes")]
    pub stagger_minutes: u64,

    #[serde(default = "defaults::default_true")]
    pub sound_enabled: bool,

    #[serde(default = "defaults::default_true")]
    pub popup_enabled: bool,

    #[serde(default = "defaults::default_true")]
    pub voice_enabled: bool,

    #[serde(default = "defaults::default_true")]
    pub voice_on_water: bool,

    #[serde(default = "defaults::default_true")]
    pub voice_on_break: bool,

    #[serde(default = "defaults::default_voice_file")]
    pub voice_file: String,

    #[serde(default)]
    pub water_title: String,

    #[serde(default)]
    pub water_message: String,

    #[serde(default)]
    pub water_script: String,

    #[serde(default)]
    pub break_title: String,

    #[serde(default)]
    pub break_message: String,

    #[serde(default)]
    pub break_script: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            water_interval_mins: 60,
            break_interval_mins: 60,
            stagger_minutes: 0,
            sound_enabled: true,
            popup_enabled: true,
            voice_enabled: true,
            voice_on_water: true,
            voice_on_break: true,
            voice_file: "assets/voice_reminder.mp3".to_string(),
            water_title: "💧 Time to Drink Water!".to_string(),
            water_message: String::new(),
            water_script: String::new(),
            break_title: "🧘 Time to Take a Break!".to_string(),
            break_message: String::new(),
            break_script: String::new(),
        }
    }
}

impl AppConfig {
    /// Loads the configuration from disk, or writes a default template if not found.
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => match toml::from_str::<AppConfig>(&content) {
                    Ok(cfg) => {
                        println!(
                            "{} Loaded configuration from {}",
                            "⚙ [Config]".bright_blue().bold(),
                            path.display().to_string().cyan()
                        );
                        return cfg;
                    }
                    Err(e) => {
                        eprintln!(
                            "{} Failed to parse config file: {}. Using defaults.",
                            "⚠ [Config Warning]".bright_yellow().bold(),
                            e
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "{} Could not read config file: {}. Using defaults.",
                        "⚠ [Config Warning]".bright_yellow().bold(),
                        e
                    );
                }
            }
        } else {
            let default_cfg = AppConfig::default();
            let toml_content = defaults::generate_default_toml(&default_cfg);

            let _ = fs::write(path, toml_content);
            println!(
                "{} Created default config at {}",
                "⚙ [Config]".bright_blue().bold(),
                path.display().to_string().cyan()
            );
        }

        AppConfig::default()
    }
}
