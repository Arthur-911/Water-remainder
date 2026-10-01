@echo off
setlocal
cd /d "%~dp0"

if not exist "%~dp0target\release\reminder_runner.exe" (
    echo Binary not found. Building reminder_runner in release mode...
    cargo build --release
    if errorlevel 1 (
        echo [Error] Cargo build failed.
        pause
        exit /b 1
    )
)

if not "%~1"=="" (
    "%~dp0target\release\reminder_runner.exe" %*
    goto :eof
)

:menu
cls
echo =======================================================
echo   Health ^& Work Reminder Runner
echo =======================================================
echo   [1] Interactive Mode  - Live countdown dashboard
echo   [2] Background Mode   - Silent hourly notifications
echo   [3] Test Mode         - Rapid 10s/15s test reminders
echo   [4] Test AI Voice     - Play AI voice message ("It's time to drink water and get rest")
echo   [5] Stop Background   - Stop background runner
echo   [6] Exit
echo =======================================================
set /p choice="Select an option [1-6]: "

if "%choice%"=="1" goto interactive
if "%choice%"=="2" goto background
if "%choice%"=="3" goto test
if "%choice%"=="4" goto voice
if "%choice%"=="5" goto stop
if "%choice%"=="6" goto :eof
goto menu

:interactive
call "%~dp0scripts\start_interactive.bat"
goto :eof

:background
start "" wscript.exe "%~dp0scripts\start_background.vbs"
goto :eof

:test
call "%~dp0scripts\test_runner.bat"
goto :eof

:voice
"%~dp0target\release\reminder_runner.exe" --test-voice
pause
goto menu

:stop
call "%~dp0scripts\stop_runner.bat"
goto :eof
