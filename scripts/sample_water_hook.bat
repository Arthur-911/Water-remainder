@echo off
:: Sample hook script executed when the Water Reminder fires
:: Environment variables provided by reminder_runner:
::   %REMINDER_TYPE%      - "water"
::   %REMINDER_COUNT%     - e.g. 1, 2, 3...
::   %REMINDER_TIME%      - ISO timestamp
::
set "LOG_FILE=%~dp0reminder_log.txt"
if exist "%LOG_FILE%" (
    for %%F in ("%LOG_FILE%") do (
        if %%~zF GTR 524288 (
            move /y "%LOG_FILE%" "%LOG_FILE%.bak" >nul
        )
    )
)
echo [%REMINDER_TIME%] Water reminder triggered (#%REMINDER_COUNT%) >> "%LOG_FILE%"
echo [Hook] Water reminder logged successfully to reminder_log.txt
