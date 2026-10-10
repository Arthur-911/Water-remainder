use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use colored::*;

static SCRIPT_RUNNING: AtomicBool = AtomicBool::new(false);

fn get_cmd_exe() -> PathBuf {
    if let Ok(sysroot) = std::env::var("SystemRoot") {
        let candidate = PathBuf::from(sysroot).join("System32").join("cmd.exe");
        if candidate.is_file() {
            return candidate;
        }
    }
    PathBuf::from("cmd.exe")
}

fn get_powershell_exe() -> PathBuf {
    if let Ok(sysroot) = std::env::var("SystemRoot") {
        let candidate = PathBuf::from(sysroot)
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        if candidate.is_file() {
            return candidate;
        }
    }
    PathBuf::from("powershell.exe")
}

pub struct ScriptRunner;

impl ScriptRunner {
    /// Executes a custom script or command in a background thread to prevent blocking the timer.
    /// Limits concurrent script execution to avoid thread and resource exhaustion.
    pub fn execute_async(script_or_cmd: &str, reminder_type: &str, count: usize) {
        let script = script_or_cmd.trim().to_string();
        if script.is_empty() {
            return;
        }

        if SCRIPT_RUNNING.swap(true, Ordering::SeqCst) {
            eprintln!(
                "{} Previous script hook is still running. Skipping execution to avoid overlap.",
                "⚠ [ScriptRunner Warning]".bright_yellow().bold()
            );
            return;
        }

        let reminder_type = reminder_type.to_string();
        std::thread::spawn(move || {
            let _ = Self::execute(&script, &reminder_type, count);
            SCRIPT_RUNNING.store(false, Ordering::SeqCst);
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
                    let mut c = Command::new(get_powershell_exe());
                    c.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "RemoteSigned", "-File", &target_str]);
                    c
                }
                "bat" | "cmd" => {
                    let mut c = Command::new(get_cmd_exe());
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
            // Reject dangerous shell metacharacters to prevent command chaining / injection
            let contains_unsafe_shell = trimmed.chars().any(|ch| {
                matches!(ch, '&' | '|' | ';' | '<' | '>' | '`' | '$' | '\n' | '\r')
            });

            if contains_unsafe_shell {
                let err_msg = "Script execution rejected: command contains unsafe shell metacharacters (&, |, ;, <, >, `, $)".to_string();
                eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                return Err(err_msg);
            }

            let mut c = Command::new(get_cmd_exe());
            c.args(["/C", trimmed]);
            c
        };

        // Pass context environment variables so user scripts know what triggered them.
        // Redirect stdin to null so scripts cannot deadlock waiting for user input.
        cmd.env("REMINDER_TYPE", reminder_type)
            .env("REMINDER_COUNT", count.to_string())
            .env("REMINDER_TIME", chrono::Local::now().to_rfc3339())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let err_msg = format!("Failed to spawn process: {}", e);
                eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                return Err(err_msg);
            }
        };

        let child_id = child.id();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let res = child.wait_with_output();
            let _ = tx.send(res);
        });

        // 30-second execution timeout guard to prevent hung processes from blocking indefinitely
        let output = match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => {
                let err_msg = format!("Failed to read process output: {}", e);
                eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                return Err(err_msg);
            }
            Err(_) => {
                #[cfg(windows)]
                {
                    let _ = Command::new("taskkill")
                        .args(["/F", "/PID", &child_id.to_string()])
                        .output();
                }
                let err_msg = "Script timed out after 30 seconds and was terminated".to_string();
                eprintln!("{} {}", "  ↳ Error:".bright_red(), err_msg);
                return Err(err_msg);
            }
        };

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

    #[test]
    fn test_execute_rejects_shell_injection() {
        let res = ScriptRunner::execute("echo safe & echo injected", "water", 1);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("unsafe shell metacharacters"));
    }
}
