/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Apple clang version 15.0.0 (clang-1500.3.9.4) */
/* r version: 4.5.0-devel */

pub const HAVE_F77_UNDERSCORE: u32 = 1;
pub const IEEE_754: u32 = 1;
pub const SUPPORT_UTF8: u32 = 1;
pub const SUPPORT_MBCS: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const HAVE_AQUA: u32 = 1;
pub const PR18534fixed: u32 = 1;
pub const SIZEOF_SIZE_T: u32 = 8;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const HAVE_UINTPTR_T: u32 = 1;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    #[doc = "not of themselves API"]
    pub fn R_chk_calloc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_free(arg1: *mut ::std::os::raw::c_void);
}
