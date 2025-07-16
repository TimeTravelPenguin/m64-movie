//! Parsed movie data structures guaranteed to be valid Mupen64 movie files.

pub mod m64;

use std::path::Path;

#[doc(inline)]
pub use m64::{
    ExtendedData, ExtendedFlags, GameInfo, Movie, MovieDetails, MupenMetadata, PluginInfo,
    RecordingInfo,
};

use crate::{BinReadExt, BinWriteExt, MovieError, raw::m64::RawMovie};

impl BinReadExt for Movie {
    type Error = MovieError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let raw_movie = RawMovie::from_bytes(bytes)?;
        Movie::try_from(raw_movie)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let raw_movie = RawMovie::from_file(path)?;
        Movie::try_from(raw_movie)
    }
}

impl BinWriteExt for Movie {
    type Error = MovieError;

    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        let raw_movie: RawMovie = self.clone().into_raw();
        raw_movie.to_bytes()
    }

    fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error> {
        let raw_movie: RawMovie = self.clone().into_raw();
        raw_movie.to_file(path)
    }
}
