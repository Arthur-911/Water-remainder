@echo off
:: Sample hook script executed when the Water Reminder fires
:: Environment variables provided by reminder_runner:
::   %REMINDER_TYPE%      - "water"
::   %REMINDER_COUNT%     - e.g. 1, 2, 3...
::   %REMINDER_TIME%      - ISO timestamp
::
echo [%REMINDER_TIME%] Water reminder triggered (#%REMINDER_COUNT%) >> "%~dp0\reminder_log.txt"
echo [Hook] Water reminder logged successfully to reminder_log.txt
