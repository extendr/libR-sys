/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.4.1 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
extern "C" {
    pub fn vmaxget() -> *mut ::std::os::raw::c_void;
    pub fn vmaxset(arg1: *const ::std::os::raw::c_void);
    pub fn R_gc();
    pub fn R_gc_running() -> ::std::os::raw::c_int;
    pub fn R_alloc(arg1: usize, arg2: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn R_allocLD(nelem: usize) -> *mut u128;
    pub fn S_alloc(
        arg1: ::std::os::raw::c_long,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn S_realloc(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_long,
        arg3: ::std::os::raw::c_long,
        arg4: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_malloc_gc(arg1: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_calloc_gc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_realloc_gc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
}
