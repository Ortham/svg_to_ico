extern crate svg_to_ico;
extern crate tempfile;

use std::path::Path;

fn main() {
    let tempdir = tempfile::tempdir().expect("failed to create temp dir");

    let input = Path::new("examples/example.svg");
    let output = tempdir.path().join("icon.ico");

    svg_to_ico::svg_to_ico(input, 96.0, &output, &[32, 64]).expect("failed to convert svg to ico");

    assert!(output.exists());
}
