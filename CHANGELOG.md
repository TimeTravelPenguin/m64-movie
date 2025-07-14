# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
