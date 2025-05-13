pub use nix::mount::MntFlags;
use std::ffi::OsStr;

/// macOS/IOS specific extensions for [`crate::mount::MountOptions`]
pub trait MountOptionsExt {
    /// Sets flags to modify the behaviour of `mount`.
    fn flags(&mut self, flags: MntFlags) -> &mut Self;
    /// Sets filesystem specific data.
    ///
    /// This is usually a comma-separated list.
    fn data<T: AsRef<OsStr>>(&mut self, data: Option<T>) -> &mut Self;
}

impl MountOptionsExt for crate::mount::MountOptions {
    fn flags(&mut self, flags: MntFlags) -> &mut Self {
        self.inner.flags(flags);
        self
    }

    fn data<T: AsRef<OsStr>>(&mut self, data: Option<T>) -> &mut Self {
        self.inner.data(data.map(|d| d.as_ref().to_os_string()));
        self
    }
}

/// macOS/IOS specific extensions for [`crate::mount::UnmountOptions`]
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
