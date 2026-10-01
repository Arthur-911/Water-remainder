use std::path::Path;
use std::process::{Command, Stdio};
use colored::*;

pub struct ScriptRunner;

impl ScriptRunner {
    /// Executes a custom script or command in a background thread to prevent blocking the timer.
    pub fn execute_async(script_or_cmd: &str, reminder_type: &str, count: usize) {
        let script = script_or_cmd.trim().to_string();
        if script.is_empty() {
            return;
        }
        let reminder_type = reminder_type.to_string();
        std::thread::spawn(move || {
            let _ = Self::execute(&script, &reminder_type, count);
        });
    }

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
        let resolved_file = if path.is_file() {
            Some(path.to_path_buf())
        } else if let Ok(exe) = std::env::current_exe() {
            exe.parent().map(|p| p.join(trimmed)).filter(|p| p.is_file())
        } else {
            None
        };

        let mut cmd = if let Some(ref target) = resolved_file {
            let ext = target
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let target_str = target.to_string_lossy().to_string();

            match ext.as_str() {
                "ps1" => {
                    let mut c = Command::new("powershell.exe");
                    c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", &target_str]);
                    c
                }
                "bat" | "cmd" => {
                    let mut c = Command::new("cmd.exe");
                    c.args(["/C", &target_str]);
                    c
                }
                "py" => {
                    let mut c = Command::new("python");
                    c.args([&target_str]);
                    c
                }
                _ => Command::new(&target_str),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_empty_script() {
        let res = ScriptRunner::execute("", "water", 1);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "No script specified");
    }

    #[test]
    fn test_execute_whitespace_script() {
        let res = ScriptRunner::execute("   ", "break", 2);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "No script specified");
    }

    #[test]
    fn test_execute_simple_command() {
        let res = ScriptRunner::execute("echo Hello from runner test", "water", 1);
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Hello from runner test"));
    }
}
