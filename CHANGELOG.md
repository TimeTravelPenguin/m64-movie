# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/TimeTravelPenguin/m64-movie/compare/v0.3.1...v0.4.0) - 2025-07-17

### Added

- [**breaking**] replaced `ExtendedFlags` and `ExtendedData` with enum variant
- [**breaking**] refactored `EncodedFixedStr<const N: usize>`
- added `Movie::controller_inputs_stream()`
- added `Movie` for a more idiomatic Rust-style use
- added `from_utf8_str` and `from_ascii_str`
- implemented `Default` for `Reserved`
- added `shared::FixedStr` and `shared::EncodedFixedStr`

### Other

- tidied up exports
- updated README
- updated docstrings
- updated docstrings
- corrected links
- [**breaking**] improved public lib api
- reordered code
- [**breaking**] moved `impl`s into respective modules
- [**breaking**] moved `m64::ControllerButton` to `movie::ControllerButton`
- [**breaking**] moved `m64::Movie` to `raw::m64:RawMovie`

## [0.3.1](https://github.com/TimeTravelPenguin/m64-movie/compare/v0.3.0...v0.3.1) - 2025-07-14

### Fixed

- *(docs)* corrected incorrect docs

## [0.3.0](https://github.com/TimeTravelPenguin/m64-movie/compare/v0.2.1...v0.3.0) - 2025-07-14

### Other

- [**breaking**] implemented `BinReadExt`, `BinWriteExt`, and `TryFrom<&[u8]>`
- [**breaking**] moved existing types into modules `m64` and `shared`

## [0.2.1](https://github.com/TimeTravelPenguin/m64-movie/compare/v0.2.0...v0.2.1) - 2025-07-12

### Added

- added `Movie::to_file` and `Movie::from_file`
- implemented TryFrom<&[u8]> for Movie

### Other

- cleaned up imports
