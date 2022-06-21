//! Utilities for bootstrapping an app that uses the Windows App SDK.

pub mod deploy;

use crate::Microsoft::WindowsAppSdk::Foundation::*;
use windows::Win32::Storage::Packaging::Appx::{
    PACKAGE_VERSION, PACKAGE_VERSION_0, PACKAGE_VERSION_0_0,
};

#[allow(clippy::identity_op)]
/// Locates the Windows App SDK framework package compatible with the
/// metadata-matched versioning criteria and loads it into the current process.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::core::Result<()> {
    let min_framework_version = PACKAGE_VERSION {
        Anonymous: PACKAGE_VERSION_0 {
            Anonymous: PACKAGE_VERSION_0_0 {
                Revision: 0,
                Build: 1918,
                Minor: 524,
                Major: 1001,
            },
        },
    };

    unsafe {
        MddBootstrapInitialize2(
            0x00010001,
            WINDOWSAPPSDK_RELEASE_VERSION_TAG_W,
            min_framework_version,
            MddBootstrapInitializeOptions_OnNoMatch_ShowUI,
        )
    }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::core::Result<()> {
    unsafe { MddBootstrapShutdown() }
    Ok(())
}
