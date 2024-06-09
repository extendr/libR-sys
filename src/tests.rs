use bindings::{
    r_embedded::Rf_initialize_R,
    r_ext::r_startup::setup_Rmainloop,
    r_interface::R_CStackLimit,
    r_internals::{
        R_GlobalEnv, R_NilValue, R_ParseEvalString, Rf_PrintValue, Rf_isS4, Rf_protect,
        Rf_unprotect, REAL, SEXPTYPE, TYPEOF,
    },
    r_prelude::Rboolean,
};

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
