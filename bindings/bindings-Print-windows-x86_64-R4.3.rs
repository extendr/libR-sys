/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.3.3 */

pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn Rvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
    pub fn REvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
}
