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

#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

extern "C" {
    // Return type should match `SEXPTYPE`
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
}

#[allow(non_camel_case_types)]
pub type R_altrep_Coerce_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>;

pub unsafe fn Rf_isS4(arg1: SEXP) -> Rboolean {
    unsafe {
        if secret::Rf_isS4_original(arg1) == 0 {
            Rboolean::FALSE
        } else {
            Rboolean::TRUE
        }
    }
}

mod secret {
    use super::*;
    extern "C" {
        #[link_name = "Rf_isS4"]
        pub fn Rf_isS4_original(arg1: SEXP) -> u32;
    }
}

impl From<Rboolean> for bool {
    fn from(value: Rboolean) -> Self {
        match value {
            Rboolean::FALSE => false,
            Rboolean::TRUE => true,
        }
    }
}

impl From<bool> for Rboolean {
    fn from(value: bool) -> Self {
        match value {
            true => Rboolean::TRUE,
            false => Rboolean::FALSE,
        }
    }
}

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
            assert_eq!(TYPEOF(val), SEXPTYPE::REALSXP);
            assert_eq!(*REAL(val), 1.);
            Rf_unprotect(1);
        }
        // There is one pathological example of `Rf_is*` where `TRUE` is not 1,
        // but 16. We show here that the casting is done as intended
        unsafe {
            let sexp = R_ParseEvalString(cstr!(r#"new("factor")"#), R_GlobalEnv);
            Rf_protect(sexp);
            Rf_PrintValue(sexp);

            assert_eq!(
                std::mem::discriminant(&Rf_isS4(sexp)),
                std::mem::discriminant(&Rboolean::TRUE),
            );
            assert!(<Rboolean as Into<bool>>::into(Rf_isS4(sexp)));
            assert!(
                (Rboolean::FALSE == Rf_isS4(sexp)) || (Rboolean::TRUE == Rf_isS4(sexp)),
                "PartialEq implementation is broken"
            );
            assert!(Rboolean::TRUE == Rf_isS4(sexp));
            assert_eq!(Rf_isS4(sexp), Rboolean::TRUE);
            Rf_unprotect(1);
        }
    }
}
