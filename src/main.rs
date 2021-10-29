#![windows_subsystem = "windows"]

mod bootstrap;

use std::convert::TryFrom;

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        IWindowNative, LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
    Windows::Win32::{
        Foundation::{HWND, RECT},
        UI::{
            HiDpi::GetDpiForWindow,
            WindowsAndMessaging::{
                GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN,
                SWP_NOMOVE, SWP_NOSIZE,
            },
        },
    },
};

use windows::runtime::{implement, IInspectable, Interface};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn create() -> windows::runtime::Result<Application> {
        let app = App { window: None }.new()?;
        Ok(app)
    }

    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::runtime::Result<()> {
        let window = Window::new()?;
        window.SetTitle("WinUI Desktop, Unpackaged (Rust, WinAppSdk-preview3)")?;

        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.Click(RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button
                    .cast::<Button>()?
                    .SetContent(IInspectable::try_from("Clicked! 🦀")?)?;
            }
            Ok(())
        }))?;

        window.SetContent(&button)?;

        let native_window: IWindowNative = window.cast()?;
        let mut hwnd = HWND(0);
        unsafe {
            native_window
                .WindowHandle(&mut hwnd as *mut _)
                .expect("Failed to get native window handle");
        }

        resize_window(hwnd, 800, 600).then(|| {
            center_window(hwnd);
        });

        let result = window.Activate();
        self.window = Some(window);
        result
    }
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

fn main() -> windows::runtime::Result<()> {
    bootstrap::initialize()
        .and_then(|_| {
            Application::Start(ApplicationInitializationCallback::new(|_| {
                let _ = App::create();
                Ok(())
            }))
        })
        .and_then(|_| bootstrap::uninitialize())
}
