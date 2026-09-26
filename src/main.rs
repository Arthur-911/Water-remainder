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

    ui::print_banner();

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
