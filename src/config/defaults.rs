use super::AppConfig;

pub fn default_water_interval() -> u64 {
    60
}

pub fn default_break_interval() -> u64 {
    60
}

pub fn default_stagger_minutes() -> u64 {
    0
}

pub fn default_true() -> bool {
    true
}

/// Generates the initial well-commented TOML configuration text.
pub fn generate_default_toml(default_cfg: &AppConfig) -> String {
    format!(
        r#"# Health & Work Reminder Configuration
# Intervals are in minutes. Default is 60 minutes (1 hour) for both.

# Interval in minutes between water reminders
water_interval_mins = {}

# Interval in minutes between break reminders
break_interval_mins = {}

# Optional stagger offset in minutes (e.g., 30 will stagger breaks 30 mins after water)
stagger_minutes = {}

# Play sound when notification appears
sound_enabled = {}

# Show on-screen pop-up dialog box (in addition to toast notification)
popup_enabled = {}

# Water reminder title and custom message (leave empty to use rotating health tips)
water_title = "{}"
water_message = "{}"

# Optional script/command to execute when water reminder fires (e.g. "scripts/sample_water_hook.bat" or "powershell ...")
water_script = "{}"

# Break reminder title and custom message (leave empty to use rotating health tips)
break_title = "{}"
break_message = "{}"

# Optional script/command to execute when break reminder fires
break_script = "{}"
"#,
        default_cfg.water_interval_mins,
        default_cfg.break_interval_mins,
        default_cfg.stagger_minutes,
        default_cfg.sound_enabled,
        default_cfg.popup_enabled,
        default_cfg.water_title,
        default_cfg.water_message,
        default_cfg.water_script,
        default_cfg.break_title,
        default_cfg.break_message,
        default_cfg.break_script
    )
}
