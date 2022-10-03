use clap::value_parser;
use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Oliver Hamlet")
        .arg(
            Arg::new("svg_path")
                .short('i')
                .long("input")
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .help("Path to the SVG file to convert")
                .required(true),
        )
        .arg(
            Arg::new("svg_dpi")
                .short('d')
                .long("dpi")
                .value_name("DPI")
                .value_parser(value_parser!(f32))
                .help("DPI to use when interpreting the SVG file")
                .default_value("96.0"),
        )
        .arg(
            Arg::new("ico_path")
                .short('o')
                .long("output")
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .help("Output path for the ICO file")
                .required(true),
        )
        .arg(
            Arg::new("ico_sizes")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .value_parser(value_parser!(u16))
                .action(ArgAction::Append)
                .num_args(1..)
                .default_values(&[
                    "16", "20", "24", "30", "32", "36", "40", "48", "60", "64", "72", "80", "96",
                    "128", "256",
                ])
                .long_help("An image size (height in pixels) to include within the ICO file."),
        )
        .get_matches();

    let svg_path = matches
        .get_one::<PathBuf>("svg_path")
        .expect("svg_path is required");
    let svg_dpi = matches
        .get_one::<f32>("svg_dpi")
        .copied()
        .expect("svg_dpi is has a default value");
    let ico_path = matches
        .get_one::<PathBuf>("ico_path")
        .expect("ico_path is required");
    let ico_sizes: Vec<u16> = matches
        .get_many("ico_sizes")
        .expect("ico_sizes has a default value")
        .copied()
        .collect();

    svg_to_ico::svg_to_ico(svg_path, svg_dpi, ico_path, &ico_sizes).unwrap();
}
