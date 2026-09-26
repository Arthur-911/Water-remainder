use std::path::Path;
use std::process::{Command, Stdio};
use colored::*;

pub struct ScriptRunner;

impl ScriptRunner {
    /// Executes a custom script or command associated with a reminder event.
    pub fn execute(
        script_or_cmd: &str,
        reminder_type: &str,
        count: usize,
    ) -> Result<String, String> {
        let trimmed = script_or_cmd.trim();
        if trimmed.is_empty() {
            return Ok("No script specified".to_string());
        }

        println!(
            "{} Executing script hook for {}: {}",
            "⚡ [ScriptRunner]".bright_cyan().bold(),
            reminder_type.yellow(),
            trimmed
        );

        let path = Path::new(trimmed);
        let mut cmd = if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            match ext.as_str() {
                "ps1" => {
                    let mut c = Command::new("powershell.exe");
                    c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", trimmed]);
                    c
                }
                "bat" | "cmd" => {
                    let mut c = Command::new("cmd.exe");
                    c.args(["/C", trimmed]);
                    c
                }
                "py" => {
                    let mut c = Command::new("python");
                    c.args([trimmed]);
                    c
                }
                _ => Command::new(trimmed),
            }
        } else {
            // Treat as raw shell command
            let mut c = Command::new("cmd.exe");
            c.args(["/C", trimmed]);
            c
        };

        // Pass context environment variables so user scripts know what triggered them
        cmd.env("REMINDER_TYPE", reminder_type)
            .env("REMINDER_COUNT", count.to_string())
            .env("REMINDER_TIME", chrono::Local::now().to_rfc3339())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

                if output.status.success() {
                    if !stdout.is_empty() {
                        println!("{} Output: {}", "  ↳".bright_green(), stdout);
                    }
                    Ok(stdout)
                } else {
                    let err_msg = format!(
                        "Script failed (exit code {:?}): {}",
                        output.status.code(),
                        if !stderr.is_empty() { stderr } else { stdout }
                    );
                    eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                    Err(err_msg)
                }
            }
            Err(e) => {
                let err_msg = format!("Failed to spawn process: {}", e);
                eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                Err(err_msg)
            }
        }
    }
}
