Add-Type -AssemblyName System.Speech
$synth = New-Object System.Speech.Synthesis.SpeechSynthesizer
$synth.SelectVoice("Microsoft Zira Desktop")
$wavPath = Join-Path $PSScriptRoot "..\assets\voice_reminder.wav"
$synth.SetOutputToWaveFile($wavPath)
$synth.Speak("It's time to drink water and get rest")
$synth.Dispose()
Write-Host "WAV generated at: $wavPath"
