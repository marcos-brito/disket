pub use nix::mount::MntFlags;
use std::ffi::OsStr;

pub trait MountOptionsExt {
    fn flags(&mut self, flags: MntFlags) -> &mut Self;
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

pub trait UnmountOptionsExt {
    fn flags(&mut self, flags: MntFlags) -> &mut Self;
}

impl UnmountOptionsExt for crate::mount::UnmountOptions {
    fn flags(&mut self, flags: MntFlags) -> &mut Self {
        self.inner.flags(flags);
        self
    }
}
