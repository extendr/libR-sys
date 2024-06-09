/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.3.3 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
#[doc = "R 4.3 redefined `Rcomplex` to a union for compatibility with Fortran.\n But the old definition is compatible both the union version\n and the struct version.\n See: <https://github.com/extendr/extendr/issues/524>\n <div rustbindgen replaces=\"Rcomplex\"></div>"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex {
    pub r: f64,
    pub i: f64,
}
