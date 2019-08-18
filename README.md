svg_to_ico
==========

[![Crate](https://img.shields.io/crates/v/svg_to_ico.svg)](https://crates.io/crates/svg_to_ico)
[![docs](https://docs.rs/svg_to_ico/badge.svg)](https://docs.rs/crate/svg_to_ico)
[![Travis Build Status](https://www.travis-ci.org/Ortham/svg_to_ico.svg?branch=master)](https://www.travis-ci.org/Ortham/svg_to_ico)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/qgfqudc6qyu1liby/branch/master?svg=true)](https://ci.appveyor.com/project/Ortham/svg-to-ico/branch/master)
[![dependency status](https://deps.rs/repo/github/Ortham/svg_to_ico/status.svg)](https://deps.rs/repo/github/Ortham/svg_to_ico)

This is a small cross-platform CLI utility to convert SVG icons into Windows ICO
files. SVG images are parsed and rasterised using [resvg](https://github.com/RazrFalcon/resvg)
with its [raqote](https://github.com/jrmuizel/raqote) backend.

## Download

Precompiled binaries are [available](https://github.com/Ortham/svg_to_ico/releases/latest) for Windows and Linux. You can also `cargo install svg_to_ico` to build and install it from source.

## Build

To build svg_to_ico from a source archive/repository, install [Rust](https://www.rust-lang.org) then run

```
cargo build --release
```

from the archive/repository root to create a release executable at `target/release/svg_to_ico` (`svg_to_ico.exe` on Windows).

## Usage

### CLI

See the output of `./svg_to_ico -h` for a description of the CLI parameters. You can specify the
input SVG path, output ICO path, the DPI to interpret the SVG with, and the image sizes that should
be included in the ICO.

Example:

```
./svg_to_ico -i icon.svg -o icon.ico
```

### Library

You can also use svg_to_ico as a Rust library, just add it to your `Cargo.toml`:

```
[dependencies]
svg_to_ico = "0.1"
```

then use it as shown in the [example](examples/library.rs).
