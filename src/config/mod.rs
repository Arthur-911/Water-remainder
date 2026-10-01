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

            if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                let _ = fs::create_dir_all(parent);
            }

            match fs::write(path, toml_content) {
                Ok(_) => {
                    println!(
                        "{} Created default config at {}",
                        "⚙ [Config]".bright_blue().bold(),
                        path.display().to_string().cyan()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "{} Could not create default config at {}: {}. Using in-memory defaults.",
                        "⚠ [Config Warning]".bright_yellow().bold(),
                        path.display().to_string().cyan(),
                        e
                    );
                }
            }
        }

        AppConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.water_interval_mins, 60);
        assert_eq!(cfg.break_interval_mins, 60);
        assert_eq!(cfg.stagger_minutes, 0);
        assert!(cfg.sound_enabled);
        assert!(cfg.popup_enabled);
        assert!(cfg.voice_enabled);
    }

    #[test]
    fn test_deserialize_partial_toml() {
        let toml_str = r#"
            water_interval_mins = 45
            break_interval_mins = 50
            sound_enabled = false
        "#;
        let cfg: AppConfig = toml::from_str(toml_str).expect("Should deserialize");
        assert_eq!(cfg.water_interval_mins, 45);
        assert_eq!(cfg.break_interval_mins, 50);
        assert!(!cfg.sound_enabled);
        // Defaults should be populated
        assert!(cfg.popup_enabled);
        assert!(cfg.voice_enabled);
        assert_eq!(cfg.stagger_minutes, 0);
    }

    #[test]
    fn test_generate_and_parse_default_toml() {
        let default_cfg = AppConfig::default();
        let toml_str = defaults::generate_default_toml(&default_cfg);
        let parsed: AppConfig = toml::from_str(&toml_str).expect("Default TOML should parse cleanly");
        assert_eq!(parsed.water_interval_mins, default_cfg.water_interval_mins);
        assert_eq!(parsed.break_interval_mins, default_cfg.break_interval_mins);
    }
}
