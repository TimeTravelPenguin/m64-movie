//! Shared types and traits for binary reading and writing.

use std::fmt::{self, Debug, Display};

use binrw::{BinRead, BinWrite, NullString};
use fixedstr::zstr;

use crate::{EncodedFixedStrError, MovieError};

/// A struct implementing [`BinRead`] and [`BinWrite`] for reserved space in bytes.
#[derive(Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
pub struct Reserved<const T: usize> {
    /// A span of `T` reserved bytes.
    pub reserved: [u8; T],
}

impl<const T: usize> Default for Reserved<T> {
    fn default() -> Self {
        Reserved { reserved: [0; T] }
    }
}

impl<const T: usize> Debug for Reserved<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved({} bytes)", T)
    }
}

/// A fixed-size string type that can hold a string of `N` bytes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct FixedStr<const N: usize>(zstr<N>);

impl<const N: usize> FixedStr<N> {
    /// Creates a new `FixedStr`.
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, String> {
        Self::try_from(s.as_ref())
    }
}

impl<const N: usize> Display for FixedStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const N: usize> TryFrom<&str> for FixedStr<N> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let zstr = zstr::try_make(value)?;
        Ok(FixedStr(zstr))
    }
}

impl<const N: usize> TryFrom<String> for FixedStr<N> {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        FixedStr::try_from(s.as_str())
    }
}

impl<const N: usize> TryFrom<NullString> for FixedStr<N> {
    type Error = String;

    fn try_from(value: NullString) -> Result<Self, Self::Error> {
        let zstr = zstr::try_make(&value.to_string())?;
        Ok(FixedStr(zstr))
    }
}

impl<const N: usize> From<FixedStr<N>> for NullString {
    fn from(val: FixedStr<N>) -> Self {
        NullString::from(val.0.to_string())
    }
}

/// An enum representing a fixed-size string that can be either ASCII or UTF-8 encoded.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EncodedFixedStr<const N: usize> {
    Ascii(FixedStr<N>),
    Utf8(FixedStr<N>),
}

impl<const N: usize> Display for EncodedFixedStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodedFixedStr::Ascii(s) => write!(f, "{}", s),
            EncodedFixedStr::Utf8(s) => write!(f, "{}", s),
        }
    }
}

impl<const N: usize> EncodedFixedStr<N> {
    /// Creates a new `EncodedFixedStr` from a UTF-8 string.
    pub fn from_utf8<B: AsRef<[u8]>>(bytes: B) -> Result<Self, MovieError> {
        let s = str::from_utf8(bytes.as_ref()).map_err(EncodedFixedStrError::Utf8Error)?;

        Ok(EncodedFixedStr::Utf8(
            FixedStr::new(s).map_err(EncodedFixedStrError::FixedStrError)?,
        ))
    }

    /// Creates a new `EncodedFixedStr` from a UTF-8 string slice.
    pub fn from_utf8_str<S: Into<String>>(s: S) -> Result<Self, MovieError> {
        EncodedFixedStr::from_utf8(s.into().as_bytes())
    }

    /// Creates a new `EncodedFixedStr` from an ASCII string.
    pub fn from_ascii<B: AsRef<[u8]>>(bytes: B) -> Result<Self, MovieError> {
        let s = str::from_utf8(bytes.as_ref())
            .map_err(EncodedFixedStrError::Utf8Error)?
            .to_string();

        if !s.is_ascii() {
            return Err(EncodedFixedStrError::InvalidAscii(s).into());
        }

        Ok(EncodedFixedStr::Ascii(
            FixedStr::new(s).map_err(EncodedFixedStrError::FixedStrError)?,
        ))
    }

    /// Creates a new `EncodedFixedStr` from an ASCII string slice.
    pub fn from_ascii_str<S: Into<String>>(s: S) -> Result<Self, MovieError> {
        Self::from_ascii(s.into().as_bytes())
    }
}
