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
    pub fn new() -> Self {
        MountOptions::default()
    }

    pub fn device<T: AsRef<OsStr>>(&mut self, device: T) -> &mut Self {
        self.inner.device(device.as_ref().to_os_string());
        self
    }

    pub fn mount_point<T: AsRef<OsStr>>(&mut self, mount_point: T) -> &mut Self {
        self.inner.mount_point(mount_point.as_ref().to_os_string());
        self
    }

    pub fn mount(&self) -> Result<()> {
        sys::mount(&self.inner)
    }
}

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
    pub fn new() -> Self {
        UnmountOptions::default()
    }

    pub fn mount_point<T: AsRef<OsStr>>(&mut self, mount_point: T) -> &mut Self {
        self.inner.mount_point(mount_point.as_ref().to_os_string());
        self
    }

    pub fn unmount(&self) -> Result<()> {
        sys::unmount(&self.inner)
    }
}
pub fn mount<T: AsRef<OsStr>>(device: T, mount_point: T) -> Result<()> {
    MountOptions::new()
        .device(device)
        .mount_point(mount_point)
        .mount()
}

pub fn unmount<T: AsRef<OsStr>>(mount_point: T) -> Result<()> {
    UnmountOptions::new().mount_point(mount_point).unmount()
}
