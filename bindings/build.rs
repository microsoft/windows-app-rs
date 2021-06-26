fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxW,
        Windows::Win32::Foundation::HWND,
    };
}
