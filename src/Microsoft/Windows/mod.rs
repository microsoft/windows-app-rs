#[cfg(feature = "Windows_AppLifecycle")]
pub mod AppLifecycle;
#[cfg(feature = "Windows_AppNotifications")]
pub mod AppNotifications;
#[cfg(feature = "Windows_ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Windows_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Windows_System")]
pub mod System;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
