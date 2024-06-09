/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Apple clang version 15.0.0 (clang-1500.3.9.4) */
/* r version: 4.5.0-devel */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Rboolean {
    #[doc = ", MAYBE"]
    FALSE = 0,
    #[doc = ", MAYBE"]
    TRUE = 1,
}
extern "C" {
    #[doc = "used by package tkrplot"]
    pub fn R_GetX11Image(
        d: ::std::os::raw::c_int,
        pximage: *mut ::std::os::raw::c_void,
        pwidth: *mut ::std::os::raw::c_int,
        pheight: *mut ::std::os::raw::c_int,
    ) -> Rboolean;
}
