use bindings::Windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
};

mod tests;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageVersion {
    pub revision: u8,
    pub build: u8,
    pub minor: u8,
    pub major: u8,
}

impl PackageVersion {
    pub fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self {
        Self {
            revision,
            build,
            minor,
            major,
        }
    }

    fn to_major_minor(self) -> u32 {
        ((self.major as u32) << 8) | self.minor as u32
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
/// versioning criteria and loads it into the current process. If multiple packages meet
/// the criteria, the best candidate is selected.
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

/// Undo the changes made by `initialize()`
pub fn uninitialize() -> windows::Result<()> {
    unsafe { MddBootstrapShutdown().ok() }
}
