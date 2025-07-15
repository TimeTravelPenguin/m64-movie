pub mod m64;

use crate::{BinReadExt, BinWriteExt, Movie, MovieError, Path, RawMovie};

impl BinReadExt for Movie {
    type Error = MovieError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let raw_movie = RawMovie::from_bytes(bytes)?;
        Ok(Movie::try_from(raw_movie)?)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let raw_movie = RawMovie::from_file(path)?;
        Ok(Movie::try_from(raw_movie)?)
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
