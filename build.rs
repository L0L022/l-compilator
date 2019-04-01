extern crate bindgen;
extern crate cc;
extern crate lalrpop;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
    cc::Build::new()
        .file("src/c_code/c3a2nasm.c")
        .file("src/c_code/code3a.c")
        .file("src/c_code/tabsymboles.c")
        .file("src/c_code/util.c")
        .compile("c_code");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/c_code/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .process()
        .unwrap();
}
