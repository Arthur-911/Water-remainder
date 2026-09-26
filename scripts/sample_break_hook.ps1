# Sample PowerShell hook script executed when the Break Reminder fires
# Environment variables provided by reminder_runner:
#   $env:REMINDER_TYPE      - "break"
#   $env:REMINDER_COUNT     - e.g. 1, 2, 3...
#   $env:REMINDER_TIME      - ISO timestamp

$logPath = Join-Path $PSScriptRoot "reminder_log.txt"
$entry = "[$($env:REMINDER_TIME)] Break reminder triggered (#$($env:REMINDER_COUNT))"
Add-Content -Path $logPath -Value $entry

Write-Output "[Hook] Break reminder logged successfully to $logPath"
