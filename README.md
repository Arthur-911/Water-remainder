# 💧 Health & Work Reminder Runner (Rust)

A high-performance, non-intrusive background runner built in **Rust** for Windows. It keeps you hydrated and physically refreshed while you work by reminding you to **drink water** and **take a break/stretch** every hour.

---

## ✨ Features

- **AI-Generated Voice Reminders**: Speaks a natural, friendly AI-generated voice message (*"It's time to drink water and get rest"*) whenever a reminder triggers so you never miss it even if looking away.
- **Native Windows Toast Notifications**: Clean desktop popups with audio chimes in the bottom-right corner that don't interrupt your typing.
- **On-Screen Pop-up Dialog Window**: Prominent top-most dialog box that pops up right on your screen so you never miss an hourly water or break reminder even if Windows Focus Assist / Do Not Disturb is enabled.
- **Hourly Health Schedules**: Default 60-minute intervals for both water and desk/eye breaks.
- **Rotating Wellness Tips**:
  - **Hydration Tips**: Reminders on focus, brain hydration, energy levels, and water intake.
  - **Ergonomic Break Tips**: 20-20-20 eye strain rule, shoulder rolls, deep breathing, posture checks, and light walking.
- **Script Runner / Hook Capabilities**: Execute custom `.bat`, `.cmd`, `.ps1`, `.py`, or shell commands automatically whenever a reminder fires.
- **Interactive Live Dashboard**: Real-time terminal countdown showing exact time remaining, total reminders sent, and session statistics.
- **Silent Background Mode**: Run completely hidden in the background without any open console window while you work.
- **Configuration File**: Easily configure intervals, messages, scripts, and sound in `reminder_config.toml`.
- **Graceful Shutdown**: Press `Ctrl+C` in interactive mode to view your total health session summary.

---

## 🚀 Quick Start

The project includes an interactive launcher script at the repository root:

```powershell
.\run.bat
```
*(Brings up an intuitive menu to start interactive mode, background mode, or quick test mode).*

You can also run specific scripts directly:

### 1. Test Notifications Immediately
Verify Windows notifications and sound on your PC:
```powershell
.\scripts\test_runner.bat
```
*(Fires immediate test notifications every 10–15 seconds)*

### 2. Run Interactively (With Live Countdown)
```powershell
.\scripts\start_interactive.bat
```
You will see a live status counter:
```text
[19:45:10] 💧 Water in: 48m 12s (done: 1) | 🧘 Break in: 18m 12s (done: 2)
```

### 3. Run Silently in the Background (While Working)
If you don't want a terminal window open while you work:
- Double-click **`scripts\start_background.vbs`**
- It runs quietly in the Windows background and sends you toast notifications and pop-ups every hour.
- To stop the background runner at any time, run **`scripts\stop_runner.bat`**.

---

## 📂 Project Structure

```text
├── Cargo.toml               # Rust package & dependency definitions
├── reminder_config.toml     # User preferences & reminder configuration
├── run.bat                  # Root unified launcher menu & CLI dispatcher
├── scripts/
│   ├── start_interactive.bat# Interactive live dashboard runner
│   ├── start_background.vbs # Silent background runner
│   ├── stop_runner.bat      # Process termination script
│   ├── test_runner.bat      # Rapid 10s/15s test runner
│   ├── sample_water_hook.bat# Sample water reminder script hook
│   └── sample_break_hook.ps1# Sample break reminder script hook
└── src/
    ├── main.rs              # Application entry point & signal handling
    ├── cli.rs               # Command-line interface definitions & overrides
    ├── ui.rs                # Terminal rendering, banners & countdown formatting
    ├── timer.rs             # Core reminder scheduling & dispatch loop
    ├── runner.rs            # Custom script hook process executor
    ├── tips.rs              # Curated hydration & ergonomic break tips
    ├── config/
    │   ├── mod.rs           # Configuration parser & file manager
    │   └── defaults.rs      # Default values & commented TOML template generator
    └── notification/
        ├── mod.rs           # High-level notification coordinator
        ├── dialog.rs        # Win32 topmost modal dialog implementation
        └── toast.rs         # Windows desktop toast notification delivery
```

---

## ⚙ Configuration (`reminder_config.toml`)

Edit `reminder_config.toml` to customize your preferences:

```toml
# Interval in minutes between water reminders (default: 60)
water_interval_mins = 60

# Interval in minutes between break reminders (default: 60)
break_interval_mins = 60

# Optional stagger offset in minutes (e.g. 30 staggers breaks 30 mins after water)
stagger_minutes = 0

# Play audio chime with notification
sound_enabled = true

# Show on-screen pop-up dialog box (in addition to toast notification)
popup_enabled = true

# Play AI-generated voice message ("It's time to drink water and get rest")
voice_enabled = true

# Play voice message on water reminder
voice_on_water = true

# Play voice message on break reminder
voice_on_break = true

# Path to voice audio file (leave default or point to a custom .mp3 / .wav file)
voice_file = "assets/voice_reminder.mp3"

# Custom title and message (leave empty to use rotating health tips)
water_title = "💧 Time to Drink Water!"
water_message = ""

# Optional script/command to execute when water reminder fires
water_script = "scripts/sample_water_hook.bat"

# Break title and custom message
break_title = "🧘 Time to Take a Break!"
break_message = ""

# Optional script/command to execute when break reminder fires
break_script = "scripts/sample_break_hook.ps1"
```

---

## ⚡ Command Line Options

You can run the binary directly or via `run.bat` with command-line flags:

```powershell
# Test-play the AI voice reminder immediately
.\run.bat --test-voice

# Custom 45-minute water and 60-minute break
.\run.bat --water 45 --break 60

# Stagger break by 30 minutes so both don't fire at the same time
.\run.bat --water 60 --break 60 --stagger 30

# Run with custom hook scripts
.\run.bat --water-script "scripts\sample_water_hook.bat" --break-script "scripts\sample_break_hook.ps1"

# Fire first reminder immediately on launch
.\run.bat --now

# Run quietly (no countdown bar, ideal for background services)
.\run.bat --quiet

# Disable or enable AI voice message
.\run.bat --no-voice
.\run.bat --voice

# Point to a custom voice file
.\run.bat --voice-file "assets\custom_voice.mp3"

# Disable or enable sound chime
.\run.bat --no-sound
.\run.bat --sound

# Disable or enable pop-up window (toast notification only)
.\run.bat --no-popup
.\run.bat --popup
```

---

## 🔌 Script Hooks & Environment Variables

When a reminder triggers, `reminder_runner` automatically passes context variables to your custom scripts:

| Variable | Description | Example |
| :--- | :--- | :--- |
| `REMINDER_TYPE` | Type of event | `"water"` or `"break"` |
| `REMINDER_COUNT` | Number of times triggered in session | `"3"` |
| `REMINDER_TIME` | ISO-8601 Timestamp | `"2026-09-26T20:00:00+05:30"` |

See `scripts/sample_water_hook.bat` and `scripts/sample_break_hook.ps1` for working examples.

---

## 🖥 Windows Startup (Optional)

To have the reminder runner start automatically every time you log into your PC:
1. Press `Win + R`, type `shell:startup` and press Enter.
2. Right-click inside your Startup folder -> **New** -> **Shortcut**.
3. Point the shortcut target to:
   ```text
   wscript.exe "C:\arthur_antigravity_shi\projects\remainder_app\scripts\start_background.vbs"
   ```
4. Now it will automatically run in the background every day!
