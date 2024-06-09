/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Ubuntu clang version 15.0.7 */
/* r version: 4.5.0-devel */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type va_list = [u64; 4usize];
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[repr(C)]
pub struct R_altrep_class_t {
    pub ptr: SEXP,
}
pub type R_altrep_UnserializeEX_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: SEXP,
        arg3: SEXP,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> SEXP,
>;
pub type R_altrep_Unserialize_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>;
pub type R_altrep_Serialized_state_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> SEXP>;
pub type R_altrep_DuplicateEX_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Duplicate_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Inspect_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ),
        >,
    ) -> Rboolean,
>;
pub type R_altrep_Length_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> R_xlen_t>;
pub type R_altvec_Dataptr_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> *mut ::std::os::raw::c_void,
>;
pub type R_altvec_Dataptr_or_null_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> *const ::std::os::raw::c_void>;
pub type R_altvec_Extract_subset_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP>;
pub type R_altinteger_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
pub type R_altinteger_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
pub type R_altinteger_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> f64>;
pub type R_altreal_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut f64) -> R_xlen_t,
>;
pub type R_altreal_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altreal_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altreal_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altlogical_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
pub type R_altlogical_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
pub type R_altlogical_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlogical_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlogical_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altraw_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rbyte>;
pub type R_altraw_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut Rbyte) -> R_xlen_t,
>;
pub type R_altcomplex_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rcomplex>;
pub type R_altcomplex_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut Rcomplex,
    ) -> R_xlen_t,
>;
pub type R_altstring_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
pub type R_altstring_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;
pub type R_altstring_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altstring_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlist_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
pub type R_altlist_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;
extern "C" {
    pub fn R_new_altrep(aclass: R_altrep_class_t, data1: SEXP, data2: SEXP) -> SEXP;
    pub fn R_make_altstring_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altinteger_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altreal_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altlogical_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altraw_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altcomplex_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altlist_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_altrep_inherits(x: SEXP, arg1: R_altrep_class_t) -> Rboolean;
    pub fn R_set_altrep_UnserializeEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_UnserializeEX_method_t,
    );
    pub fn R_set_altrep_Unserialize_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Unserialize_method_t,
    );
    pub fn R_set_altrep_Serialized_state_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Serialized_state_method_t,
    );
    pub fn R_set_altrep_DuplicateEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_DuplicateEX_method_t,
    );
    pub fn R_set_altrep_Duplicate_method(cls: R_altrep_class_t, fun: R_altrep_Duplicate_method_t);
    pub fn R_set_altrep_Coerce_method(cls: R_altrep_class_t, fun: R_altrep_Coerce_method_t);
    pub fn R_set_altrep_Inspect_method(cls: R_altrep_class_t, fun: R_altrep_Inspect_method_t);
    pub fn R_set_altrep_Length_method(cls: R_altrep_class_t, fun: R_altrep_Length_method_t);
    pub fn R_set_altvec_Dataptr_method(cls: R_altrep_class_t, fun: R_altvec_Dataptr_method_t);
    pub fn R_set_altvec_Dataptr_or_null_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Dataptr_or_null_method_t,
    );
    pub fn R_set_altvec_Extract_subset_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Extract_subset_method_t,
    );
    pub fn R_set_altinteger_Elt_method(cls: R_altrep_class_t, fun: R_altinteger_Elt_method_t);
    pub fn R_set_altinteger_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Get_region_method_t,
    );
    pub fn R_set_altinteger_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Is_sorted_method_t,
    );
    pub fn R_set_altinteger_No_NA_method(cls: R_altrep_class_t, fun: R_altinteger_No_NA_method_t);
    pub fn R_set_altinteger_Sum_method(cls: R_altrep_class_t, fun: R_altinteger_Sum_method_t);
    pub fn R_set_altinteger_Min_method(cls: R_altrep_class_t, fun: R_altinteger_Min_method_t);
    pub fn R_set_altinteger_Max_method(cls: R_altrep_class_t, fun: R_altinteger_Max_method_t);
    pub fn R_set_altreal_Elt_method(cls: R_altrep_class_t, fun: R_altreal_Elt_method_t);
    pub fn R_set_altreal_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altreal_Get_region_method_t,
    );
    pub fn R_set_altreal_Is_sorted_method(cls: R_altrep_class_t, fun: R_altreal_Is_sorted_method_t);
    pub fn R_set_altreal_No_NA_method(cls: R_altrep_class_t, fun: R_altreal_No_NA_method_t);
    pub fn R_set_altreal_Sum_method(cls: R_altrep_class_t, fun: R_altreal_Sum_method_t);
    pub fn R_set_altreal_Min_method(cls: R_altrep_class_t, fun: R_altreal_Min_method_t);
    pub fn R_set_altreal_Max_method(cls: R_altrep_class_t, fun: R_altreal_Max_method_t);
    pub fn R_set_altlogical_Elt_method(cls: R_altrep_class_t, fun: R_altlogical_Elt_method_t);
    pub fn R_set_altlogical_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Get_region_method_t,
    );
    pub fn R_set_altlogical_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Is_sorted_method_t,
    );
    pub fn R_set_altlogical_No_NA_method(cls: R_altrep_class_t, fun: R_altlogical_No_NA_method_t);
    pub fn R_set_altlogical_Sum_method(cls: R_altrep_class_t, fun: R_altlogical_Sum_method_t);
    pub fn R_set_altraw_Elt_method(cls: R_altrep_class_t, fun: R_altraw_Elt_method_t);
    pub fn R_set_altraw_Get_region_method(cls: R_altrep_class_t, fun: R_altraw_Get_region_method_t);
    pub fn R_set_altcomplex_Elt_method(cls: R_altrep_class_t, fun: R_altcomplex_Elt_method_t);
    pub fn R_set_altcomplex_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altcomplex_Get_region_method_t,
    );
    pub fn R_set_altstring_Elt_method(cls: R_altrep_class_t, fun: R_altstring_Elt_method_t);
    pub fn R_set_altstring_Set_elt_method(cls: R_altrep_class_t, fun: R_altstring_Set_elt_method_t);
    pub fn R_set_altstring_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altstring_Is_sorted_method_t,
    );
    pub fn R_set_altstring_No_NA_method(cls: R_altrep_class_t, fun: R_altstring_No_NA_method_t);
    pub fn R_set_altlist_Elt_method(cls: R_altrep_class_t, fun: R_altlist_Elt_method_t);
    pub fn R_set_altlist_Set_elt_method(cls: R_altrep_class_t, fun: R_altlist_Set_elt_method_t);
}
