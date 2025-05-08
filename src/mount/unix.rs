use nix::{
    mount::{self, MsFlags},
    Result,
};
use std::ffi::OsString;

pub struct MountOptions {
    device: OsString,
    mount_point: OsString,
    flags: MsFlags,
    fs_type: Option<OsString>,
    data: Option<OsString>,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            device: OsString::new(),
            mount_point: OsString::new(),
            flags: MsFlags::empty(),
            fs_type: None,
            data: None,
        }
    }

    pub fn device(&mut self, device: OsString) -> &mut Self {
        self.device = device;
        self
    }

    pub fn mount_point(&mut self, mount_point: OsString) -> &mut Self {
        self.mount_point = mount_point;
        self
    }

    pub fn flags(&mut self, flags: MsFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn fs_type(&mut self, fs_type: Option<OsString>) -> &mut Self {
        self.fs_type = fs_type;
        self
    }

    pub fn data(&mut self, data: Option<OsString>) -> &mut Self {
        self.data = data;
        self
    }
}

pub fn mount(options: MountOptions) -> Result<()> {
    mount::mount(
        Some(options.device.as_os_str()),
        options.mount_point.as_os_str(),
        options.fs_type.as_ref().map(|t| t.as_os_str()),
        options.flags,
        options.data.as_ref().map(|t| t.as_os_str()),
    )
}
