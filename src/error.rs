use std::result;

pub type Result<T> = result::Result<T, Error>;

// TODO: write actual messages
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(windows)]
    #[error("")]
    Platform(#[from] windows::core::Error),
    #[cfg(unix)]
    #[error("")]
    Platform(#[from] nix::Error),
}
