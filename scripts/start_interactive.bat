@echo off
title Health & Work Reminder Runner
echo Starting Health & Work Reminder Runner in interactive mode...
echo (Press Ctrl+C in this window at any time to exit)
echo.
cd /d "%~dp0.."
"target\release\reminder_runner.exe"
pause
