use std::ffi::{OsStr, OsString};

pub struct MountOptions {
    device: OsString,
    mount_point: OsString,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            device: OsString::new(),
            mount_point: OsString::new(),
        }
    }

    pub fn device(&mut self, device: &OsStr) -> &mut Self {
        self.device = device.to_os_string();
        self
    }

    pub fn mount_point(&mut self, mount_point: &OsStr) -> &mut Self {
        self.device = mount_point.to_os_string();
        self
    }
}

pub fn mount(options: MountOptions) {
    todo!()
}
