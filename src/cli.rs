use clap::Parser;
use std::path::PathBuf;
use crate::config::AppConfig;

#[derive(Parser, Debug)]
#[command(
    name = "reminder_runner",
    version = "1.0.0",
    about = "A background script runner and health reminder for hydration and desk breaks",
    author = "Health & Productivity Tool"
)]
pub struct Cli {
    /// Water reminder interval in minutes (default: 60)
    #[arg(short = 'w', long = "water")]
    pub water: Option<u64>,

    /// Break reminder interval in minutes (default: 60)
    #[arg(short = 'b', long = "break")]
    pub break_interval: Option<u64>,

    /// Stagger break reminder by X minutes relative to water
    #[arg(short = 's', long = "stagger")]
    pub stagger: Option<u64>,

    /// Path to TOML config file
    #[arg(short = 'c', long = "config", default_value = "reminder_config.toml")]
    pub config: PathBuf,

    /// Script or command to run when water reminder fires
    #[arg(long = "water-script")]
    pub water_script: Option<String>,

    /// Script or command to run when break reminder fires
    #[arg(long = "break-script")]
    pub break_script: Option<String>,

    /// Run in test mode (intervals shortened to 10s and 15s)
    #[arg(long = "test")]
    pub test: bool,

    /// Trigger first reminder immediately upon startup
    #[arg(long = "now")]
    pub now: bool,

    /// Enable notification sound
    #[arg(long = "sound", overrides_with = "no_sound")]
    pub sound: bool,

    /// Disable notification sound
    #[arg(long = "no-sound")]
    pub no_sound: bool,

    /// Enable on-screen pop-up dialog box
    #[arg(long = "popup", overrides_with = "no_popup")]
    pub popup: bool,

    /// Disable on-screen pop-up dialog box (toast notification only)
    #[arg(long = "no-popup")]
    pub no_popup: bool,

    /// Enable AI voice reminder message
    #[arg(long = "voice", overrides_with = "no_voice")]
    pub voice: bool,

    /// Disable AI voice reminder message
    #[arg(long = "no-voice")]
    pub no_voice: bool,

    /// Custom voice audio file (.mp3 or .wav)
    #[arg(long = "voice-file")]
    pub voice_file: Option<String>,

    /// Test-play the AI voice reminder immediately and exit
    #[arg(long = "test-voice")]
    pub test_voice: bool,

    /// Run quietly (suppress countdown, only log events)
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,
}

impl Cli {
    /// Applies any command-line overrides to the given configuration.
    pub fn apply_to_config(&self, cfg: &mut AppConfig) {
        if let Some(w) = self.water {
            cfg.water_interval_mins = w;
        }
        if let Some(b) = self.break_interval {
            cfg.break_interval_mins = b;
        }
        if let Some(s) = self.stagger {
            cfg.stagger_minutes = s;
        }
        if let Some(ref ws) = self.water_script {
            cfg.water_script = ws.clone();
        }
        if let Some(ref bs) = self.break_script {
            cfg.break_script = bs.clone();
        }
        if self.sound {
            cfg.sound_enabled = true;
        } else if self.no_sound {
            cfg.sound_enabled = false;
        }
        if self.popup {
            cfg.popup_enabled = true;
        } else if self.no_popup {
            cfg.popup_enabled = false;
        }
        if self.voice {
            cfg.voice_enabled = true;
        } else if self.no_voice {
            cfg.voice_enabled = false;
        }
        if let Some(ref vf) = self.voice_file {
            cfg.voice_file = vf.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_config_intervals() {
        let mut cfg = AppConfig::default();
        let cli = Cli {
            water: Some(45),
            break_interval: Some(30),
            stagger: Some(15),
            config: PathBuf::from("reminder_config.toml"),
            water_script: Some("test_water.bat".to_string()),
            break_script: Some("test_break.bat".to_string()),
            test: false,
            now: false,
            sound: false,
            no_sound: true,
            popup: false,
            no_popup: true,
            voice: false,
            no_voice: true,
            voice_file: Some("assets/custom.mp3".to_string()),
            test_voice: false,
            quiet: false,
        };

        cli.apply_to_config(&mut cfg);
        assert_eq!(cfg.water_interval_mins, 45);
        assert_eq!(cfg.break_interval_mins, 30);
        assert_eq!(cfg.stagger_minutes, 15);
        assert_eq!(cfg.water_script, "test_water.bat");
        assert_eq!(cfg.break_script, "test_break.bat");
        assert!(!cfg.sound_enabled);
        assert!(!cfg.popup_enabled);
        assert!(!cfg.voice_enabled);
        assert_eq!(cfg.voice_file, "assets/custom.mp3");
    }

    #[test]
    fn test_apply_to_config_enable_flags() {
        let mut cfg = AppConfig::default();
        cfg.sound_enabled = false;
        cfg.popup_enabled = false;
        cfg.voice_enabled = false;

        let cli = Cli {
            water: None,
            break_interval: None,
            stagger: None,
            config: PathBuf::from("reminder_config.toml"),
            water_script: None,
            break_script: None,
            test: false,
            now: false,
            sound: true,
            no_sound: false,
            popup: true,
            no_popup: false,
            voice: true,
            no_voice: false,
            voice_file: None,
            test_voice: false,
            quiet: false,
        };

        cli.apply_to_config(&mut cfg);
        assert!(cfg.sound_enabled);
        assert!(cfg.popup_enabled);
        assert!(cfg.voice_enabled);
    }
}
