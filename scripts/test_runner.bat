@echo off
title Health & Work Reminder Runner [Test Mode]
echo Running quick test mode:
echo   - Immediate initial notification
echo   - Reminders fire every 10-15 seconds
echo   - Tests Windows toast notifications and sound
echo.
echo Press Ctrl+C to stop.
echo.
cd /d "%~dp0.."
if not exist "target\release\reminder_runner.exe" (
    echo Binary not found. Building reminder_runner...
    cargo build --release
    if errorlevel 1 (
        pause
        exit /b 1
    )
)
"target\release\reminder_runner.exe" --test --now
pause
