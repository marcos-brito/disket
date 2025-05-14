#![allow(dead_code)]
#![allow(unused)]

use std::ffi::OsStr;

pub struct Device {}

impl Device {
    pub fn name(&self) -> &OsStr {
        todo!();
    }

    pub fn label(&self) -> &OsStr {
        todo!();
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
