use crate::common::windows::Wide;
use crate::Result;
use std::ffi::{OsStr, OsString};
use windows::{core::PCWSTR, Win32::Storage::FileSystem};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MountOptions {
    volume: OsString,
    mount_point: OsString,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            volume: OsString::new(),
            mount_point: OsString::new(),
        }
    }

    pub fn volume(&mut self, device: OsString) -> &mut Self {
        self.volume = device;
        self
    }

    pub fn mount_point(&mut self, mount_point: OsString) -> &mut Self {
        self.volume = mount_point;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnmountOptions {
    mount_point: OsString,
}

impl UnmountOptions {
    pub fn new() -> Self {
        Self {
            mount_point: OsString::new(),
        }
    }

    pub fn mount_point(&mut self, mount_point: OsString) -> &mut Self {
        self.mount_point = mount_point;
        self
    }
}

pub fn mount(options: &MountOptions) -> Result<()> {
    let volume = PCWSTR::from_raw(options.volume.wide().as_ptr());
    let mount_point = PCWSTR::from_raw(options.mount_point.wide().as_ptr());

    unsafe {
        FileSystem::SetVolumeMountPointW(volume, mount_point)?;
    }

    Ok(())
}

pub fn unmount(options: &UnmountOptions) -> Result<()> {
    let volume = PCWSTR::from_raw(options.mount_point.wide().as_ptr());

    unsafe {
        FileSystem::DeleteVolumeMountPointW(volume)?;
    }

    Ok(())
}
