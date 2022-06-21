#![doc(html_no_source)]

extern crate windows;

pub mod core;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(clippy::derivable_impls)]
pub mod Microsoft;

#[cfg(feature = "WindowsAppSdk_Foundation")]
pub mod bootstrap;

pub mod build;
