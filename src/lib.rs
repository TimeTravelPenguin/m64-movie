#![doc = include_str!("../README.md")]

pub mod doc;
pub mod movie;
pub mod parsed;
pub mod raw;
pub mod shared;

use std::path::Path;

use thiserror::Error;

#[doc(inline)]
use parsed::m64::Movie;
#[doc(inline)]
pub use raw::m64::{
    ControllerFlags, ControllerState, ExtendedData, ExtendedFlags, MovieStartType, RawMovie,
};

#[derive(Debug, Error)]
pub enum MovieError {
    #[error("Failed to read movie data: {0}")]
    BinRWError(#[from] binrw::Error),
    #[error("Failed to read file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse string: {0}")]
    StringError(#[from] shared::EncodedFixedStrError),
}

/// Extensions for reading binary data.
pub trait BinReadExt
where
    Self: Sized,
{
    type Error;
    /// Reads the binary data from a byte slice.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
    /// Reads the binary data from a file.
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
}

/// Extensions for writing binary data.
pub trait BinWriteExt {
    type Error;
    /// Converts the instance to a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    /// Writes the instance to a binary file.
    fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
}
