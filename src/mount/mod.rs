//! Mount and unmount operations.
//!
//! Cross-platform abstraction for mouting and unmouting file systems. Extra functionality
//! for specific platforms can be found at [`crate::os`].

#[cfg(windows)]
pub(crate) mod windows;
#[cfg(windows)]
use windows as sys;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(unix)]
use unix as sys;

use crate::Result;
use std::ffi::OsStr;

/// Options used to configure how the volume is mounted.
///
/// When using [`MountOptions`], you start by calling `new`. Every field
/// will be set to an initial value. That is, most of the time, not what you want.
/// You should chain calls to set every option and then call `mount`. Doing
/// otherwise will probably return an error.
///
/// Only `device` and `mount_point` are available in all platforms. For platform-specific
/// options use an extension trait such as `disket::os::mount::linux::MountOptionsExt`.
///
/// # Examples
///
/// Mount with no platform-specific options:
///
/// ```no_run
/// use disket::mount::MountOptions;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     MountOptions::new()
///         .device("/dev/sda2")
///         .mount_point("/mnt")
///         .mount()?;
///
///     Ok(())
/// }
/// ```
/// Mount with platform-specific options:
///
/// ```no_run
/// use disket::{mount::MountOptions, os::mount::linux::MountOptionsExt};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     MountOptions::new()
///         .device("/dev/sda2")
///         .mount_point("/mnt")
///         .fstype("ext4")
///         .mount()?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MountOptions {
    pub(crate) inner: sys::MountOptions,
}

impl Default for MountOptions {
    fn default() -> Self {
        Self {
            inner: sys::MountOptions::new(),
        }
    }
}

impl MountOptions {
    /// Creates a new set of options with default values.
    pub fn new() -> Self {
        MountOptions::default()
    }

    /// Sets the volume identifier to be mounted.
    pub fn device<T: AsRef<OsStr>>(&mut self, device: T) -> &mut Self {
        self.inner.device(device.as_ref().to_os_string());
        self
    }

    /// Sets the mount point of the volume to be mounted.
    pub fn mount_point<T: AsRef<OsStr>>(&mut self, mount_point: T) -> &mut Self {
        self.inner.mount_point(mount_point.as_ref().to_os_string());
        self
    }

    /// Mounts a volume with the options specified by `self`.
    ///
    /// See [`mount`] for details.
    pub fn mount(&self) -> Result<()> {
        sys::mount(&self.inner)
    }
}

/// Options used to configure how the volume is unmounted.
///
/// It's very similar to [`MountOptions`]. Start by calling `new`, which
/// will return an instance with empty values, chain calls to fill
/// the options and then call `unmount`.
///
/// `mount_point` is the only option available in all platforms. Use an
/// extension crate, i.g `disket::os::mount::apple::UnmountOptionsExt` for
/// platform-specific options.
///
/// # Examples
///
/// Unmount with no platform-specific options:
///
/// ```
/// use disket::mount::UnmountOptions;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     UnmountOptions::new()
///         .mount_point("/mnt")
///         .unmount()?;
///
///     Ok(())
/// }
/// ```
///
/// Unmount with platform-specific options:
///
/// ```
/// use disket::{mount::UnmountOptions, os::mount::apple::{UnmountOptionsExt, MntFlags}};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     UnmountOptions::new()
///         .mount_point("/mnt")
///         .flags(MntFlags::MNT_FORCE | MntFlags::MNT_NOBLOCK)
///         .unmount()?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnmountOptions {
    pub(crate) inner: sys::UnmountOptions,
}

impl Default for UnmountOptions {
    fn default() -> Self {
        Self {
            inner: sys::UnmountOptions::new(),
        }
    }
}

impl UnmountOptions {
    /// Creates a new set of options with default values.
    pub fn new() -> Self {
        UnmountOptions::default()
    }

    /// Sets the mount point of the volume to be unmounted.
    pub fn mount_point<T: AsRef<OsStr>>(&mut self, mount_point: T) -> &mut Self {
        self.inner.mount_point(mount_point.as_ref().to_os_string());
        self
    }

    /// Unmounts a volume with the options specified by `self`
    ///
    /// See [`unmount`] for details.
    pub fn unmount(&self) -> Result<()> {
        sys::unmount(&self.inner)
    }
}

/// Mounts the file system pointed by `volume` at `mount_point`.
///
/// This function is interchangeable with manually calling `mount` on [`MountOptions`]
/// with `device` and `mount_point` set.
///
/// # Platform-specific behaviour
///
/// On all platforms no extra data is passed by default. Every flag and option is set to its
/// null equivalent.
///
/// On Windows, this function corresponds to `SetVolumeMountPointW`.
///
/// On Linux and Android, this function corresponds to the `mount` syscall.
///
/// On FreeBSD , this function corresponds to the `nmount` syscall.
///
/// On MacOS and IOS , this function corresponds to the `mount` syscall.
///
/// # Errors
///
/// Every error is returned from the underlying platform, that is `errno` on *nix systems or whatever
/// windows bindings returns. For detailed reasoning consult its respective reference.
///
/// # Examples
///
/// ```
/// use disket::mount;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     mount::mount("/dev/sdc1", "/mnt")?; // Mount /dev/sdc1 at /mnt
///     Ok(())
/// }
/// ```
///
/// # References
///
/// Details for each of the underlying platform calls can be found at:
///
/// - [Linux/Android]
/// - [FreeBSD]
/// - [macOS/IOS]
/// - [Windows]
///
/// [Linux/Android]: https://www.man7.org/linux/man-pages/man2/mount.2.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?nmount(2)
/// [macOS/IOS]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mount.2.html
/// [Windows]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setvolumemountpointw
pub fn mount<T: AsRef<OsStr>>(device: T, mount_point: T) -> Result<()> {
    MountOptions::new()
        .device(device)
        .mount_point(mount_point)
        .mount()
}

/// Unmounts the file system pointed by `mount_point`.
///
/// This function is interchangeable with manually calling `unmount` on [`UnmountOptions`]
/// with `mount_point` set.
///
/// # Platform-specific behaviour
///
/// On all platforms no extra data is passed by default. Every flag and option is set to its
/// null equivalent.
///
/// On Windows, this function corresponds to `DeleteVolumeMountPointW`.
///
/// On Linux and Android, this function corresponds to the `umount2` syscall.
///
/// On FreeBSD, MacOS and IOS , this function corresponds to the `unmount` syscall.
///
/// # Errors
///
/// Every error is returned from the underlying platform, that is `errno` on *nix systems or whatever
/// windows bindings returns. For detailed reasoning consult its respective reference.
///
/// # Examples
///
/// ```
/// use disket::mount;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     mount::unmount("/mnt")?; // Unmount volume mounted at /mnt
///     Ok(())
/// }
/// ```
///
/// # References
///
/// Details for each of the underlying platform calls can be found at:
///
/// - [Linux/Android]
/// - [FreeBSD/macOS/IOS]
/// - [Windows]
///
/// [Linux/Android]: https://www.man7.org/linux/man-pages/man2/umount.2.html
/// [FreeBSD/macOS/IOS]: https://man.freebsd.org/cgi/man.cgi?query=unmount
/// [Windows]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletevolumemountpointw
pub fn unmount<T: AsRef<OsStr>>(mount_point: T) -> Result<()> {
    UnmountOptions::new().mount_point(mount_point).unmount()
}
