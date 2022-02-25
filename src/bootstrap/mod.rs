//! Utilities for bootstrapping an app that uses the Windows App SDK.

pub mod deploy;

use crate::Microsoft::WindowsAppSdk::Foundation::*;
use windows::Win32::Storage::Packaging::Appx::{
    PACKAGE_VERSION, PACKAGE_VERSION_0, PACKAGE_VERSION_0_0,
};

#[allow(clippy::identity_op)]
/// Locates the Windows App SDK framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::core::Result<()> {
    let mdd_version = (1 << 16) | 0_u32;
    let min_framework_version = PACKAGE_VERSION {
        Anonymous: PACKAGE_VERSION_0 {
            Anonymous: PACKAGE_VERSION_0_0 {
                Revision: 0,
                Build: 455,
                Minor: 319,
                Major: 0,
            },
        },
    };

    unsafe { MddBootstrapInitialize(mdd_version, windows::core::PCWSTR::default(), min_framework_version) }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::core::Result<()> {
    unsafe { MddBootstrapShutdown() }
    Ok(())
}
