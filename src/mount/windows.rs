use crate::common::windows::Wide;
use crate::Result;
use std::ffi::{OsStr, OsString};
use windows::{core::PCWSTR, Win32::Storage::FileSystem};


pub struct MountOptions {
    volume_name: OsString,
    mount_point: OsString,
}

impl MountOptions {
    pub fn new() -> Self {
        Self {
            volume_name: OsString::new(),
            mount_point: OsString::new(),
        }
    }

    pub fn device(&mut self, device: OsString) -> &mut Self {
        self.volume_name = device;
        self
    }

    pub fn mount_point(&mut self, mount_point: OsString) -> &mut Self {
        self.volume_name = mount_point;
        self
    }
}

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
    let volume_name = PCWSTR::from_raw(options.volume_name.wide().as_ptr());
    let mount_point = PCWSTR::from_raw(options.mount_point.wide().as_ptr());

    unsafe {
        FileSystem::SetVolumeMountPointW(volume_name, mount_point)?;
    }

    Ok(())
}

pub fn unmount(options: &UnmountOptions) -> Result<()> {
    let volume_name = PCWSTR::from_raw(options.mount_point.wide().as_ptr());

    unsafe {
        FileSystem::DeleteVolumeMountPointW(volume_name)?;
    }

    Ok(())
}
