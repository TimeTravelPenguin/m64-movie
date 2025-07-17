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

/// A marker type for ASCII encoded strings.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ascii;

/// A marker type for UTF-8 encoded strings.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Utf8;

/// An enum representing a fixed-size string that can be either ASCII or UTF-8 encoded.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EncodedFixedStr<const N: usize, E> {
    /// The underlying fixed-size string value.
    value: zstr<N>,
    /// A marker to indicate the encoding type.
    _marker: std::marker::PhantomData<E>,
}

impl<const N: usize, E> Display for EncodedFixedStr<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A trait for fixed-size strings that can be constructed from bytes or strings.
pub trait FixedString
where
    Self: Sized,
{
    /// The error type returned by the methods of this trait.
    type Error;

    /// Creates a new instance from bytes.
    fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Self::Error>;
    /// Creates a new instance from a string slice.
    fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error>;
}

impl<const N: usize> EncodedFixedStr<N, Utf8> {
    /// Creates a new `EncodedFixedStr` from a UTF-8 string.
    pub fn from_utf8<B: AsRef<[u8]>>(bytes: B) -> Result<Self, MovieError> {
        let s = str::from_utf8(bytes.as_ref()).map_err(EncodedFixedStrError::Utf8Error)?;

        Self::from_utf8_str(s)
    }

    /// Creates a new `EncodedFixedStr` from a UTF-8 string slice.
    pub fn from_utf8_str<S: AsRef<str>>(s: S) -> Result<Self, MovieError> {
        Ok(EncodedFixedStr {
            value: zstr::try_make(s.as_ref())
                .map_err(|err: &str| EncodedFixedStrError::ZStrError(err.to_string()))?,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<const N: usize> FixedString for EncodedFixedStr<N, Utf8> {
    type Error = MovieError;

    fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Self::Error> {
        Self::from_utf8(bytes)
    }

    fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
        Self::from_utf8_str(s)
    }
}

impl<const N: usize> EncodedFixedStr<N, Ascii> {
    /// Creates a new `EncodedFixedStr` from an ASCII string.
    pub fn from_ascii<B: AsRef<[u8]>>(bytes: B) -> Result<Self, MovieError> {
        Self::from_ascii_str(
            str::from_utf8(bytes.as_ref()).map_err(EncodedFixedStrError::Utf8Error)?,
        )
    }

    /// Creates a new `EncodedFixedStr` from an ASCII string slice.
    pub fn from_ascii_str<S: AsRef<str>>(s: S) -> Result<Self, MovieError> {
        let s = s.as_ref();

        if !s.is_ascii() {
            return Err(EncodedFixedStrError::InvalidAscii(s.to_string()).into());
        }

        Ok(EncodedFixedStr {
            value: zstr::try_make(s)
                .map_err(|arg0: &str| EncodedFixedStrError::ZStrError(arg0.to_string()))?,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<const N: usize> FixedString for EncodedFixedStr<N, Ascii> {
    type Error = MovieError;

    fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Self::Error> {
        Self::from_ascii(bytes)
    }

    fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
        Self::from_ascii_str(s)
    }
}

impl<const N: usize, E, Err> TryFrom<&str> for EncodedFixedStr<N, E>
where
    EncodedFixedStr<N, E>: FixedString<Error = Err>,
{
    type Error = Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EncodedFixedStr::<N, E>::from_str(value)
    }
}

impl<const N: usize, E, Err> TryFrom<String> for EncodedFixedStr<N, E>
where
    EncodedFixedStr<N, E>: FixedString<Error = Err>,
{
    type Error = Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EncodedFixedStr::<N, E>::from_str(value)
    }
}

impl<const N: usize, E> From<EncodedFixedStr<N, E>> for NullString {
    fn from(encoded: EncodedFixedStr<N, E>) -> Self {
        encoded.value.as_str().into()
    }
}
