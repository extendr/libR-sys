/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.2.3 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    #[doc = "IEEE NaN"]
    pub static mut R_NaN: f64;
    #[doc = "IEEE Inf"]
    pub static mut R_PosInf: f64;
    #[doc = "IEEE -Inf"]
    pub static mut R_NegInf: f64;
    #[doc = "NA_REAL: IEEE"]
    pub static mut R_NaReal: f64;
    #[doc = "NA_INTEGER:= INT_MIN currently"]
    pub static mut R_NaInt: ::std::os::raw::c_int;
    #[doc = "NA_STRING is a SEXP, so defined in Rinternals.h"]
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
}
