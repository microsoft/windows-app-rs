use ::windows::core::*;
use ::windows_app::Microsoft::UI::Windowing::*;
use ::windows_app::*;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PostQuitMessage, TranslateMessage, MSG,
};

fn main() -> Result<()> {
    bootstrap::initialize()?;
    sample_main()?;
    bootstrap::uninitialize()
}

fn sample_main() -> Result<()> {
    // The Windows App SDK does not currently provide methods for attaching UI
    // framework content to new instances of AppWindow. We are currently limited
    // to accessing AppWindow functionality via interop APIs hanging off existing
    // instances of Window instead...

    let presenter = OverlappedPresenter::Create()?;
    presenter.SetBorderAndTitleBar(true, true)?;
    presenter.SetIsAlwaysOnTop(true)?;

    let window = AppWindow::CreateWithPresenter(presenter)?;
    window.SetTitle("Hello, world!")?;
    window.Show()?;

    window.Destroying(TypedEventHandler::new(|_, __| unsafe {
        PostQuitMessage(0);
        Ok(())
    }))?;

    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, HWND(0), 0, 0) != BOOL(0) {
            println!("{:?}", msg);
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
