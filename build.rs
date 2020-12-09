extern crate bindgen;

use regex::Regex;
use std::{env, io, io::Error, io::ErrorKind, path::Path, path::PathBuf, process::exit, process::Command};

struct InstallationPaths {
    r_home: String,
    include: String,
    library: String,
}

fn probe_r_paths() -> io::Result<InstallationPaths> {
    // First we locate the R home
    let r_home = match env::var("R_HOME") {
        // If the environment variable R_HOME is set we use it
        Ok(s) => s,

        // Otherwise, we try to execute `R` to find `R_HOME`. Note that this is
        // discouraged, see Section 1.6 of "Writing R Extensions"
        // https://cran.r-project.org/doc/manuals/r-release/R-exts.html#Writing-portable-packages
        _ => {
            let rout = Command::new("R")
                .args(&[
                    "-s",
                    "-e",
                    r#"cat(normalizePath(R.home()), sep = '\n')"#
                ])
                .output()?;

            let rout = String::from_utf8_lossy(&rout.stdout);
            let mut lines = rout.lines();

            match lines.next() {
                Some(line) => line.to_string(),
                _ => return Err(Error::new(ErrorKind::Other, "Cannot find R home.")),
            }
        }
    };

    // Now the library location. On Windows, it depends on the target architecture
    let pkg_target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let library = if cfg!(target_os = "windows") {
        if pkg_target_arch == "x86_64" {
            Path::new(&r_home)
                .join("bin")
                .join("x64")
                .to_str()
                .unwrap()
                .to_string()
        } else if pkg_target_arch == "x86" {
            Path::new(&r_home)
                .join("bin")
                .join("i386")
                .to_str()
                .unwrap()
                .to_string()
        } else {
            panic!("Unknown architecture")
        }
    } else {
        Path::new(&r_home).join("lib").to_str().unwrap().to_string()
    };

    // Finally the include location. It may or may not be located under R home
    let include = match env::var("R_INCLUDE_DIR") {
        // If the environment variable R_INCLUDE_DIR is set we use it
        Ok(s) => s,

        // Otherwise, we try to execute `R` to find the include dir. Here,
        // we're using the R home we found earlier, to make sure we're consistent.
        _ => {
            let r_binary = if cfg!(target_os = "windows") {
                Path::new(&library)
                    .join("R.exe")
                    .to_str()
                    .unwrap()
                    .to_string()
            } else {
                Path::new(&r_home)
                    .join("bin")
                    .join("R")
                    .to_str()
                    .unwrap()
                    .to_string()
            };

            println!("R home: {}", r_home);
            println!("R library: {}", library);
            println!("R binary: {}", r_binary);

            let rout = Command::new(&r_binary)
                .args(&[
                    "-s",
                    "-e",
                    r#"cat(normalizePath(R.home("include")), sep = '\n')"#
                ])
                .output()?;

            let rout = String::from_utf8_lossy(&rout.stdout);
            let mut lines = rout.lines();

            match lines.next() {
                Some(line) => line.to_string(),
                _ => return Err(Error::new(ErrorKind::Other, "Cannot find R include.")),
            }
        }
    };

    Ok(InstallationPaths {
        r_home,
        include,
        library,
    })
}

fn main() {
    let details = probe_r_paths();

    let details = match details {
        Ok(result) => result,
        Err(error) => {
            println!("Problem locating local R install: {:?}", error);
            exit(1);
        }
    };

    println!("cargo:rustc-env=R_HOME={}", &details.r_home);
    println!("cargo:r_home={}", &details.r_home); // Becomes DEP_R_R_HOME for clients
    // make sure cargo links properly against library
    println!("cargo:rustc-link-search={}", &details.library);
    println!("cargo:rustc-link-lib=dylib=R");

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

        // println!("TARGET: {}",cargo_env("TARGET"));
    // Point to the correct headers
    let bindgen_builder = bindgen_builder.clang_args(&[
        format!("-I{}", &details.include),
        format!("--target={}", std::env::var("TARGET").expect("Could not get the target triple"))
    ]);

    // Finish the builder and generate the bindings.
    let bindings = bindgen_builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Extract the version number from the R headers.
    let version_matcher = Regex::new(r"pub const R_VERSION ?: ?u32 = (\d+)").unwrap();
    if let Some(version) = version_matcher.captures(bindings.to_string().as_str()) {
        let version = version.get(1).unwrap().as_str().parse::<u32>().unwrap();
        println!("cargo:r_version={}", version);
    } else {
        panic!("failed to find R_VERSION");
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings to default output path!");

    // Also write the bindings to a folder specified by $LIBRSYS_BINDINGS_DIR, if it exists

    if let Ok(alt_target) = env::var("LIBRSYS_BINDINGS_DIR") {
        let out_path = PathBuf::from(alt_target);

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings to output path specified by $LIBRSYS_BINDINGS_DIR!");

    }
}
