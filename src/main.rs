mod cli;
mod config;
mod notification;
mod runner;
mod timer;
mod tips;
mod ui;

use clap::Parser;
use colored::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::cli::Cli;
use crate::config::AppConfig;
use crate::timer::ReminderRunner;

fn main() {
    let cli = Cli::parse();

    let mut cfg = AppConfig::load_or_create(&cli.config);
    cli.apply_to_config(&mut cfg);

    if !cli.quiet {
        ui::print_banner();
    }

    if cli.test_voice {
        println!("{}", "Testing AI voice message...".bright_cyan());
        let voice_arg = if cfg.voice_file.is_empty() {
            None
        } else {
            Some(cfg.voice_file.as_str())
        };
        crate::notification::voice::play_voice_blocking(voice_arg);
        println!("{}", "Voice message test complete.".bright_green());
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!(
            "{} Could not set Ctrl+C handler: {}",
            "⚠ [Warning]".yellow(),
            e
        );
    }

    let mut runner = ReminderRunner::new(&cfg, cli.test, cli.now, cli.quiet);
    runner.run(running);
}
