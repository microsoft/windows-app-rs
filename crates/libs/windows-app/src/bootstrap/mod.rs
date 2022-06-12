//! Utilities for bootstrapping an app that uses the Windows App SDK.

use crate::Microsoft::WindowsAppSdk::Foundation::*;
use windows::Win32::Storage::Packaging::Appx::{
    PACKAGE_VERSION, PACKAGE_VERSION_0, PACKAGE_VERSION_0_0,
};

/// Locates the Windows App SDK framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::core::Result<()> {
    let mdd_version = 0x00010001;
    let min_framework_version = PACKAGE_VERSION {
        Anonymous: PACKAGE_VERSION_0 {
            Anonymous: PACKAGE_VERSION_0_0 {
                Major: 1000,
                Minor: 516,
                Build: 2156,
                Revision: 0,
            },
        },
    };
    let min_framework_version = PACKAGE_VERSION::default();

    unsafe {
        MddBootstrapInitialize2(
            mdd_version,
            "",
            min_framework_version,
            MddBootstrapInitializeOptions_None,
        )
    }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::core::Result<()> {
    unsafe { MddBootstrapShutdown() }
    Ok(())
}
