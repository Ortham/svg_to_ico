# Changelog

## [1.2.0] - 2022-09-14

### Fixed

- Panic when working with an image that is wider than it is tall.

### Changed

- Updated `resvg` to 0.23 and switched to the `usvg` backend.
- Updated `clap` to 3.2.21.
- Updated patch versions of various indirect dependencies.

### Removed

- Deprecated `Error::description()` implementation.

## [1.1.0] - 2019-08-18

### Changed

- Replaced `nsvg` dependency with `resvg` v0.8, using the raqote backend, as it
  gives more accurate rasterisation results.

## [1.0.0] - 2018-04-11

### Changed

- Fixed docs link in `Cargo.toml`.
- Fleshed out README and library docs.

## [0.1.0] - 2018-04-08

Initial release.
