pub mod dialog;
pub mod toast;
pub mod voice;

pub struct Notifier;

impl Notifier {
    /// Shows both an on-screen dialog pop-up, desktop toast notification, and optional AI voice message for drinking water.
    pub fn show_water_reminder(
        title: &str,
        message: &str,
        sound: bool,
        popup: bool,
        voice: bool,
        voice_file: Option<&str>,
    ) -> Result<(), String> {
        if voice {
            voice::play_voice(voice_file);
        }

        if popup {
            let popup_body = format!(
                "Time to drink water!\n\n💡 Health Tip:\n{}\n\nClick OK once you've had a fresh glass.",
                message
            );
            dialog::show_dialog(title, &popup_body);
        }

        toast::show_toast("💧 Drink Water Reminder", title, message, sound)
    }

    /// Shows both an on-screen dialog pop-up, desktop toast notification, and optional AI voice message for taking a break.
    pub fn show_break_reminder(
        title: &str,
        message: &str,
        sound: bool,
        popup: bool,
        voice: bool,
        voice_file: Option<&str>,
    ) -> Result<(), String> {
        if voice {
            voice::play_voice(voice_file);
        }

        if popup {
            let popup_body = format!(
                "Time for a break!\n\n💡 Health Tip:\n{}\n\nStep away from your screen, stretch, and click OK.",
                message
            );
            dialog::show_dialog(title, &popup_body);
        }

        toast::show_toast("🧘 Break & Stretch Reminder", title, message, sound)
    }
}
