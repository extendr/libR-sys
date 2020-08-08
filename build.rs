extern crate bindgen;

use std::env;
use std::path::PathBuf;

struct InstallationPaths {
    config: Option<String>,
    library: Option<String>
}

fn paths() -> InstallationPaths {
    if let Ok(r_home) = env::var("R_HOME") {
        InstallationPaths {
            config: Some(format!("{}/../pkgconfig", r_home)),
            library: Some(format!("{}/lib", r_home))
        }
    } else {
        InstallationPaths {
            config: None,
            library: None
        }
    }
}

fn main() {
    let details = paths();

    if let Some(v) = details.config {
        env::set_var("PKG_CONFIG_PATH", v);
        env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    }

    if let Some(v) = details.library {
        env::set_var("LD_LIBRARY_PATH", v);
    }

    let r_lib = pkg_config::probe_library("libR").unwrap();
    let r_home = pkg_config::get_variable("libR", "rhome").unwrap();
    println!("cargo:rustc-env=R_HOME={}", r_home);
    println!("cargo:r_home={}", r_home); // Becomes DEP_R_R_HOME for clients

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindgen_builder = bindgen::Builder::default()
        // These constants from libm break bindgen.
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // Point to the correct headers
    let bindgen_builder = r_lib.include_paths
        .iter()
        .fold(bindgen_builder, |bb, path| bb.clang_arg(format!("-I{}", path.to_str().unwrap())));

    // Finish the builder and generate the bindings.
    let bindings = bindgen_builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
