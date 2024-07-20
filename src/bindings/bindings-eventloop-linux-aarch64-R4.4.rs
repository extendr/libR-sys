/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.4.1 */

pub const XActivity: u32 = 1;
pub const StdinActivity: u32 = 2;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type __time_t = ::std::os::raw::c_long;
pub type __suseconds_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type InputHandlerProc =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _InputHandler {
    pub activity: ::std::os::raw::c_int,
    pub fileDescriptor: ::std::os::raw::c_int,
    pub handler: InputHandlerProc,
    pub next: *mut _InputHandler,
    #[doc = "Whether we should be listening to this file descriptor or not."]
    pub active: ::std::os::raw::c_int,
    #[doc = "Data that can be passed to the routine as its only argument.\nThis might be a user-level function or closure when we implement\na callback to R mechanism."]
    pub userData: *mut ::std::os::raw::c_void,
}
pub type InputHandler = _InputHandler;
extern "C" {
    pub fn consoleInputHandler(buf: *mut ::std::os::raw::c_uchar, len: ::std::os::raw::c_int);
}
