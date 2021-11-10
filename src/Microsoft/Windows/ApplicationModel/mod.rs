#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Windows_ApplicationModel_DynamicDependency")]
pub mod DynamicDependency;
#[cfg(feature = "Windows_ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "Windows_ApplicationModel_WindowsAppRuntime")]
pub mod WindowsAppRuntime;
