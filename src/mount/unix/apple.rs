use crate::Result;
use nix::mount::{self, MntFlags};
use std::ffi::OsString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MountOptions {
    device: OsString,
    mount_point: OsString,
    flags: MntFlags,
    data: Option<OsString>,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            device: OsString::new(),
            mount_point: OsString::new(),
            flags: MntFlags::empty(),
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

    pub fn flags(&mut self, flags: MntFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn data(&mut self, data: Option<OsString>) -> &mut Self {
        self.data = data;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnmountOptions {
    mount_point: OsString,
    flags: MntFlags,
}

impl UnmountOptions {
    pub fn new() -> Self {
        Self {
            mount_point: OsString::new(),
            flags: MntFlags::empty(),
        }
    }

    pub fn mount_point(&mut self, mount_point: OsString) -> &mut Self {
        self.mount_point = mount_point;
        self
    }

    pub fn flags(&mut self, flags: MntFlags) -> &mut Self {
        self.flags = flags;
        self
    }
}

pub fn mount(options: &MountOptions) -> Result<()> {
    mount::mount(
        options.device.as_os_str(),
        options.mount_point.as_os_str(),
        options.flags,
        options.data.as_ref().map(|t| t.as_os_str()),
    )?;

    Ok(())
}

pub fn unmount(options: &UnmountOptions) -> Result<()> {
    mount::unmount(options.mount_point.as_os_str(), options.flags)?;
    Ok(())
}
