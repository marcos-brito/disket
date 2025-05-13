use crate::mount::unix::linux;
use crate::Result;
pub use nix::mount::{MntFlags, MsFlags};
use std::ffi::OsStr;

/// Linux specific extensions for [`crate::mount::MountOptions`]
pub trait MountOptionsExt {
    /// Sets flags to modify the behaviour of `mount`.
    fn flags(&mut self, flags: MsFlags) -> &mut Self;
    /// Sets the filesystem type.
    ///
    /// This is typically a member of a set of filesystems the OS supports.
    fn fs_type<T: AsRef<OsStr>>(&mut self, fs_type: Option<T>) -> &mut Self;
    /// Sets filesystem specific data.
    ///
    /// This is usually a comma-separated list.
    fn data<T: AsRef<OsStr>>(&mut self, data: Option<T>) -> &mut Self;
}

impl MountOptionsExt for crate::mount::MountOptions {
    fn flags(&mut self, flags: MsFlags) -> &mut Self {
        self.inner.flags(flags);
        self
    }

    fn fs_type<T: AsRef<OsStr>>(&mut self, fs_type: Option<T>) -> &mut Self {
        self.inner
            .fs_type(fs_type.map(|t| t.as_ref().to_os_string()));
        self
    }

    fn data<T: AsRef<OsStr>>(&mut self, data: Option<T>) -> &mut Self {
        self.inner.data(data.map(|d| d.as_ref().to_os_string()));
        self
    }
}

/// Linux specific extensions for [`crate::mount::UnmountOptions`]
pub trait UnmountOptionsExt {
    /// Sets flags to modify the behaviour of `unmount`.
    fn flags(&mut self, flags: MntFlags) -> &mut Self;
}

impl UnmountOptionsExt for crate::mount::UnmountOptions {
    fn flags(&mut self, flags: MntFlags) -> &mut Self {
        self.inner.flags(flags);
        self
    }
}

/// Equivalent to the `umount` syscall.
///
/// Unlike [`crate::mount::mount`], which is equivalent to the `umount2` syscall, this one takes no flags.
pub fn umount<T: AsRef<OsStr>>(target: T) -> Result<()> {
    linux::umount(target.as_ref())
}
