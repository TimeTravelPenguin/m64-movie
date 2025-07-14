//! Shared types and traits for binary reading and writing.

use std::fmt::{self, Debug};

use binrw::{BinRead, BinWrite};

/// A struct implementing [`BinRead`] and [`BinWrite`] for reserved space in bytes.
#[derive(Copy, Clone, Eq, PartialEq, BinRead, BinWrite)]
pub struct Reserved<const T: usize> {
    /// A span of `T` reserved bytes.
    pub reserved: [u8; T],
}

impl<const T: usize> Debug for Reserved<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reserved({} bytes)", T)
    }
}
