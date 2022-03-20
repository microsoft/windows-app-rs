#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[cfg(feature = "Foundation")]
pub mod Foundation;
#[cfg(feature = "Graphics")]
pub mod Graphics;
#[cfg(feature = "MRM")]
pub mod MRM;
#[cfg(feature = "UI")]
pub mod UI;
#[cfg(feature = "Web")]
pub mod Web;
#[cfg(feature = "Windows")]
pub mod Windows;
#[cfg(feature = "WindowsAppSdk")]
pub mod WindowsAppSdk;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
