# Metadata generation

This MSBuild project generates metadata (`Microsoft.WindowsAppSdk.Interop.winmd`)
for various Windows App SDK platform interoperability APIs, such as
`Microsoft.UI.Xaml.IWindowNative`.

## Usage


1. Install [the latest .NET 5.0 SDK](https://dotnet.microsoft.com/download/dotnet/5.0).
2. `Set-ExecutionPolicy -Scope CurrentUser Unrestricted`
3.  `dotnet build`
