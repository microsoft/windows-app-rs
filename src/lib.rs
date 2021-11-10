#![doc(html_no_source)]

extern crate windows;
pub mod Microsoft;

#[cfg(feature = "WindowsAppSdk_Foundation")]
pub mod bootstrap;
