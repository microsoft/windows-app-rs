use ::windows::core::*;
use ::windows_app::Microsoft::UI::Windowing::*;
use ::windows_app::*;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::{BOOL, HWND, RECT};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, GetSystemMetrics, GetWindowRect, PostQuitMessage, SetWindowPos,
    TranslateMessage, MSG, SM_CXSCREEN, SM_CYSCREEN, SWP_NOMOVE, SWP_NOSIZE,
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

    window.Destroying(TypedEventHandler::new(|_, __| unsafe {
        PostQuitMessage(0);
        Ok(())
    }))?;

    let hwnd = HWND(window.Id()?.Value as _);
    resize_window(hwnd, 800, 600).then(|| {
        center_window(hwnd);
    });

    window.Show()?;

    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, HWND(0), 0, 0) != BOOL(0) {
            println!("{:#?}", msg);
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

pub fn resize_window(handle: HWND, width: u32, height: u32) -> bool {
    let scale_factor = unsafe { GetDpiForWindow(handle) } / 96;
    let width = width * scale_factor;
    let height = height * scale_factor;
    unsafe {
        SetWindowPos(
            handle,
            HWND(0),
            0, // x
            0, // y
            width as i32,
            height as i32,
            SWP_NOMOVE,
        )
        .into()
    }
}

pub fn center_window(handle: HWND) -> bool {
    let mut rect = RECT::default();
    unsafe {
        if GetWindowRect(handle, &mut rect).as_bool() {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            SetWindowPos(
                handle,
                HWND(0),
                (screen_width / 2) - (rect.right - rect.left) / 2,
                (screen_height / 2) - (rect.bottom - rect.top) / 2,
                0, // cx
                0, // cy
                SWP_NOSIZE,
            )
            .into()
        } else {
            false
        }
    }
}
