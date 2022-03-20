[![Build and Test](https://github.com/microsoft/windows-app-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-app-rs/actions)

# Rust for the Windows App SDK
The `windows-app` crate lets you call any [Windows App SDK](https://github.com/microsoft/WindowsAppSDK) (formerly known as Project Reunion) API using code generated from the metadata describing the API. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

Note: This is an experimental 🧪 crate and is not ready for production use.

## Release channel coverage
The Windows App SDK is delivered via [three release channels](https://docs.microsoft.com/windows/apps/windows-app-sdk/release-channels)—experimental, preview, and stable. The `windows-app` crate currently targets APIs available in the preview and stable channels.

## Getting started
It's very early days for the `windows-app` crate. To try it out, add the following to your Cargo.toml file:

```toml
[build-dependencies.windows-app]
git = "https://github.com/microsoft/windows-app-rs"
features = [
    "WindowsAppSdk_Foundation"
]

[dependencies.windows]
version = "0.34.0"

[dependencies.windows-app]
git = "https://github.com/microsoft/windows-app-rs"
features = [
    "WindowsAppSdk_Foundation",
    "Windows_System_Power"
]
```

Add a build script (`build.rs`) to your crate to deploy the [Windows App SDK Bootstrapper](https://docs.microsoft.com/en-us/windows/apps/windows-app-sdk/deploy-unpackaged-apps?WT.mc_id=WD-MVP-5002756#using-features-at-run-time) with your app:

```rust
fn main() {
    ::windows_app::bootstrap::deploy::to_output_dir();
}
```

Now make use of any Windows App SDK APIs as needed:

```rust
use ::windows_app::Microsoft::Windows::System::Power::*;
use ::windows_app::*;

fn main() -> ::windows::core::Result<()> {
    bootstrap::initialize()?;
    let charge = PowerManager::RemainingChargePercent()?;
    println!("Remaining charge: {charge}%");
    bootstrap::uninitialize()
}
```

Finally, install the [Windows App Runtime](https://docs.microsoft.com/windows/apps/windows-app-sdk/deploy-unpackaged-apps?WT.mc_id=WD-MVP-5002756) on all target machines:

1. Download the [Windows App Runtime Redistributable](https://aka.ms/windowsappsdk/1.0-stable/msix-installer) package.
2. Execute `WindowsAppSDK-Installer-{arch}\WindowsAppRuntimeInstall.exe`.

After you install the Windows App Runtime, it will be kept up-to-date by Microsoft via Windows Update.
