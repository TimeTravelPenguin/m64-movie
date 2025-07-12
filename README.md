# m64-movie

A Rust library for reading and writing [Mupen64](https://github.com/mupen64/mupen64-rr-lua) movie files.
Only version 3 .m64 files are supported.

If you need more information regarding the semantics of the file type, please
refer to the [movie file documentation](https://tasvideos.org/EmulatorResources/Mupen/M64).

## Important Note

As Mupen64 continues to develop, additional changes may be made to the movie
file definition. Feel free to open an issue or make a pull request if ever such
change occurs.

Additionally, feel free to request changes, such as for a more convenient API.

## Usage

```rs
use m64_movie::Movie;

const MOVIE_BYTES: &[u8] = include_bytes!("path/to/my_movie.m64");
let movie = Movie::from_bytes(&bytes).expect("Failed to parse movie");

println!("Author: {:?}", movie.author_name);
```

Refer to [`Movie`](https://docs.rs/m64-movie/latest/m64_movie/struct.Movie.html)
for additional information.
