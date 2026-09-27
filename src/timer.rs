use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use colored::*;

use crate::config::AppConfig;
use crate::notification::Notifier;
use crate::runner::ScriptRunner;
use crate::tips::Tips;
use crate::ui;

pub struct ReminderRunner {
    water_interval: Duration,
    break_interval: Duration,
    next_water: Instant,
    next_break: Instant,
    water_count: usize,
    break_count: usize,
    water_title: String,
    water_message: String,
    water_script: String,
    break_title: String,
    break_message: String,
    break_script: String,
    sound_enabled: bool,
    popup_enabled: bool,
    voice_enabled: bool,
    voice_on_water: bool,
    voice_on_break: bool,
    voice_file: String,
    quiet: bool,
}

impl ReminderRunner {
    pub fn new(
        cfg: &AppConfig,
        test_mode: bool,
        fire_immediately: bool,
        quiet: bool,
    ) -> Self {
        let (water_interval, break_interval, stagger) = if test_mode {
            println!(
                "{} Running with short test intervals (Water: 10s, Break: 15s)",
                "⚡ [Test Mode]".bright_magenta().bold()
            );
            (
                Duration::from_secs(10),
                Duration::from_secs(15),
                Duration::from_secs(5),
            )
        } else {
            (
                Duration::from_secs(cfg.water_interval_mins * 60),
                Duration::from_secs(cfg.break_interval_mins * 60),
                Duration::from_secs(cfg.stagger_minutes * 60),
            )
        };

        let now = Instant::now();
        let next_water = if fire_immediately { now } else { now + water_interval };
        let next_break = if fire_immediately {
            now
        } else if stagger.as_secs() > 0 && stagger < break_interval {
            now + stagger
        } else {
            now + break_interval
        };

        Self {
            water_interval,
            break_interval,
            next_water,
            next_break,
            water_count: 0,
            break_count: 0,
            water_title: if cfg.water_title.is_empty() {
                "💧 Time to Drink Water!".to_string()
            } else {
                cfg.water_title.clone()
            },
            water_message: cfg.water_message.clone(),
            water_script: cfg.water_script.clone(),
            break_title: if cfg.break_title.is_empty() {
                "🧘 Time for a Break!".to_string()
            } else {
                cfg.break_title.clone()
            },
            break_message: cfg.break_message.clone(),
            break_script: cfg.break_script.clone(),
            sound_enabled: cfg.sound_enabled,
            popup_enabled: cfg.popup_enabled,
            voice_enabled: cfg.voice_enabled,
            voice_on_water: cfg.voice_on_water,
            voice_on_break: cfg.voice_on_break,
            voice_file: cfg.voice_file.clone(),
            quiet,
        }
    }

    /// Runs the main timing loop until the `running` flag becomes false.
    pub fn run(&mut self, running: Arc<AtomicBool>) {
        ui::print_status(
            self.water_interval,
            self.break_interval,
            self.popup_enabled,
            self.voice_enabled,
            &self.water_script,
            &self.break_script,
        );

        let mut last_status_update = Instant::now() - Duration::from_secs(10);

        while running.load(Ordering::SeqCst) {
            let now = Instant::now();

            if now >= self.next_water {
                self.trigger_water_reminder();
                self.next_water = Instant::now() + self.water_interval;
            }

            if now >= self.next_break {
                self.trigger_break_reminder();
                self.next_break = Instant::now() + self.break_interval;
            }

            if !self.quiet && now.duration_since(last_status_update) >= Duration::from_millis(500) {
                last_status_update = now;
                ui::render_countdown(
                    self.next_water,
                    self.next_break,
                    self.water_count,
                    self.break_count,
                );
            }

            std::thread::sleep(Duration::from_millis(250));
        }

        ui::print_summary(self.water_count, self.break_count);
    }

    fn trigger_water_reminder(&mut self) {
        self.water_count += 1;
        let tip = if self.water_message.is_empty() {
            Tips::get_water_tip(self.water_count)
        } else {
            &self.water_message
        };

        if !self.quiet {
            ui::clear_line();
        }

        let voice_active = self.voice_enabled && self.voice_on_water;
        let voice_arg = if self.voice_file.is_empty() {
            None
        } else {
            Some(self.voice_file.as_str())
        };

        let _ = Notifier::show_water_reminder(
            &self.water_title,
            tip,
            self.sound_enabled,
            self.popup_enabled,
            voice_active,
            voice_arg,
        );

        if !self.water_script.is_empty() {
            let _ = ScriptRunner::execute(&self.water_script, "water", self.water_count);
        }
    }

    fn trigger_break_reminder(&mut self) {
        self.break_count += 1;
        let tip = if self.break_message.is_empty() {
            Tips::get_break_tip(self.break_count)
        } else {
            &self.break_message
        };

        if !self.quiet {
            ui::clear_line();
        }

        let voice_active = self.voice_enabled && self.voice_on_break;
        let voice_arg = if self.voice_file.is_empty() {
            None
        } else {
            Some(self.voice_file.as_str())
        };

        let _ = Notifier::show_break_reminder(
            &self.break_title,
            tip,
            self.sound_enabled,
            self.popup_enabled,
            voice_active,
            voice_arg,
        );

        if !self.break_script.is_empty() {
            let _ = ScriptRunner::execute(&self.break_script, "break", self.break_count);
        }
    }
}
