# Metadata generation

This MSBuild project generates metadata (`Microsoft.WindowsAppSdk.Interop.winmd`)
for various Windows App SDK platform interoperability APIs, such as
`Microsoft.UI.Xaml.IWindowNative`.

It also retrives an application manifest for use in registration-free WinRT
scenarios (i.e. xcopy), supported in Windows App SDK 1.1-preview1+.

## Usage

1. Install [the latest .NET 6.0 SDK](https://dotnet.microsoft.com/download/dotnet/6.0).
2. `Set-ExecutionPolicy -Scope CurrentUser Unrestricted`
3. `dotnet build`
