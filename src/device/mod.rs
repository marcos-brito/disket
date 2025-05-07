#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as sys;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

use std::ffi::OsStr;

pub struct Device {
    inner: sys::DeviceInner,
}

impl Device {
    pub fn name(&self) -> &OsStr {
        todo!();
    }

    pub fn label(&self) -> &OsStr {
        self.inner.label()
    }

    pub fn mount_point(&self) -> &OsStr {
        todo!();
    }

    pub fn file_system(&self) -> &OsStr {
        todo!();
    }

    pub fn total(&self) -> usize {
        todo!();
    }

    pub fn available(&self) -> usize {
        todo!();
    }
}
