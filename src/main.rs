extern crate clap;
extern crate svg_to_ico;

use clap::{App, Arg};
use std::path::Path;

fn main() {
    let default_ico_sizes = vec![16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 256];

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Oliver Hamlet")
        .arg(
            Arg::with_name("svg_path")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Path to the SVG file to convert")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("svg_dpi")
                .short("d")
                .long("dpi")
                .value_name("DPI")
                .help("DPI to use when interpreting the SVG file")
                .takes_value(true)
                .default_value("96.0"),
        )
        .arg(
            Arg::with_name("ico_path")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output path for the ICO file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("ico_sizes")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .multiple(true)
                .long_help(&format!("An image size (height in pixels) to include within the ICO file. If no sizes are specified, the following are used: {:?}.", default_ico_sizes))
                .takes_value(true),
        )
        .get_matches();

    let svg_path = matches.value_of("svg_path").map(Path::new).unwrap();
    let svg_dpi = matches.value_of("svg_dpi").unwrap().parse::<f32>().unwrap();
    let ico_path = matches.value_of("ico_path").map(Path::new).unwrap();
    let ico_sizes: Vec<u16> = matches
        .values_of("ico_sizes")
        .map(|i| i.map(|v| v.parse::<u16>().unwrap()).collect())
        .unwrap_or(default_ico_sizes);

    svg_to_ico::svg_to_ico(svg_path, svg_dpi, ico_path, &ico_sizes).unwrap();
}
