use windows::Win32::{
    Foundation::{ERROR_FILE_NOT_FOUND, HWND},
    System::LibraryLoader::LoadLibraryW,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
};
use windows_app::Microsoft::Windows::System::Power::*;

fn main() -> ::windows::core::Result<()> {
    unsafe {
        if LoadLibraryW(r"Microsoft.WindowsAppRuntime.dll").0 == 0 {
            MessageBoxW(
                HWND::default(),
                "This sample requires Windows App SDK files to be deployed before use. (Run Stage.ps1)",
                "Error",
                MB_ICONSTOP | MB_OK,
            );
            return Err(ERROR_FILE_NOT_FOUND.into());
        }
    }

    let charge = PowerManager::RemainingChargePercent()?;
    println!("Remaining charge: {charge}%");
    Ok(())
}
