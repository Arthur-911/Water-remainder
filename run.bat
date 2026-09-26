@echo off
setlocal
cd /d "%~dp0"

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
echo   [4] Stop Background   - Stop background runner
echo   [5] Exit
echo =======================================================
set /p choice="Select an option [1-5]: "

if "%choice%"=="1" goto interactive
if "%choice%"=="2" goto background
if "%choice%"=="3" goto test
if "%choice%"=="4" goto stop
if "%choice%"=="5" goto :eof
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

:stop
call "%~dp0scripts\stop_runner.bat"
goto :eof
