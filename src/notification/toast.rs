use colored::*;
use notify_rust::{Notification, Timeout};

/// Delivers a desktop toast notification with an optional chime sound.
pub fn show_toast(
    app_name: &str,
    title: &str,
    message: &str,
    sound: bool,
) -> Result<(), String> {
    let mut notif = Notification::new();
    notif
        .appname(app_name)
        .summary(title)
        .body(message)
        .timeout(Timeout::Milliseconds(8000));

    if sound {
        notif.sound_name("Notification.Default");
    }

    match notif.show() {
        Ok(_) => {
            println!(
                "{} [{}] {}",
                "✔ [Notification & Pop-up]".bright_green().bold(),
                chrono::Local::now().format("%H:%M:%S").to_string().cyan(),
                title.bright_white().bold()
            );
            Ok(())
        }
        Err(e) => {
            let err_str = format!("Failed to deliver toast notification: {:?}", e);
            eprintln!("{} {}", "✖ [Notification Error]".bright_red().bold(), err_str);
            // Terminal bell fallback
            print!("\x07");
            Err(err_str)
        }
    }
}
