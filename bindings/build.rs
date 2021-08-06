fn main() {
    windows::build! {
        Microsoft::UI::Xaml::Controls::Button,
        Microsoft::UI::Xaml::{Application, Window, IWindowNative},
        Windows::Win32::Foundation::{BOOL, HWND},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, GetWindowRect, SetWindowPos, MessageBoxW},
        Windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,
    };
}