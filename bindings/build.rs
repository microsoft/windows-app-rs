fn main() {
    windows::build! {
        Microsoft::Foundation::*,
        Microsoft::UI::Xaml::Window,
        Windows::Win32::Foundation::{BOOL, RECT},
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::*,
    };
}
