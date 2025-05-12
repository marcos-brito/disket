#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;

#[cfg(target_vendor = "apple")]
pub mod apple;
