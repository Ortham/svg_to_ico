//! # svg_to_ico
//!
//! Simple SVG to ICO conversion.
//!
//! SVG images are parsed and rasterised using [resvg](https://github.com/RazrFalcon/resvg)
//! with its [raqote](https://github.com/jrmuizel/raqote) backend.
//!
//! This crate provides a single function to create an ICO file from an SVG file.
use std::convert::TryFrom;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::Path;

use resvg::usvg;

/// Error returned when creating an ICO file from an SVG file fails.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred, e.g. the input file doesn't exist.
    IoError(std::io::Error),
    /// No longer used.
    NulError(std::ffi::NulError),
    /// Something went wrong when parsing the SVG file.
    ParseError,
    /// Something went wrong when rasterizing the SVG file.
    RasterizeError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
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

/// Create a new ICO file from given SVG file.
///
/// SVG dimensions are interpreted as pixels and the image rasterized using the given DPI. The ICO
/// entry sizes are the heights in pixels of the images to store inside the ICO file: the SVG image
/// will be scaled to produce images of the specified sizes. If the ICO
/// file's parent directory does not exist, it will be created.
///
/// ## Examples
///
/// Interpret an SVG file as having a DPI of 96 and create an ICO file containing images with
/// heights of 32 px and 64 px:
///
/// ```
/// # extern crate svg_to_ico;
/// use std::path::Path;
/// use svg_to_ico::svg_to_ico;
///
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<(), svg_to_ico::Error> {
/// let input = Path::new("examples/example.svg");
/// let output = Path::new("examples/example.ico");
///
/// svg_to_ico(input, 96.0, output, &[32, 64])?;
/// #     Ok(())
/// # }
/// ```
pub fn svg_to_ico(
    svg_path: &Path,
    svg_dpi: f32,
    ico_path: &Path,
    ico_entry_sizes: &[u16],
) -> Result<(), Error> {
    let mut opt = usvg::Options::default();
    opt.path = Some(svg_path.into());
    opt.dpi = svg_dpi.into();
    let svg = usvg::Tree::from_file(svg_path, &opt).map_err(|_| Error::ParseError)?;

    let images: Vec<Image> = ico_entry_sizes
        .iter()
        .map(|size| rasterize(&svg, *size))
        .collect::<Result<Vec<Image>, Error>>()?;

    create_ico(ico_path, images).map_err(Error::from)
}

fn rasterize(svg: &usvg::Tree, height_in_pixels: u16) -> Result<Image, Error> {
    let mut opt = resvg::Options::default();
    opt.fit_to = resvg::FitTo::Height(height_in_pixels.into());
    let image = match resvg::backend_raqote::render_to_image(svg, &opt) {
        Some(i) => i,
        None => return Err(Error::RasterizeError),
    };

    let width = u32::try_from(image.width()).map_err(|_| Error::RasterizeError)?;
    let height = u32::try_from(image.height()).map_err(|_| Error::RasterizeError)?;

    Ok(Image {
        width,
        height,
        data: argb_to_rgba(image.into_vec()),
    })
}

fn argb_to_rgba(argb: Vec<u32>) -> Vec<u8> {
    argb.into_iter()
        .flat_map(|pixel| {
            let [a, r, g, b] = pixel.to_be_bytes();
            vec![r, g, b, a].into_iter()
        })
        .collect()
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

    fn load_svg(path: &Path) -> usvg::Tree {
        let svg_dpi = 96.0;

        let mut opt = usvg::Options::default();
        opt.path = Some(path.into());
        opt.dpi = svg_dpi.into();
        usvg::Tree::from_file(path, &opt).unwrap()
    }

    #[test]
    fn rasterize_should_scale_svg_to_given_height() {
        let svg_path = Path::new("examples/example.svg");
        let svg = load_svg(svg_path);

        assert_eq!(24.0, svg.svg_node().size.height());
        assert_eq!(24.0, svg.svg_node().size.width());

        let image = rasterize(&svg, 400).unwrap();
        assert_eq!(400, image.height);
        assert_eq!(400, image.width);
    }

    #[test]
    fn rasterize_should_set_pixel_colour_correctly() {
        let svg_path = Path::new("examples/example.svg");
        let svg = load_svg(svg_path);

        let image = rasterize(&svg, 24).unwrap();
        let pixel_index = 24 * 6 + 12;
        let pixel = &image.data[pixel_index * 4..(pixel_index + 1) * 4];

        assert_eq!(&[50, 100, 150, 255], pixel);
    }
}
