#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[cfg(feature = "Windows_ApplicationModel_DynamicDependency")]
pub mod DynamicDependency;
#[cfg(feature = "Windows_ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "Windows_ApplicationModel_WindowsAppRuntime")]
pub mod WindowsAppRuntime;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
