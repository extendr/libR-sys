/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.4.1 */

pub const __STDC_WANT_IEC_60559_FUNCS_EXT__: u32 = 1;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    pub fn R_FlushConsole();
    #[doc = "always declared, but only usable under Win32 and Aqua"]
    pub fn R_ProcessEvents();
    pub fn R_WaitEvent();
}
