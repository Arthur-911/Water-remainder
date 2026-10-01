@echo off
title Health & Work Reminder Runner
echo Starting Health & Work Reminder Runner in interactive mode...
echo (Press Ctrl+C in this window at any time to exit)
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
"target\release\reminder_runner.exe"
pause
