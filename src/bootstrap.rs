/*!
Utilities for bootstrapping an app that uses the Windows App SDK.
!*/
use bindings::Windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
};

/// A target version of the Windows App SDK.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageVersion {
    /// The build revision
    pub revision: u8,
    /// The build version
    pub build: u8,
    /// The minor version
    pub minor: u8,
    /// The major version
    pub major: u8,
}

impl PackageVersion {
    /// Create a new Windows App SDK package version.
    pub fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self {
        Self {
            major,
            minor,
            build,
            revision,
        }
    }

    fn to_major_minor(self) -> u32 {
        ((self.major as u32) << 8) | self.minor as u32
    }
}

#[test]
#[allow(non_snake_case)]
fn package_version_zero_produces_zero_major_minor() {
    let v = PackageVersion {
        major: 0,
        minor: 0,
        revision: 0,
        build: 0,
    };
    assert_eq!(v.to_major_minor(), 0);
}

#[test]
fn package_version_max_produces_correct_major_minor() {
    let v = PackageVersion {
        major: 255,
        minor: 255,
        revision: 42,
        build: 42,
    };
    assert_eq!(v.to_major_minor(), 255 << 8 | 255);
}

#[test]
fn package_version_mixed_produces_correct_major_minor() {
    let v = PackageVersion {
        major: 0,
        minor: 8,
        revision: 42,
        build: 192,
    };
    assert_eq!(v.to_major_minor(), 0 << 8 | 8);
}

#[test]
fn package_version_has_expected_c_representation() {
    let v = PackageVersion {
        major: 1,
        minor: 2,
        build: 3,
        revision: 4,
    };
    unsafe {
        let bytes = &v as *const _ as *const u8;
        assert_eq!(std::slice::from_raw_parts(bytes, 4), [4, 3, 2, 1]);
    }
}

#[link(name = "Microsoft.ProjectReunion.Bootstrap")]
extern "system" {
    fn MddBootstrapInitialize(
        majorMinorVersion: u32,
        versionTag: *const u16,
        minVersion: PackageVersion,
    ) -> windows::HRESULT;

    fn MddBootstrapShutdown() -> windows::HRESULT;
}

/// Locates the Windows App framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
///
/// On error a dialogue box will be displayed. To not have the dialogue box displayed,
/// use [`initialize_without_dialog`] instead.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::Result<()> {
    initialize_without_dialog()
    .map_err(|outer_error| {
        unsafe {
            MessageBoxW(
                HWND::default(),
                "To run this application, the Windows App preview runtime must be installed.\n\nhttps://aka.ms/projectreunion/0.8preview",
                "This application could not be started",
                MB_OK | MB_ICONERROR,
            );
            outer_error
        }
    })
}

/// Locates the Windows App framework package compatible with the (currently internal)
/// versioning criteria and loads it into the current process.
pub fn initialize_without_dialog() -> windows::Result<()> {
    let version_tag: Vec<u16> = "preview".encode_utf16().collect();
    let mdd_version = PackageVersion {
        major: 0,
        minor: 8,
        revision: 0,
        build: 0,
    };
    let min_framework_version = PackageVersion {
        major: 0,
        minor: 1,
        revision: 0,
        build: 0,
    };
    unsafe {
        MddBootstrapInitialize(
            mdd_version.to_major_minor(),
            version_tag.as_ptr(),
            min_framework_version,
        )
        .ok()
    }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::Result<()> {
    unsafe { MddBootstrapShutdown().ok() }
}
