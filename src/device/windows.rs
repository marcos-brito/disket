use std::ffi::{OsStr, OsString};

pub struct DeviceInner {
    label: OsString,
}

impl DeviceInner {
    pub fn label(&self) -> &OsStr {
        &self.label
    }
}
