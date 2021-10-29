fn main() {
    windows::runtime::build! {
        Microsoft::UI::Xaml::Controls::{Button, XamlControlsResources},
        Microsoft::UI::Xaml::{Application, Window, IWindowNative},
        Microsoft::WindowsAppSDK::Foundation::*,
        Windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
        Windows::Win32::Foundation::{BOOL, HWND},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, GetWindowRect, SetWindowPos, MessageBoxW},
        Windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,
    };
}
