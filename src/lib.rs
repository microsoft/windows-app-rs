#![doc(html_no_source)]

extern crate windows;

pub mod core;

#[allow(unused_variables)]
pub mod Microsoft;

#[cfg(feature = "WindowsAppSdk_Foundation")]
pub mod bootstrap;
