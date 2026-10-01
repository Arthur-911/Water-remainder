' Launch Health & Work Reminder Runner silently in the background
Set WshShell = CreateObject("WScript.Shell")
Set Fso = CreateObject("Scripting.FileSystemObject")
strScriptsDir = Fso.GetParentFolderName(WScript.ScriptFullName)
strRootDir = Fso.GetParentFolderName(strScriptsDir)

WshShell.CurrentDirectory = strRootDir
exeFullPath = strRootDir & "\target\release\reminder_runner.exe"

If Not Fso.FileExists(exeFullPath) Then
    MsgBox "Could not find reminder_runner.exe!" & vbCrLf & vbCrLf & _
           "Please run 'run.bat' first to compile the application in release mode.", _
           vbCritical, "Reminder Runner Missing"
    WScript.Quit 1
End If

exePath = """" & exeFullPath & """ --quiet"

' 0 = Hide window, False = Do not wait for script to finish
WshShell.Run exePath, 0, False

MsgBox "Health & Work Reminder is now running silently in the background!" & vbCrLf & vbCrLf & _
       "You will receive desktop toast notifications every hour to:" & vbCrLf & _
       "  * Drink water" & vbCrLf & _
       "  * Take a screen break & stretch" & vbCrLf & vbCrLf & _
       "To stop the runner, run 'scripts\stop_runner.bat'.", _
       vbInformation, "Reminder Runner Active"
