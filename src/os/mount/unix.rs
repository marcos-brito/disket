pub trait MountOptionsExt {
    fn flags(&mut self) -> &mut Self;
}

impl MountOptionsExt for crate::mount::MountOptions {
    fn flags(&mut self) -> &mut Self {
        self
    }
}
