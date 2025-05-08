use std::{ffi::OsStr, iter, os::windows::ffi::OsStrExt};

pub trait Wide {
    fn wide(&self) -> Vec<u16>;
}

impl<T: AsRef<OsStr>> Wide for T {
    fn wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(iter::once(0)).collect()
    }
}
