extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=R");

    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // These constants from libm break bindgen.
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")

        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // TODO: Change this depending on version
        .clang_arg("-I/usr/share/R/include")

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
