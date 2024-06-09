/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Homebrew clang version 18.1.6 */
/* r version: 4.3.3 */

pub const R_VERSION: u32 = 262915;
pub const R_NICK: &[u8; 16] = b"Angel Food Cake\0";
pub const R_MAJOR: &[u8; 2] = b"4\0";
pub const R_MINOR: &[u8; 4] = b"3.3\0";
pub const R_STATUS: &[u8; 1] = b"\0";
pub const R_YEAR: &[u8; 5] = b"2024\0";
pub const R_MONTH: &[u8; 3] = b"02\0";
pub const R_DAY: &[u8; 3] = b"29\0";
pub const R_SVN_REVISION: u32 = 86002;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
