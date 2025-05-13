use crate::Result;
use nix::mount::{self, MntFlags};
use std::ffi::OsString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MountOptions {
    volume: OsString,
    mount_point: OsString,
    flags: MntFlags,
    fs_type: Option<OsString>,
    data: Option<OsString>,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            volume: OsString::new(),
            mount_point: OsString::new(),
            flags: MntFlags::empty(),
            fs_type: None,
            data: None,
        }
    }

    pub fn volume(&mut self, volume: OsString) -> &mut Self {
        self.volume = volume;
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
    let mut mount = mount::Nmount::new();

    mount
        .str_opt_owned("target", options.volume.as_os_str())
        .str_opt_owned("fspath", options.mount_point.as_os_str());

    if let Some(fstype) = &options.fs_type {
        mount.str_opt_owned("fstype", fstype.as_os_str());
    }

    mount.nmount(options.flags).map_err(|e| e.error())?;

    Ok(())
}

pub fn unmount(options: &UnmountOptions) -> Result<()> {
    mount::unmount(options.mount_point.as_os_str(), options.flags)?;
    Ok(())
}
