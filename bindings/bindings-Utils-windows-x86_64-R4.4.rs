/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.4.1 */

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
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
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
extern "C" {
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
    #[doc = "not API"]
    pub fn R_ExpandFileNameUTF8(
        arg1: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    #[doc = "Non-API, attribute_hidden and no longer used.  Will be removed in R 4.5.0."]
    pub fn Rf_setIVector(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_setRVector(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: f64);
    #[doc = "These two are guaranteed to use '.' as the decimal point,\nand to accept \"NA\". Documented since 4.4.0 patched."]
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
    #[doc = "../../appl/interv.c: first also in Applic.h"]
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
    #[doc = "not API, entry point no longer exists"]
    pub fn find_interv_vec(
        xt: *mut f64,
        n: *mut ::std::os::raw::c_int,
        x: *mut f64,
        nx: *mut ::std::os::raw::c_int,
        rightmost_closed: *mut ::std::os::raw::c_int,
        all_inside: *mut ::std::os::raw::c_int,
        indx: *mut ::std::os::raw::c_int,
    );
    #[doc = "../../appl/maxcol.c"]
    pub fn R_max_col(
        matrix: *mut f64,
        nr: *mut ::std::os::raw::c_int,
        nc: *mut ::std::os::raw::c_int,
        maxes: *mut ::std::os::raw::c_int,
        ties_meth: *mut ::std::os::raw::c_int,
    );
}
