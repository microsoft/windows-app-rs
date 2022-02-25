#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[cfg(feature = "Windows_AppLifecycle")]
pub mod AppLifecycle;
#[cfg(feature = "Windows_ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Windows_System")]
pub mod System;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
