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


pub mod bindings {
    #[cfg(target_os = "macos")]
    #[path = "bindings-R-macos-aarch64.rs"]
    pub mod r;

    pub mod r_internals {
        use super::r_ext::boolean::Rboolean;
        use super::r_ext::complex::Rcomplex;
        use super::r_ext::r_dynload::DL_FUNC;

        #[non_exhaustive]
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct SEXPREC(std::ffi::c_void);

        extern "C" {
            // Return type should match `SEXPTYPE`
            pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
        }

        mod secret {
            use super::*;
            extern "C" {
                #[link_name = "Rf_isS4"]
                pub fn Rf_isS4_original(arg1: SEXP) -> u32;
            }
        }
        
        pub unsafe fn Rf_isS4(arg1: SEXP) -> Rboolean {
            unsafe {
                if secret::Rf_isS4_original(arg1) == 0 {
                    Rboolean::FALSE
                } else {
                    Rboolean::TRUE
                }
            }
        }

        #[cfg(target_os = "macos")]
        include!("bindings/bindings-Rinternals-macos-aarch64.rs");
    }

    #[cfg(target_os = "macos")]
    #[path = "bindings-Rmath-macos-aarch64.rs"]
    pub mod r_math;

    #[cfg(target_os = "macos")]
    #[path = "bindings-Rversion-macos-aarch64.rs"]
    pub mod r_version;

    #[cfg(target_os = "windows")]
    #[path = "bindings-Rversion-windows-x86_64.rs"]
    pub mod r_version;

    #[cfg(target_os = "macos")]
    #[path = "bindings-Rinterface-macos-aarch64.rs"]
    pub mod r_interface;

    pub mod r_embedded {
        use super::r_ext::boolean::Rboolean;

        #[cfg(target_os = "macos")]
        include!("bindings/bindings-Rembedded-macos-aarch64.rs");
    }

    #[path = ""]
    pub mod r_ext {
        pub mod applic {
            use super::boolean::Rboolean;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Applic-macos-aarch64.rs");
        }

        pub mod blas {
            use super::complex::Rcomplex;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-BLAS-macos-aarch64.rs");
        }

        pub mod callbacks {
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Callbacks-macos-aarch64.rs");
        }

        pub mod GetX11Image {
            use super::boolean::Rboolean;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-GetX11Image-macos-aarch64.rs");
        }

        pub mod lapack {
            use super::complex::Rcomplex;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Lapack-macos-aarch64.rs");
        }

        #[cfg(target_os = "macos")]
        #[path = "bindings-Linpack-macos-aarch64.rs"]
        pub mod linpack;

        pub mod parse {
            use super::super::r_internals::SEXP;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Parse-macos-aarch64.rs");
        }

        pub mod r_startup {
            use super::boolean::Rboolean;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-RStartup-macos-aarch64.rs");
        }

        pub mod r_dynload {
            use super::boolean::Rboolean;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Rdynload-macos-aarch64.rs");
        }

        #[path = "bindings-Riconv-macos-aarch64.rs"]
        pub mod r_iconv;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Visibility-macos-aarch64.rs"]
        pub mod visibility;

        #[path = "bindings-eventloop-macos-aarch64.rs"]
        pub mod event_loop;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Boolean-macos-aarch64.rs"]
        mod boolean_bindings;

        pub mod boolean {
            pub use super::boolean_bindings::*;

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
        }

        #[cfg(target_os = "macos")]
        #[path = "bindings-Complex-macos-aarch64.rs"]
        pub mod complex;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Arith-macos-aarch64.rs"]
        pub mod arith;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Constants-macos-aarch64.rs"]
        pub mod constants;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Error-macos-aarch64.rs"]
        pub mod error;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Memory-macos-aarch64.rs"]
        pub mod memory;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Print-macos-aarch64.rs"]
        pub mod print;

        #[cfg(target_os = "macos")]
        #[path = "bindings-RS-macos-aarch64.rs"]
        pub mod rs;

        #[cfg(target_os = "macos")]
        #[path = "bindings-Random-macos-aarch64.rs"]
        pub mod random;

        #[path = "Utils.rs"]
        pub mod utils {
            use super::boolean::Rboolean;
            use super::complex::Rcomplex;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Utils-macos-aarch64.rs");
        }

        #[cfg(target_os = "macos")]
        #[path = "bindings-Itermacros-macos-aarch64.rs"]
        pub mod itermacros;

        #[cfg(target_os = "macos")]
        #[path = "bindings-stats_stubs-macos-aarch64.rs"]
        pub mod stats_stubs;

        #[cfg(target_os = "macos")]
        #[path = "bindings-stats_package-macos-aarch64.rs"]
        pub mod stats_package;

        pub mod graphics_engine {
            use super::super::r_internals::cetype_t;
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;
            use super::graphics_device::pDevDesc;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-GraphicsEngine-macos-aarch64.rs");
        }

        pub mod graphics_device {
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;
            use super::graphics_engine::pGEcontext;
            #[cfg(target_os = "macos")]
            include!("bindings/bindings-GraphicsDevice-macos-aarch64.rs");
        }

        #[path = "bindings-QuartzDevice-macos-aarch64.rs"]
        pub mod quartz_device;

        #[path = "bindings-MathThreads-macos-aarch64.rs"]
        pub mod math_threads;

        pub mod connections {
            use super::boolean::Rboolean;

            include!("bindings/bindings-Connections-macos-aarch64.rs");
        }

        pub mod prt_util {
            use super::super::r_internals::SEXP;
            use super::complex::Rcomplex;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-PrtUtil-macos-aarch64.rs");
        }

        pub mod altrep {
            use super::super::r_internals::{Rbyte, SEXP, SEXPTYPE};
            use super::boolean::Rboolean;
            use super::complex::Rcomplex;
            use super::r_dynload::DllInfo;

            #[cfg(target_os = "macos")]
            include!("bindings/bindings-Altrep-macos-aarch64.rs");

            #[allow(non_camel_case_types)]
            pub type R_altrep_Coerce_method_t =
                ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>;
        }

        #[path = "bindings-Rallocators-macos-aarch64.rs"]
        pub mod r_allocators;

        //     //TODO: this is windows specific.
        #[path = "bindings-libextern-macos-aarch64.rs"]
        pub mod libextern;
    }

    #[cfg(target_os = "macos")]
    #[path = "bindings-Rconfig-macos-aarch64.rs"]
    pub mod r_config;

    pub mod r_prelude {
        pub use super::r_config::*;
        pub use super::r_ext::arith::*;
        pub use super::r_ext::boolean::*;
        pub use super::r_ext::complex::*;
        pub use super::r_ext::constants::*;
        pub use super::r_ext::error::*;
        pub use super::r_ext::libextern::*;
        pub use super::r_ext::memory::*;
        pub use super::r_ext::print::*;
        pub use super::r_ext::random::*;
        pub use super::r_ext::rs::*;
        pub use super::r_ext::utils::*;
    }
}

#[cfg(test)]
mod tests;
