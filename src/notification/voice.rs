use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use colored::*;

static VOICE_PLAYING: AtomicBool = AtomicBool::new(false);
static EMBEDDED_VOICE_MP3: &[u8] = include_bytes!("../../assets/voice_reminder.mp3");

#[link(name = "winmm")]
unsafe extern "system" {
    fn mciSendStringW(
        lpstrCommand: *const u16,
        lpstrReturnString: *mut u16,
        uReturnLength: u32,
        hwndCallback: *mut std::ffi::c_void,
    ) -> u32;
}

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// Resolves the voice file path from disk or extracts the embedded MP3 into a temporary directory.
pub fn resolve_or_extract_audio(custom_file: Option<&str>) -> Option<PathBuf> {
    // 1. Check custom file if supplied
    if let Some(cf) = custom_file {
        if !cf.trim().is_empty() {
            let p = Path::new(cf);
            if p.is_file() {
                return Some(p.to_path_buf());
            }
            if let Ok(exe) = std::env::current_exe() {
                if let Some(parent) = exe.parent() {
                    let candidate = parent.join(cf);
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        }
    }

    // 2. Check default relative assets/voice_reminder.mp3
    let default_path = Path::new("assets/voice_reminder.mp3");
    if default_path.is_file() {
        return Some(default_path.to_path_buf());
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("assets").join("voice_reminder.mp3");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // 3. Fallback: extract embedded MP3 to %TEMP%
    let temp_dir = std::env::temp_dir();
    let temp_audio = temp_dir.join("reminder_voice_ai.mp3");

    let needs_write = match std::fs::metadata(&temp_audio) {
        Ok(meta) => meta.len() as usize != EMBEDDED_VOICE_MP3.len(),
        Err(_) => true,
    };

    if needs_write {
        if let Err(e) = std::fs::write(&temp_audio, EMBEDDED_VOICE_MP3) {
            eprintln!(
                "{} Could not extract embedded voice: {}",
                "⚠ [Voice Warning]".bright_yellow().bold(),
                e
            );
            return None;
        }
    }

    Some(temp_audio)
}

/// Plays the AI voice reminder in a background thread.
pub fn play_voice(custom_file: Option<&str>) {
    let resolved = resolve_or_extract_audio(custom_file);

    std::thread::spawn(move || {
        play_internal(resolved);
    });
}

/// Plays the AI voice reminder synchronously (used for testing voice).
pub fn play_voice_blocking(custom_file: Option<&str>) {
    let resolved = resolve_or_extract_audio(custom_file);
    play_internal(resolved);
}

fn play_internal(resolved_path: Option<PathBuf>) {
    if VOICE_PLAYING.swap(true, Ordering::SeqCst) {
        // Prevent overlapping speech
        return;
    }

    println!(
        "{} {}",
        "🗣 [AI Voice Reminder]".bright_magenta().bold(),
        "\"It's time to drink water and get rest\"".bright_white().italic()
    );

    let clean_path = match resolved_path {
        Some(p) => match std::fs::canonicalize(&p) {
            Ok(canon) => canon.to_string_lossy().trim_start_matches(r"\\?\").to_string(),
            Err(_) => p.to_string_lossy().to_string(),
        },
        None => {
            fallback_tts();
            VOICE_PLAYING.store(false, Ordering::SeqCst);
            return;
        }
    };

    let play_success = unsafe {
        mciSendStringW(to_wide("close reminder_voice").as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
        let open_cmd = format!(r#"open "{}" type mpegvideo alias reminder_voice"#, clean_path);
        let open_res = mciSendStringW(to_wide(&open_cmd).as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
        if open_res == 0 {
            let play_cmd = "play reminder_voice wait";
            mciSendStringW(to_wide(play_cmd).as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
            mciSendStringW(to_wide("close reminder_voice").as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
            true
        } else {
            false
        }
    };

    if !play_success {
        fallback_play(&clean_path);
    }

    VOICE_PLAYING.store(false, Ordering::SeqCst);
}

fn fallback_play(path: &str) {
    let escaped = path.replace('\'', "''");
    let script = format!(
        r#"Add-Type -AssemblyName presentationCore; $p = New-Object System.Windows.Media.MediaPlayer; $p.Open([System.Uri]'{}'); $p.Play(); Start-Sleep -Seconds 3; $p.Close()"#,
        escaped
    );
    let res = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status();

    if res.map(|s| !s.success()).unwrap_or(true) {
        fallback_tts();
    }
}

fn fallback_tts() {
    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            r#"Add-Type -AssemblyName System.Speech; (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak("It's time to drink water and get rest")"#,
        ])
        .status();
}
