# m64-movie

[![crates](https://img.shields.io/crates/v/m64-movie.svg)](https://crates.io/crates/m64-movie)
[![build](https://github.com/timetravelpenguin/m64-movie/actions/workflows/ci.yml/badge.svg)](https://github.com/timetravelpenguin/m64-movie/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/m64-movie/badge.svg)](https://docs.rs/m64-movie)

A Rust library for reading and writing [Mupen64](https://github.com/mupen64/mupen64-rr-lua) movie files.
Only version 3 .m64 files are supported.

If you need more information regarding the semantics of the file type, please
refer to the [movie file documentation](https://tasvideos.org/EmulatorResources/Mupen/M64).

## Important Note

As Mupen64 continues to develop, additional changes may be made to the movie
file definition. Feel free to open an issue or make a pull request if ever such
change occurs.

Additionally, feel free to request changes, such as for a more convenient API.

## General Information

This crate provides two interfaces for working with .m64 files.

At a higher-level, [`Movie`](https://docs.rs/m64-movie/latest/m64_movie/struct.Movie.html)
exposes movie data through types and enum variants. This struct exposes only
what the movie file specifies. If the movie file provides a certain
extended-flag, then it will be accessible through an enum. However, missing flags that
are valid for movies containing different extended versions will not be
accessible here. Additionally, there is no access the reserved regions within
the movie file.

[`RawMovie`](https://docs.rs/m64-movie/latest/m64_movie/struct.RawMovie.html) is
instead a struct with access to all regions data, including reserved values. It
makes no attempt to limit what is and isn't accessible, providing complete
freedom for working with m64 files. Only when reading or writing a movie is any
form of validation performed. Note that this struct may change if future
additions to the movie format occur.

## Usage

```rs
use m64_movie::{BinReadExt, Movie, RawMovie};

const MOVIE_BYTES: &[u8] = include_bytes!("path/to/my_movie.m64");

let movie = Movie::from_bytes(MOVIE_BYTES).expect("Failed to parse movie bytes");
let raw_movie = RawMovie::from_bytes(MOVIE_BYTES).expect("Failed to parse movie");

assert_eq!(
    movie.recording_info.author_name.to_string(),
    raw_movie.author_name.to_string()
);
```
