/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Homebrew clang version 18.1.8 */
/* r version: 4.4.1 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    pub fn Riconv_open(
        tocode: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn Riconv(
        cd: *mut ::std::os::raw::c_void,
        inbuf: *mut *const ::std::os::raw::c_char,
        inbytesleft: *mut usize,
        outbuf: *mut *mut ::std::os::raw::c_char,
        outbytesleft: *mut usize,
    ) -> usize;
    pub fn Riconv_close(cd: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
