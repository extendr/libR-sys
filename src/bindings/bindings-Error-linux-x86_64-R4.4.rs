/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.4.0 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type va_list = __builtin_va_list;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
extern "C" {
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;
    pub fn UNIMPLEMENTED(arg1: *const ::std::os::raw::c_char) -> !;
    pub fn WrongArgCount(arg1: *const ::std::os::raw::c_char) -> !;
    pub fn Rf_warning(arg1: *const ::std::os::raw::c_char, ...);
    pub fn R_ShowMessage(s: *const ::std::os::raw::c_char);
}
