#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod doc;
pub mod parsed;
pub mod raw;
pub mod shared;

#[doc(inline)]
pub use parsed::Movie;

#[doc(inline)]
pub use raw::RawMovie;

/// Error type for [`RawMovie`] operations.
#[derive(Debug, thiserror::Error)]
pub enum MovieError {
    /// Error when reading or writing binary data.
    #[error("Failed to read movie data: {0}")]
    BinRWError(#[from] binrw::Error),
    /// Error when reading or writing files.
    #[error("Failed to read file: {0}")]
    FileError(#[from] std::io::Error),
    /// Error when parsing a [`EncodedFixedStr`](`shared::EncodedFixedStr`).
    #[error("Failed to parse string: {0}")]
    FixedStrError(#[from] EncodedFixedStrError),
    /// Error when parsing a [`Movie`].
    #[error("Failed to parse movie: {0}")]
    MovieParseError(#[from] MovieParseError),
}

/// Error type for [`EncodedFixedStr`](`shared::EncodedFixedStr`) encoding and decoding.
#[derive(Debug, thiserror::Error)]
pub enum EncodedFixedStrError {
    /// Error when the byte slice is not valid UTF-8.
    #[error("Invalid UTF-8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Error when the string is not valid ASCII.
    #[error("Invalid ASCII string: {0}")]
    InvalidAscii(String),
    /// Errors related to [`fixedstr::zstr`].
    #[error("Fixed string error: {0}")]
    ZStrError(String),
}

/// Error type for [`Movie`] parsing errors.
#[derive(Debug, thiserror::Error)]
pub enum MovieParseError {
    /// Error when the movie file has an invalid or unsupported version.
    #[error("Invalid movie version: {0}")]
    UnsupportedVersion(u32),
    /// Error when the movie file has an invalid or unsupported extended version.
    #[error("Invalid movie extended version: {0}")]
    UnsupportedExtendedVersion(u8),
}

/// Extensions for reading binary data.
pub trait BinReadExt
where
    Self: Sized,
{
    /// The error type returned by the reading methods.
    type Error;
    /// Reads the binary data from a byte slice.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
    /// Reads the binary data from a file.
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error>;
}

/// Extensions for writing binary data.
pub trait BinWriteExt {
    /// The error type returned by the writing methods.
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
