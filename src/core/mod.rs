#![allow(non_snake_case)]
#![cfg(feature = "UI")]

use crate::Microsoft::UI::{DisplayId, IconId, WindowId};
use std::convert::TryInto;
use windows::{
    core::{IntoParam, Param},
    Win32::{Foundation::HWND, Graphics::Gdi::HMONITOR, UI::WindowsAndMessaging::HICON},
};

impl From<HWND> for WindowId {
    fn from(hwnd: HWND) -> Self {
        WindowId {
            Value: hwnd.0.try_into().unwrap(),
        }
    }
}

impl From<WindowId> for HWND {
    fn from(window_id: WindowId) -> Self {
        HWND(window_id.Value.try_into().unwrap())
    }
}

impl<'a> IntoParam<'a, HWND> for WindowId {
    fn into_param(self) -> Param<'a, HWND> {
        IntoParam::into_param(TryInto::<HWND>::try_into(self).unwrap())
    }
}

impl<'a> IntoParam<'a, WindowId> for HWND {
    fn into_param(self) -> Param<'a, WindowId> {
        IntoParam::into_param(TryInto::<WindowId>::try_into(self).unwrap())
    }
}

impl From<DisplayId> for HMONITOR {
    fn from(display_id: DisplayId) -> Self {
        HMONITOR(display_id.Value.try_into().unwrap())
    }
}

impl From<HMONITOR> for DisplayId {
    fn from(handle: HMONITOR) -> Self {
        DisplayId {
            Value: handle.0.try_into().unwrap(),
        }
    }
}

impl<'a> IntoParam<'a, HMONITOR> for DisplayId {
    fn into_param(self) -> Param<'a, HMONITOR> {
        IntoParam::into_param(TryInto::<HMONITOR>::try_into(self).unwrap())
    }
}

impl<'a> IntoParam<'a, DisplayId> for HMONITOR {
    fn into_param(self) -> Param<'a, DisplayId> {
        IntoParam::into_param(TryInto::<DisplayId>::try_into(self).unwrap())
    }
}

impl From<IconId> for HICON {
    fn from(icon_id: IconId) -> Self {
        HICON(icon_id.Value.try_into().unwrap())
    }
}

impl From<HICON> for IconId {
    fn from(handle: HICON) -> Self {
        IconId {
            Value: handle.0.try_into().unwrap(),
        }
    }
}

impl<'a> IntoParam<'a, HICON> for IconId {
    fn into_param(self) -> Param<'a, HICON> {
        IntoParam::into_param(TryInto::<HICON>::try_into(self).unwrap())
    }
}

impl<'a> IntoParam<'a, IconId> for HICON {
    fn into_param(self) -> Param<'a, IconId> {
        IntoParam::into_param(TryInto::<IconId>::try_into(self).unwrap())
    }
}
