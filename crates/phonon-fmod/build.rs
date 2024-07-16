extern crate bindgen;

use bindgen::Builder;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let builder = Builder::default()
        .header("headers/fmod_common.h")
        .header("headers/fmod_dsp.h");

    let out_dir = env::var("OUT_DIR").unwrap();

    // todo give it a better name
    let dest_path = Path::new(&out_dir).join("bindgen.rs");

    let bindings = builder.generate().unwrap();
    bindings.write_to_file(&dest_path).unwrap();
}
