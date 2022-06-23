#[cfg(feature = "DirectWriteCore")]
pub mod DirectWriteCore;
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
