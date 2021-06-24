pub mod bootstrap;

use windows::{IInspectable, Interface, HRESULT};

#[repr(transparent)]
struct IWindowNative(windows::IUnknown);

#[allow(dead_code)]
impl IWindowNative {
    pub fn handle(&self) -> Option<isize> {
        unsafe {
            let mut hwnd = 0;
            match self.WindowHandle(&mut hwnd as *mut isize) {
                HRESULT(0) => Some(hwnd),
                _ => None,
            }
        }
    }

    #[allow(non_snake_case)]
    unsafe fn WindowHandle(&self, hwnd: *mut isize) -> windows::HRESULT {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), hwnd)
    }
}

#[repr(C)]
struct IWindowNative_vtbl(
    // IUnknown::QueryInterface
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::HRESULT,
    // IUnknown::AddRef
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    // IUnknown::Release
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    // HRESULT WindowHandle(HWND* hWnd);
    pub unsafe extern "system" fn(this: ::windows::RawPtr, hwnd: *const isize) -> ::windows::HRESULT,
);

unsafe impl ::windows::Interface for IWindowNative {
    type Vtable = IWindowNative_vtbl;
    // {eecdbf0e-bae9-4cb6-a68e-9598e1cb57bb}
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        0xeecdbf0e,
        0xbae9,
        0x4cb6,
        [0xa6, 0x8e, 0x95, 0x98, 0xe1, 0xcb, 0x57, 0xbb],
    );
}

pub fn window_handle(window: &IInspectable) -> Option<isize> {
    window.cast::<IWindowNative>().unwrap().handle()
}
