#![doc = include_str!("../README.md")]

pub mod doc;
pub mod movie;
pub mod raw;
pub mod shared;

use std::{fs::File, io::Cursor, path::Path};

use binrw::{BinRead, BinResult, BinWrite};

#[doc(inline)]
pub use raw::m64::{
    ControllerFlags, ControllerState, ExtendedData, ExtendedFlags, MovieStartType, RawMovie,
};

/// Extensions for reading binary data.
pub trait BinReadExt
where
    Self: Sized,
{
    /// Reads the binary data from a byte slice.
    fn from_bytes(bytes: &[u8]) -> BinResult<Self>;
    /// Reads the binary data from a file.
    fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self>;
}

/// Extensions for writing binary data.
pub trait BinWriteExt {
    /// Converts the instance to a byte vector.
    fn to_bytes(&self) -> BinResult<Vec<u8>>;
    /// Writes the instance to a binary file.
    fn to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()>;
}

macro_rules! impl_bin_read_ext {
    ($type:ty) => {
        impl BinReadExt for $type {
            fn from_bytes(bytes: &[u8]) -> BinResult<Self> {
                let mut cursor = Cursor::new(bytes);
                Self::read_le(&mut cursor)
            }

            fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
                let mut file = File::open(path)?;
                Self::read_le(&mut file)
            }
        }
    };
}

macro_rules! impl_bin_write_ext {
    ($type:ty) => {
        impl BinWriteExt for $type {
            fn to_bytes(&self) -> BinResult<Vec<u8>> {
                let mut cursor = Cursor::new(Vec::new());
                self.write(&mut cursor)?;
                Ok(cursor.into_inner())
            }

            fn to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
                let mut file = File::create(path)?;
                self.write(&mut file)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_try_from {
    ($type:ty) => {
        impl TryFrom<&[u8]> for $type {
            type Error = binrw::Error;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                Self::from_bytes(bytes)
            }
        }
    };
}

impl_try_from!(RawMovie);
impl_bin_read_ext!(RawMovie);
impl_bin_write_ext!(RawMovie);

impl_try_from!(ExtendedFlags);
impl_bin_read_ext!(ExtendedFlags);
impl_bin_write_ext!(ExtendedFlags);

impl_bin_read_ext!(MovieStartType);
impl_bin_write_ext!(MovieStartType);

impl_try_from!(ControllerFlags);
impl_bin_read_ext!(ControllerFlags);
impl_bin_write_ext!(ControllerFlags);

impl_bin_read_ext!(ExtendedData);
impl_bin_write_ext!(ExtendedData);

impl_try_from!(ControllerState);
impl_bin_read_ext!(ControllerState);
impl_bin_write_ext!(ControllerState);
