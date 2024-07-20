/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.3.3 */

pub const R_CONNECTIONS_VERSION: u32 = 1;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type Rconnection = *mut Rconn;
#[repr(C)]
pub struct Rconn {
    pub class: *mut ::std::os::raw::c_char,
    pub description: *mut ::std::os::raw::c_char,
    #[doc = "the encoding of 'description'"]
    pub enc: ::std::os::raw::c_int,
    pub mode: [::std::os::raw::c_char; 5usize],
    pub text: Rboolean,
    pub isopen: Rboolean,
    pub incomplete: Rboolean,
    pub canread: Rboolean,
    pub canwrite: Rboolean,
    pub canseek: Rboolean,
    pub blocking: Rboolean,
    pub isGzcon: Rboolean,
    pub open: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> Rboolean>,
    #[doc = "routine closing after auto open"]
    pub close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    #[doc = "when closing connection"]
    pub destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    pub vfprintf: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Rconn,
            arg2: *const ::std::os::raw::c_char,
            arg3: va_list,
        ) -> ::std::os::raw::c_int,
    >,
    pub fgetc:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub fgetc_internal:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Rconn,
            arg2: f64,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub truncate: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    pub fflush:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: usize,
            arg3: usize,
            arg4: *mut Rconn,
        ) -> usize,
    >,
    pub write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_void,
            arg2: usize,
            arg3: usize,
            arg4: *mut Rconn,
        ) -> usize,
    >,
    #[doc = "number of lines, position on top line"]
    pub nPushBack: ::std::os::raw::c_int,
    #[doc = "number of lines, position on top line"]
    pub posPushBack: ::std::os::raw::c_int,
    pub PushBack: *mut *mut ::std::os::raw::c_char,
    pub save: ::std::os::raw::c_int,
    pub save2: ::std::os::raw::c_int,
    pub encname: [::std::os::raw::c_char; 101usize],
    #[doc = "will be iconv_t, which is a pointer. NULL if not in use"]
    pub inconv: *mut ::std::os::raw::c_void,
    #[doc = "will be iconv_t, which is a pointer. NULL if not in use"]
    pub outconv: *mut ::std::os::raw::c_void,
    #[doc = "The idea here is that no MBCS char will ever not fit"]
    pub iconvbuff: [::std::os::raw::c_char; 25usize],
    #[doc = "The idea here is that no MBCS char will ever not fit"]
    pub oconvbuff: [::std::os::raw::c_char; 50usize],
    #[doc = "The idea here is that no MBCS char will ever not fit"]
    pub next: *mut ::std::os::raw::c_char,
    #[doc = "The idea here is that no MBCS char will ever not fit"]
    pub init_out: [::std::os::raw::c_char; 25usize],
    pub navail: ::std::os::raw::c_short,
    pub inavail: ::std::os::raw::c_short,
    pub EOF_signalled: Rboolean,
    pub UTF8out: Rboolean,
    pub id: *mut ::std::os::raw::c_void,
    pub ex_ptr: *mut ::std::os::raw::c_void,
    pub private: *mut ::std::os::raw::c_void,
    #[doc = "for pipes etc"]
    pub status: ::std::os::raw::c_int,
    pub buff: *mut ::std::os::raw::c_uchar,
    pub buff_len: usize,
    pub buff_stored_len: usize,
    pub buff_pos: usize,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
