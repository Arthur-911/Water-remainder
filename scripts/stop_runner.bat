@echo off
echo Stopping Health & Work Reminder Runner...
taskkill /IM reminder_runner.exe /F 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Successfully stopped Reminder Runner!
) else (
    echo Reminder Runner is not currently running.
)
timeout /t 2 >nul
