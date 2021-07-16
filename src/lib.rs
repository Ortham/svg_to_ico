//! # svg_to_ico
//!
//! Simple SVG to ICO conversion.
//!
//! SVG images are parsed and rasterised using [resvg](https://github.com/RazrFalcon/resvg)
//! with its [raqote](https://github.com/jrmuizel/raqote) backend.
//!
//! This crate provides a single function to create an ICO file from an SVG file.
use std::fs::{create_dir_all, read, File};
use std::io;
use std::path::Path;

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
    let opt = usvg::Options {
        dpi: svg_dpi.into(),
        ..Default::default()
    };

    let file_content = read(svg_path)?;
    let svg = usvg::Tree::from_data(&file_content, &opt).map_err(|_| Error::ParseError)?;

    let images = ico_entry_sizes
        .iter()
        .map(|size| rasterize(&svg, *size))
        .collect::<Result<Vec<_>, Error>>()?;

    create_ico(ico_path, images).map_err(Error::from)
}

fn rasterize(svg: &usvg::Tree, height_in_pixels: u16) -> Result<tiny_skia::Pixmap, Error> {
    let fit_to = usvg::FitTo::Height(height_in_pixels.into());
    let svg_size = svg.svg_node().size;
    let target_height = f64::from(height_in_pixels);
    let target_width = svg_size.width() * target_height / svg_size.height();

    usvg::Size::new(target_width, target_height)
        .map(|size| size.to_screen_size())
        .and_then(|size| tiny_skia::Pixmap::new(size.width(), size.height()))
        .map(|mut pixmap| {
            resvg::render(svg, fit_to, pixmap.as_mut());
            pixmap
        })
        .ok_or(Error::RasterizeError)
}

fn create_ico(ico_path: &Path, pngs: Vec<tiny_skia::Pixmap>) -> io::Result<()> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for png in pngs {
        let image = ico::IconImage::from_rgba_data(png.width(), png.height(), png.take());
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
        opt.dpi = svg_dpi.into();

        let file_content = read(path).unwrap();
        usvg::Tree::from_data(&file_content, &opt).unwrap()
    }

    #[test]
    fn rasterize_should_scale_svg_to_given_height() {
        let svg_path = Path::new("examples/example.svg");
        let svg = load_svg(svg_path);

        assert_eq!(24.0, svg.svg_node().size.height());
        assert_eq!(24.0, svg.svg_node().size.width());

        let image = rasterize(&svg, 400).unwrap();
        assert_eq!(400, image.height());
        assert_eq!(400, image.width());
    }

    #[test]
    fn rasterize_should_set_pixel_colour_correctly() {
        let svg_path = Path::new("examples/example.svg");
        let svg = load_svg(svg_path);

        let image = rasterize(&svg, 24).unwrap();
        let pixel_index = 24 * 6 + 12;
        let pixel = &image.take()[pixel_index * 4..(pixel_index + 1) * 4];

        assert_eq!(&[50, 100, 150, 255], pixel);
    }
}
