#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Foundation")]
pub mod Foundation;
#[cfg(feature = "Graphics")]
pub mod Graphics;
#[cfg(feature = "UI")]
pub mod UI;
#[cfg(feature = "Web")]
pub mod Web;
#[cfg(feature = "Windows")]
pub mod Windows;
#[cfg(feature = "WindowsAppSdk")]
pub mod WindowsAppSdk;
