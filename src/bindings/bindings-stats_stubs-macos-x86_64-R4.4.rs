/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Homebrew clang version 18.1.6 */
/* r version: 4.4.0 */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
    pub fn S_Rf_divset(
        alg: ::std::os::raw::c_int,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        v: *mut f64,
    );
    pub fn S_nlminb_iterate(
        b: *mut f64,
        d: *mut f64,
        fx: f64,
        g: *mut f64,
        h: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_nlsb_iterate(
        b: *mut f64,
        d: *mut f64,
        dr: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nd: ::std::os::raw::c_int,
        p: ::std::os::raw::c_int,
        r: *mut f64,
        rd: *mut f64,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_rcont2(
        nrow: ::std::os::raw::c_int,
        ncol: ::std::os::raw::c_int,
        nrowt: *const ::std::os::raw::c_int,
        ncolt: *const ::std::os::raw::c_int,
        ntotal: ::std::os::raw::c_int,
        fact: *const f64,
        jwork: *mut ::std::os::raw::c_int,
        matrix: *mut ::std::os::raw::c_int,
    );
}