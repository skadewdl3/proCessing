extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    println!("{}", crate_dir);

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("../test-c/bindings.h");
}