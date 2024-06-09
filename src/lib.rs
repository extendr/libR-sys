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
    #[cfg(feature = "Boolean")] // FIXME: maybe remove?
    #[cfg(feature = "Rinternals")]
    // Return type should match `SEXPTYPE`
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
}

#[cfg(feaature = "Altrep")]
#[allow(non_camel_case_types)]
pub type R_altrep_Coerce_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>;

    
#[cfg(feature = "Boolean")]
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
        #[cfg(feature = "Rinternals")]
        #[link_name = "Rf_isS4"]
        pub fn Rf_isS4_original(arg1: SEXP) -> u32;
    }
}


#[cfg(feature = "Boolean")]
impl From<Rboolean> for bool {
    fn from(value: Rboolean) -> Self {
        match value {
            Rboolean::FALSE => false,
            Rboolean::TRUE => true,
        }
    }
}

#[cfg(feature = "Boolean")]
impl From<bool> for Rboolean {
    fn from(value: bool) -> Self {
        match value {
            true => Rboolean::TRUE,
            false => Rboolean::FALSE,
        }
    }
}

#[cfg(test)]
mod tests;
