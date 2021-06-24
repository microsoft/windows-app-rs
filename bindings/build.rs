fn main() {
    windows::build! {
        Microsoft::UI::Xaml::Window,
        Windows::Win32::Foundation::{BOOL, HWND, RECT},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::{
            GetSystemMetrics, GetWindowRect, MessageBoxW, SetWindowPos, MESSAGEBOX_STYLE,
            SYSTEM_METRICS_INDEX,
        },
    };
}
