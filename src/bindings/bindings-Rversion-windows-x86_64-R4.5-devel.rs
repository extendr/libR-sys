/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 16.0.6 */
/* r version: 4.5.0-devel */

pub const R_VERSION: u32 = 263424;
pub const R_NICK: &[u8; 24] = b"Unsuffered Consequences\0";
pub const R_MAJOR: &[u8; 2] = b"4\0";
pub const R_MINOR: &[u8; 4] = b"5.0\0";
pub const R_STATUS: &[u8; 29] = b"Under development (unstable)\0";
pub const R_YEAR: &[u8; 5] = b"2024\0";
pub const R_MONTH: &[u8; 3] = b"06\0";
pub const R_DAY: &[u8; 3] = b"08\0";
pub const R_SVN_REVISION: u32 = 86709;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
