//! Cross-platform volume/disk management library.
//!
//! # Support
//!
//! The following OSes are supported:
//!
//! - Linux
//! - Android
//! - FreeBSD
//! - macOS
//! - IOS
//! - Windows
//!
//! For unsupporterd systems `disket` will simply do nothing.
//!
//! # Features
//!
//! - `device`: Get information about devices
//! - `mount`: Mount and unmount file systems
//! - `watcher`: Watch for device changes
//! - `os`: Platform specific extensions and functions

mod common;
mod error;

pub use error::{Error, Result};

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "mount")]
pub mod mount;

#[cfg(feature = "watch")]
pub mod watch;

// #[cfg(feature = "os")]
pub mod os;
