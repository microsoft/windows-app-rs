/*!
Utilities for bootstrapping an app that uses the Windows App SDK.
!*/

use bindings::{
    Microsoft::WindowsAppSDK::Foundation::*,
    Windows::Win32::{
        Foundation::HWND,
        Storage::Packaging::Appx::{PACKAGE_VERSION, PACKAGE_VERSION_0, PACKAGE_VERSION_0_0},
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
    },
};

/// Locates the Windows App SDK framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
///
/// On error a dialog box will be displayed. To not have the dialog box displayed,
/// use [`initialize_without_dialog`] instead.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::Result<()> {
    initialize_without_dialog().map_err(|outer_error| {
        unsafe {
            // There is no runtime download link to provide the user at this time. Awaiting resolution of
            // https://github.com/microsoft/WindowsAppSDK/issues/1205

            MessageBoxW(
                HWND::default(),
                "To run this application, the experimental Windows App SDK runtime must be installed.",
                "This application could not be started",
                MB_OK | MB_ICONERROR,
            );
            outer_error
        }
    })
}

/// Locates the Windows App SDK framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
pub fn initialize_without_dialog() -> windows::Result<()> {
    let version_tag = "experimental1";
    let mdd_version = (1 << 16) | 0_u32;
    let min_framework_version = PACKAGE_VERSION {
        Anonymous: PACKAGE_VERSION_0 {
            Anonymous: PACKAGE_VERSION_0_0 {
                Revision: 0,
                Build: 0,
                Minor: 218,
                Major: 0,
            },
        },
    };

    unsafe { MddBootstrapInitialize(mdd_version, version_tag, min_framework_version) }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::Result<()> {
    unsafe { MddBootstrapShutdown() }
    Ok(())
}
