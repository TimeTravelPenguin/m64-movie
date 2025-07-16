#![doc = include_str!("../README.md")]

pub mod doc;
pub mod parsed;
pub mod raw;
pub mod shared;

#[doc(inline)]
pub use parsed::Movie;

#[doc(inline)]
pub use raw::RawMovie;

/// Error type for movie operations.
#[derive(Debug, thiserror::Error)]
pub enum MovieError {
    #[error("Failed to read movie data: {0}")]
    BinRWError(#[from] binrw::Error),
    #[error("Failed to read file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse string: {0}")]
    FixedStrError(#[from] EncodedFixedStrError),
    #[error("Failed to parse movie: {0}")]
    MovieParseError(#[from] MovieParseError),
}

/// Error type for fixed-size string encoding and decoding.
#[derive(Debug, thiserror::Error)]
pub enum EncodedFixedStrError {
    #[error("Invalid UTF-8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Invalid ASCII string: {0}")]
    InvalidAscii(String),
    #[error("Fixed string error: {0}")]
    FixedStrError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum MovieParseError {
    #[error("Invalid movie version: {0}")]
    UnsupportedVersion(u32),
    #[error("Invalid movie extended version: {0}")]
    UnsupportedExtendedVersion(u8),
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
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error>;
}

/// Extensions for writing binary data.
pub trait BinWriteExt {
    type Error;
    /// Converts the instance to a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    /// Writes the instance to a binary file.
    fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Self::Error>;
}

/// An enum representing the buttons on a Mupen64 controller.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ControllerButton {
    /// The right directional pad button.
    DPadRight,
    /// The left directional pad button.
    DPadLeft,
    /// The down directional pad button.
    DPadDown,
    /// The up directional pad button.
    DPadUp,
    /// The start button.
    Start,
    /// The Z button.
    Z,
    /// The B button.
    B,
    /// The A button.
    A,
    /// The C-right button.
    CRight,
    /// The C-left button.
    CLeft,
    /// The C-down button.
    CDown,
    /// The C-up button.
    CUp,
    /// The right trigger button.
    TriggerRight,
    /// The left trigger button.
    TriggerLeft,
    /// Reserved button 01.
    Reserved01,
    /// Reserved button 02.
    Reserved02,
}
