//! This module contains the raw data structures for the M64 file format.

use std::{fs::File, io::Cursor, path::Path};

use binrw::{BinRead, BinWrite};

use crate::{BinReadExt, BinWriteExt, MovieError};

#[doc(hidden)]
pub mod m64;

#[doc(inline)]
pub use m64::*;

macro_rules! impl_bin_read_ext {
    ($type:ty) => {
        impl BinReadExt for $type {
            type Error = MovieError;

            fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(bytes);
                Ok(Self::read_le(&mut cursor)?)
            }

            fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
                let mut file = File::open(path)?;
                Ok(Self::read_le(&mut file)?)
            }
        }
    };
}

macro_rules! impl_bin_write_ext {
    ($type:ty) => {
        impl BinWriteExt for $type {
            type Error = MovieError;

            fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
                let mut cursor = Cursor::new(Vec::new());
                self.write(&mut cursor)?;
                Ok(cursor.into_inner())
            }

            fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error> {
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
            type Error = MovieError;

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
