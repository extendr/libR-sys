/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.2.3 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type SEXP = *mut SEXPREC;
extern "C" {
    pub fn formatLogicalS(arg1: SEXP, arg2: R_xlen_t, arg3: *mut ::std::os::raw::c_int);
    pub fn formatIntegerS(arg1: SEXP, arg2: R_xlen_t, arg3: *mut ::std::os::raw::c_int);
    pub fn formatRealS(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    );
    pub fn formatComplexS(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut ::std::os::raw::c_int,
        arg9: ::std::os::raw::c_int,
    );
    pub fn Rf_EncodeReal0(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    #[doc = "Printing"]
    pub fn Rf_IndexWidth(arg1: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn Rf_VectorIndex(arg1: R_xlen_t, arg2: ::std::os::raw::c_int);
    #[doc = "void printLogicalVector(int *, R_xlen_t, int);"]
    pub fn Rf_printIntegerVector(
        arg1: *const ::std::os::raw::c_int,
        arg2: R_xlen_t,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_printComplexVector(
        arg1: *const Rcomplex,
        arg2: R_xlen_t,
        arg3: ::std::os::raw::c_int,
    );
    pub fn printIntegerVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
    pub fn printRealVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
    pub fn printComplexVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
}
