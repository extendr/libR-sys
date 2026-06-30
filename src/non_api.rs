//! R "non-API" entry points, gated behind the off-by-default `non-api` feature.
//!
//! These are not part of R's stable C API; some are not exported by every R
//! version (e.g. `SET_FORMALS`, `SET_BODY`, `SET_CLOENV`, `PRSEEN`,
//! `Rf_isValidStringF` are not exported by R 4.6), so calling them may fail to
//! link depending on the R you build against. Use with care.
//!
//! Ported from `extendr-ffi` (`src/non_api.rs`).
use crate::{Rboolean, SEXP, SEXPTYPE};
extern "C" {
    pub fn Rf_isValidString(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidStringF(arg1: SEXP) -> Rboolean;
    pub fn SYMVALUE(x: SEXP) -> SEXP;
    pub fn PRSEEN(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_ENCLOS(x: SEXP, v: SEXP);
    pub fn ENVFLAGS(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_ENVFLAGS(x: SEXP, v: ::std::os::raw::c_int);
    pub fn ENCLOS(x: SEXP) -> SEXP;
    pub fn HASHTAB(x: SEXP) -> SEXP;
    pub fn FRAME(x: SEXP) -> SEXP;
    pub fn Rf_allocSExp(arg1: SEXPTYPE) -> SEXP;
    pub fn SET_FORMALS(x: SEXP, v: SEXP);
    pub fn SET_BODY(x: SEXP, v: SEXP);
    pub fn SET_CLOENV(x: SEXP, v: SEXP);
    pub fn SET_PRCODE(x: SEXP, v: SEXP);
    pub fn SET_PRENV(x: SEXP, v: SEXP);
    pub fn SET_PRVALUE(x: SEXP, v: SEXP);
    #[doc = "Promise Access Functions"]
    pub fn PRCODE(x: SEXP) -> SEXP;
    pub fn PRENV(x: SEXP) -> SEXP;
    pub fn PRVALUE(x: SEXP) -> SEXP;
}
