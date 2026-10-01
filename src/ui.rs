use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use colored::*;

/// Prints the stylized ASCII application logo.
pub fn print_banner() {
    println!(
        "{}",
        r#"
  __  __         _ _   _       ____                       _           
 |  \/  |___  __| (_) | |_ ___|  _ \ _   _ _ __  _ __ ___| | _____  __
 | |\/| / _ \/ _` | | | __/ _ \ |_) | | | | '_ \| '_ \ / _ \ |/ / _ \/ _ \
 | |  | \__  \ (_| | | | || (_) |  _ <| |_| | | | | | |  __/   <  __/ (_)
 |_|  |_|___/\__,_|_|_|\__\___/|_| \_\\__,_|_| |_|_| |_|\___|_|\_\___|\___/ 
        "#
        .bright_cyan()
    );
}

/// Displays the active runner monitoring configuration block.
pub fn print_status(
    water_interval: Duration,
    break_interval: Duration,
    popup_enabled: bool,
    voice_enabled: bool,
    water_script: &str,
    break_script: &str,
) {
    println!();
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════".bright_blue()
    );
    println!(
        "  {} Active! Monitoring in background.",
        "Health & Work Reminder Runner".bright_cyan().bold()
    );
    println!(
        "  Water interval: {} | Break interval: {}",
        format_duration(water_interval).yellow(),
        format_duration(break_interval).yellow()
    );
    println!(
        "  Notifications : {} | On-screen Pop-up: {}",
        "Enabled (Toast)".green(),
        if popup_enabled {
            "Enabled (Top-most Dialog)".green()
        } else {
            "Disabled".yellow()
        }
    );
    println!(
        "  AI Voice Alert: {}",
        if voice_enabled {
            "Enabled (\"It's time to drink water and get rest\")".green()
        } else {
            "Disabled".yellow()
        }
    );
    if !water_script.is_empty() {
        println!("  Water script hook: {}", water_script.green());
    }
    if !break_script.is_empty() {
        println!("  Break script hook: {}", break_script.green());
    }
    println!(
        "  Press {} at any time to exit.",
        "Ctrl+C".bright_red().bold()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════".bright_blue()
    );
    println!();
}

/// Clears the current countdown terminal line before printing notification events.
pub fn clear_line() {
    print!("\r\x1B[2K");
    let _ = stdout().flush();
}

/// Renders the real-time in-place countdown status line in the terminal.
pub fn render_countdown(
    next_water: Instant,
    next_break: Instant,
    water_count: usize,
    break_count: usize,
) {
    let now = Instant::now();
    let water_rem = next_water.saturating_duration_since(now);
    let break_rem = next_break.saturating_duration_since(now);

    let time_str = chrono::Local::now().format("%H:%M:%S").to_string();

    print!(
        "\r[{}] 💧 Water in: {} (done: {}) | 🧘 Break in: {} (done: {})  ",
        time_str.bright_black(),
        format_duration(water_rem).bright_cyan(),
        water_count.to_string().cyan(),
        format_duration(break_rem).bright_yellow(),
        break_count.to_string().yellow()
    );
    let _ = stdout().flush();
}

/// Prints a formatted summary upon application exit.
pub fn print_summary(water_count: usize, break_count: usize) {
    println!();
    println!();
    println!(
        "{}",
        "───────────────────────────────────────────────────────────".bright_blue()
    );
    println!(
        "  {} Session Summary",
        "✨ Health & Work Reminder".bright_green().bold()
    );
    println!(
        "  💧 Water reminders delivered : {}",
        water_count.to_string().bright_cyan().bold()
    );
    println!(
        "  🧘 Break reminders delivered : {}",
        break_count.to_string().bright_yellow().bold()
    );
    println!("  Great job taking care of your hydration and health today!");
    println!(
        "{}",
        "───────────────────────────────────────────────────────────".bright_blue()
    );
}

/// Formats a duration into a human-readable string (e.g. `1h 00m 00s`, `15m 30s`, or `12s`).
pub fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{:02}m {:02}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0s");
        assert_eq!(format_duration(Duration::from_secs(45)), "45s");
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(Duration::from_secs(60)), "01m 00s");
        assert_eq!(format_duration(Duration::from_secs(90)), "01m 30s");
        assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h 00m 00s");
        assert_eq!(format_duration(Duration::from_secs(3665)), "1h 01m 05s");
        assert_eq!(format_duration(Duration::from_secs(7325)), "2h 02m 05s");
    }
}
