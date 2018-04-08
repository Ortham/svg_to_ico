extern crate ico;
extern crate nsvg;
#[cfg(test)]
extern crate tempfile;

use std::fs::{create_dir_all, File};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NulError(std::ffi::NulError),
    ParseError,
    RasterizeError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<nsvg::Error> for Error {
    fn from(error: nsvg::Error) -> Self {
        match error {
            nsvg::Error::IoError(e) => Error::IoError(e),
            nsvg::Error::NulError(e) => Error::NulError(e),
            nsvg::Error::ParseError => Error::ParseError,
            nsvg::Error::MallocError | nsvg::Error::RasterizeError => Error::RasterizeError,
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
            Error::NulError(ref e) => e.description(),
            Error::ParseError => "An unknown SVG parsing error",
            Error::RasterizeError => "Failed to rasterize SVG",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::NulError(ref e) => e.fmt(f),
            Error::ParseError => write!(f, "An unknown SVG parsing error"),
            Error::RasterizeError => write!(f, "Failed to rasterize SVG"),
        }
    }
}

struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

pub fn svg_to_ico(
    svg_path: &Path,
    svg_dpi: f32,
    ico_path: &Path,
    ico_entry_sizes: &[u16],
) -> Result<(), Error> {
    let svg = nsvg::parse_file(svg_path, nsvg::Units::Pixel, svg_dpi)?;

    let images: Vec<Image> = ico_entry_sizes
        .iter()
        .map(|size| rasterize(&svg, *size))
        .collect::<Result<Vec<Image>, nsvg::Error>>()?;

    create_ico(ico_path, images).map_err(Error::from)
}

fn rasterize(svg: &nsvg::SvgImage, height_in_pixels: u16) -> Result<Image, nsvg::Error> {
    let scale = height_in_pixels as f32 / svg.height();

    svg.rasterize(scale).map(|img| Image {
        width: img.width(),
        height: img.height(),
        data: img.into_raw(),
    })
}

fn create_ico(ico_path: &Path, pngs: Vec<Image>) -> io::Result<()> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for png in pngs {
        let image = ico::IconImage::from_rgba_data(png.width, png.height, png.data);
        icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
    }

    if let Some(p) = ico_path.parent() {
        create_dir_all(p)?;
    }

    let file = File::create(ico_path)?;
    icon_dir.write(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasterize_should_scale_svg_to_given_height() {
        let svg =
            nsvg::parse_file(Path::new("examples/example.svg"), nsvg::Units::Pixel, 96.0).unwrap();
        assert_eq!(24.0, svg.height());
        assert_eq!(24.0, svg.width());

        let image = rasterize(&svg, 400).unwrap();
        assert_eq!(400, image.height);
        assert_eq!(400, image.width);
    }
}
