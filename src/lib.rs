//! A low-level libR binding library which is kept deliberately
//! minimal.
//!
//! In particular, it has no external dependencies other that libR
//! installed on the target.
//!
//! ## Synopsis
//!
//! The `libR-sys` crate is a low level bindgen wrapper for the R
//! programming language. The intention is to allow one or more extension
//! mechanisms to be implemented for rust.
//!
//! Effort to make the extension libraries platform-independent can be
//! concentrated here.
//!
//! # Examples
//!
//! ```no_run
//! use libR_sys::{Rf_initialize_R, R_CStackLimit, setup_Rmainloop};
//! use std::os::raw;
//!
//! unsafe {
//!   std::env::set_var("R_HOME", "/usr/lib/R");
//!   let arg0 = "R\0".as_ptr() as *mut raw::c_char;
//!   Rf_initialize_R(1, [arg0].as_mut_ptr());
//!   R_CStackLimit = usize::max_value();
//!   setup_Rmainloop();
//! }
//! ```
//!
//! # Conditional compilation depending on R installation
//!
//! The libR-sys crate provides these environmental variables that you can use in `build.rs`:
//!
//! - `DEP_R_R_VERSION_MAJOR`: The major part of the R version (e.g. `4` in version `4.1.0`)
//! - `DEP_R_R_VERSION_MINOR`: The minor part of the R version (e.g. `1` in version `4.1.0`)
//! - `DEP_R_R_VERSION_PATCH`: The patch part of the R version (e.g. `0` in version `4.1.0`)
//! - `DEP_R_R_VERSION_DEVEL`: `true` if the R is a development version, otherwise `false`
//! - `DEP_R_R_VERSION_STRING`: The full version string (e.g. `R version 4.1.0 (2021-05-18)`)
//! - `DEP_R_R_HOME`: The R home directory
//!
//! ## Example `build.rs`
//!
//! ```ignore
//! use std::env;
//!
//! fn main() {
//!     // Set R_HOME envvar, and refer to it on compile time by env!("R_HOME")
//!     let r_home = env::var("DEP_R_R_HOME").unwrap();
//!     println!("cargo:rustc-env=R_HOME={}", r_home);
//!
//!     // Enable cfg setting to conditionally compile a code using a feature
//!     // available only on newer versions of R
//!     let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
//!     let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();
//!     if &*major >= "4" && &*minor >= "1" {
//!         println!("cargo:rustc-cfg=use_a_feature");
//!     }
//! }
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw;

    // Generate constant static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *const raw::c_char
        };
    }

    // Generate mutable static strings.
    // Much more efficient than CString.
    // Generates asciiz.
    macro_rules! cstr_mut {
        ($s: expr) => {
            concat!($s, "\0").as_ptr() as *mut raw::c_char
        };
    }

    // Thanks to @qinwf and @scottmmjackson for showing the way here.
    fn start_R() {
        unsafe {
            if std::env::var("R_HOME").is_err() {
                // env! gets the build-time R_HOME made in build.rs
                std::env::set_var("R_HOME", env!("R_HOME"));
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            if cfg!(target_os = "windows") && cfg!(target_arch = "x86") {
                Rf_initialize_R(
                    4,
                    [
                        cstr_mut!("R"),
                        cstr_mut!("--arch=i386"),
                        cstr_mut!("--slave"),
                        cstr_mut!("--no-save"),
                    ]
                    .as_mut_ptr(),
                );
            } else {
                Rf_initialize_R(
                    3,
                    [cstr_mut!("R"), cstr_mut!("--slave"), cstr_mut!("--no-save")].as_mut_ptr(),
                );
            }

            // In case you are curious.
            // Maybe 8MB is a bit small.
            // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);

            if cfg!(not(target_os = "windows")) {
                R_CStackLimit = usize::max_value();
            }

            setup_Rmainloop();
        }
    }

    // Run some R code. Check the result.
    #[test]
    fn test_eval() {
        start_R();
        unsafe {
            let val = Rf_protect(R_ParseEvalString(cstr!("1"), R_NilValue));
            Rf_PrintValue(val);
            assert_eq!(TYPEOF(val) as u32, REALSXP);
            assert_eq!(*REAL(val), 1.);
            Rf_unprotect(1);
        }
    }
}
