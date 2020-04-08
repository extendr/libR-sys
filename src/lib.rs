//
// This is a low-level libR binding library which is kept deliberately
// minimal.
//
// In particular, it has no external dependencies

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    //use super::{Rf_initialize_R, R_CStackLimit};
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
            // TODO: This has only been tested on the debian package
            // r-base-core.
            if cfg!(unix) {
                if std::env::var("R_HOME").is_err() {
                    std::env::set_var("R_HOME", "/usr/lib/R");
                }
            }

            // Due to Rf_initEmbeddedR using __libc_stack_end
            // We can't call Rf_initEmbeddedR.
            // Instead we must follow rustr's example and call the parts.

            //let res = unsafe { Rf_initEmbeddedR(1, args.as_mut_ptr()) };
            Rf_initialize_R(1, [cstr_mut!("R")].as_mut_ptr());

            // In case you are curious.
            // Maybe 8MB is a bit small.
            // eprintln!("R_CStackLimit={:016x}", R_CStackLimit);

            R_CStackLimit = usize::max_value();

            setup_Rmainloop();
        }
    }

    // Run some R code. Check the result.
    #[test]
    fn eval() {
        start_R();
        unsafe {
            let res = R_ParseEvalString(cstr!("1"), R_NilValue);
            assert_eq!(TYPEOF(res) as u32, REALSXP);
            assert_eq!(*REAL(res), 1.);
        }
    }
}
