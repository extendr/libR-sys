/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.4.1 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    pub fn Rf_endEmbeddedR(fatal: ::std::os::raw::c_int);
    pub fn Rf_initialize_R(
        ac: ::std::os::raw::c_int,
        av: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn setup_Rmainloop();
    pub fn R_RunExitFinalizers();
    pub fn CleanEd();
    pub fn R_CleanTempDir();
}
