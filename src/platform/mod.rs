#[cfg(target_os = "linux")]
#[path="unix/mod.rs"]
pub mod platform;

#[cfg(target_os = "macos")]
#[path="unix/mod.rs"]
pub mod platform;

#[cfg(target_os = "unix")]
#[path="unix/mod.rs"]
pub mod platform;

#[cfg(target_os = "windows")]
#[path="win/mod.rs"]
pub mod platform;
