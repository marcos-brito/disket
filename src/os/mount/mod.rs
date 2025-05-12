#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;

#[cfg(target_vendor = "apple")]
pub mod apple;

#[cfg(target_os = "freebsd")]
pub mod freebsd;
