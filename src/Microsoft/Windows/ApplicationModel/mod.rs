#[cfg(feature = "Windows_ApplicationModel_DynamicDependency")]
pub mod DynamicDependency;
#[cfg(feature = "Windows_ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "Windows_ApplicationModel_WindowsAppRuntime")]
pub mod WindowsAppRuntime;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
