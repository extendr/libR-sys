/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.4.1 */

pub const GET_REGION_BUFSIZE: u32 = 512;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
