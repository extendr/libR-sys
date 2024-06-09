/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 16.0.6 */
/* r version: 4.3.3 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const SINGLESXP: u32 = 302;
pub const HAVE_F77_UNDERSCORE: u32 = 1;
pub const IEEE_754: u32 = 1;
pub const SUPPORT_UTF8: u32 = 1;
pub const SUPPORT_MBCS: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const PR18534fixed: u32 = 1;
pub const SIZEOF_SIZE_T: u32 = 8;
pub const HAVE_UINTPTR_T: u32 = 1;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::std::os::raw::c_void,
}
pub type FILE = _iobuf;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Rboolean {
    #[doc = ", MAYBE"]
    FALSE = 0,
    #[doc = ", MAYBE"]
    TRUE = 1,
}
#[repr(C)]
pub struct Rcomplex {
    pub __bindgen_anon_1: __BindgenUnionField<Rcomplex__bindgen_ty_1>,
    pub private_data_c: __BindgenUnionField<__BindgenComplex<f64>>,
    pub bindgen_union_field: [u64; 2usize],
}
#[doc = "Called with a variable argument set after casting to a compatible\nfunction pointer."]
pub type DL_FUNC = ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>;
pub type R_NativePrimitiveArgType = ::std::os::raw::c_uint;
#[doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
    pub types: *mut R_NativePrimitiveArgType,
}
#[doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."]
pub type R_FortranMethodDef = R_CMethodDef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CallMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
}
pub type R_ExternalMethodDef = R_CallMethodDef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DllInfo {
    _unused: [u8; 0],
}
pub type DllInfo = _DllInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rf_RegisteredNativeSymbol {
    _unused: [u8; 0],
}
pub type R_RegisteredNativeSymbol = Rf_RegisteredNativeSymbol;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NativeSymbolType {
    R_ANY_SYM = 0,
    R_C_SYM = 1,
    R_CALL_SYM = 2,
    R_FORTRAN_SYM = 3,
    R_EXTERNAL_SYM = 4,
}
#[repr(u32)]
#[doc = "PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
extern "C" {
    #[doc = "IEEE NaN"]
    pub static mut R_NaN: f64;
    #[doc = "IEEE Inf"]
    pub static mut R_PosInf: f64;
    #[doc = "IEEE -Inf"]
    pub static mut R_NegInf: f64;
    #[doc = "NA_REAL: IEEE"]
    pub static mut R_NaReal: f64;
    #[doc = "NA_INTEGER:= INT_MIN currently"]
    pub static mut R_NaInt: ::std::os::raw::c_int;
    #[doc = "NA_STRING is a SEXP, so defined in Rinternals.h"]
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;
    pub fn UNIMPLEMENTED(arg1: *const ::std::os::raw::c_char) -> !;
    pub fn WrongArgCount(arg1: *const ::std::os::raw::c_char) -> !;
    pub fn Rf_warning(arg1: *const ::std::os::raw::c_char, ...);
    pub fn R_ShowMessage(s: *const ::std::os::raw::c_char);
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
    #[doc = "../../main/sort.c :"]
    pub fn R_isort(arg1: *mut ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
    pub fn R_rsort(arg1: *mut f64, arg2: ::std::os::raw::c_int);
    pub fn R_csort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int);
    pub fn rsort_with_index(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_revsort(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_iPsort(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_rPsort(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
    pub fn Rf_cPsort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
    #[doc = "../../main/qsort.c : */\n/* dummy renamed to II to avoid problems with g++ on Solaris"]
    pub fn R_qsort(v: *mut f64, i: usize, j: usize);
    pub fn R_qsort_I(
        v: *mut f64,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
    pub fn R_qsort_int(iv: *mut ::std::os::raw::c_int, i: usize, j: usize);
    pub fn R_qsort_int_I(
        iv: *mut ::std::os::raw::c_int,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
    #[doc = "../../main/util.c  and others :"]
    pub fn R_ExpandFileName(arg1: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
    pub fn Rf_setIVector(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_setRVector(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: f64);
    #[doc = "These two are guaranteed to use '.' as the decimal point,\nand to accept \"NA\"."]
    pub fn R_atof(str_: *const ::std::os::raw::c_char) -> f64;
    pub fn R_strtod(c: *const ::std::os::raw::c_char, end: *mut *mut ::std::os::raw::c_char)
        -> f64;
    pub fn R_tmpnam(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_tmpnam2(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
        fileext: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_free_tmpnam(name: *mut ::std::os::raw::c_char);
    pub fn R_CheckUserInterrupt();
    pub fn R_CheckStack();
    pub fn R_CheckStack2(arg1: usize);
    #[doc = "../../appl/interv.c: also in Applic.h"]
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn findInterval2(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        left_open: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn find_interv_vec(
        xt: *mut f64,
        n: *mut ::std::os::raw::c_int,
        x: *mut f64,
        nx: *mut ::std::os::raw::c_int,
        rightmost_closed: *mut ::std::os::raw::c_int,
        all_inside: *mut ::std::os::raw::c_int,
        indx: *mut ::std::os::raw::c_int,
    );
    #[doc = "../../appl/maxcol.c: also in Applic.h"]
    pub fn R_max_col(
        matrix: *mut f64,
        nr: *mut ::std::os::raw::c_int,
        nc: *mut ::std::os::raw::c_int,
        maxes: *mut ::std::os::raw::c_int,
        ties_meth: *mut ::std::os::raw::c_int,
    );
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn Rvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
    pub fn REvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
    pub fn R_registerRoutines(
        info: *mut DllInfo,
        croutines: *const R_CMethodDef,
        callRoutines: *const R_CallMethodDef,
        fortranRoutines: *const R_FortranMethodDef,
        externalRoutines: *const R_ExternalMethodDef,
    ) -> ::std::os::raw::c_int;
    pub fn R_useDynamicSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_forceSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_getDllInfo(name: *const ::std::os::raw::c_char) -> *mut DllInfo;
    #[doc = "To be used by applications embedding R to register their symbols\nthat are not related to any dynamic module"]
    pub fn R_getEmbeddingDllInfo() -> *mut DllInfo;
    pub fn R_FindSymbol(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        symbol: *mut R_RegisteredNativeSymbol,
    ) -> DL_FUNC;
    #[doc = "Interface for exporting and importing functions from one package\nfor use from C code in a package.  The registration part probably\nought to be integrated with the other registrations.  The naming of\nthese routines may be less than ideal."]
    pub fn R_RegisterCCallable(
        package: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        fptr: DL_FUNC,
    );
    pub fn R_GetCCallable(
        package: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> DL_FUNC;
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
}
