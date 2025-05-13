//! Platform-specific extensions
//!
//! Provides access to platform-specific information and functionality, supporting tasks that a
//! platform-agnostic abstraction layer can not. You shouldn't expect any extensive documentation
//! for each platform here. Check specific `man pages` and `APIs` for that.

#[cfg(feature = "mount")]
pub mod mount;
