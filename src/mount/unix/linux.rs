use crate::Result;
use nix::mount::{self, MntFlags, MsFlags};
use std::ffi::{OsStr, OsString};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MountOptions {
    volume: OsString,
    mount_point: OsString,
    flags: MsFlags,
    fs_type: Option<OsString>,
    data: Option<OsString>,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            volume: OsString::new(),
            mount_point: OsString::new(),
            flags: MsFlags::empty(),
            fs_type: None,
            data: None,
        }
    }

    pub fn volume(&mut self, device: OsString) -> &mut Self {
        self.volume = device;
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
        Some(options.volume.as_os_str()),
        options.mount_point.as_os_str(),
        options.fs_type.as_ref().map(|t| t.as_os_str()),
        options.flags,
        options.data.as_ref().map(|t| t.as_os_str()),
    )?;

    Ok(())
}

pub fn unmount(options: &UnmountOptions) -> Result<()> {
    mount::umount2(options.mount_point.as_os_str(), options.flags)?;
    Ok(())
}

pub fn umount(target: &OsStr) -> Result<()> {
    mount::umount(target)?;
    Ok(())
}
