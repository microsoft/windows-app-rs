Write-Output "[*] Restoring NuGet packages..."
nuget.exe restore -PackagesDirectory .packages

Write-Output "[*] Extracting Windows App Runtime into target directory..."
Expand-Archive .\.packages\Microsoft.WindowsAppSDK.1.1.0-preview1\tools\MSIX\win10-x64\Microsoft.WindowsAppRuntime.?.?-*.msix ..\..\..\target\debug\ -Force

Write-Output "[*] Done."
