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
pub const HAVE_F77_UNDERSCORE: u32 = 1;
pub const IEEE_754: u32 = 1;
pub const SUPPORT_UTF8: u32 = 1;
pub const SUPPORT_MBCS: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const PR18534fixed: u32 = 1;
pub const SIZEOF_SIZE_T: u32 = 8;
pub const HAVE_UINTPTR_T: u32 = 1;
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
#[doc = "../../appl/integrate.c"]
pub type integr_fn = ::std::option::Option<
    unsafe extern "C" fn(x: *mut f64, n: ::std::os::raw::c_int, ex: *mut ::std::os::raw::c_void),
>;
#[doc = "main/optim.c"]
pub type optimfn = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut ::std::os::raw::c_void,
    ) -> f64,
>;
pub type optimgr = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = "type of pointer to the target and gradient functions"]
pub type fcn_p = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = "type of pointer to the hessian functions"]
pub type d2fcn_p = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    #[doc = "S Like Memory Management"]
    pub fn R_chk_calloc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_free(arg1: *mut ::std::os::raw::c_void);
    pub fn dasum_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn daxpy_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn dcopy_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn ddot_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *const f64,
        incy: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dnrm2_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn drot_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotg_(a: *const f64, b: *const f64, c: *mut f64, s: *mut f64);
    pub fn drotm_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        dparam: *const f64,
    );
    pub fn drotmg_(
        dd1: *const f64,
        dd2: *const f64,
        dx1: *const f64,
        dy1: *const f64,
        param: *mut f64,
    );
    pub fn dscal_(
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn dswap_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn idamax_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn dgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        kl: *const ::std::os::raw::c_int,
        ku: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        ap: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsymv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dtbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dger_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn dsyr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dsyr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dtrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dtrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dcabs1_(z: *const Rcomplex) -> f64;
    pub fn dzasum_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dznrm2_(
        n: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn izamax_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn zaxpy_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zcopy_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zdotc_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdotu_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdrot_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn zdscal_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        kl: *mut ::std::os::raw::c_int,
        ku: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        y: *mut Rcomplex,
        incy: *mut ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgerc_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zgeru_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zhbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhemm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhemv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zherk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhpmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        ap: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhpr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zhpr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zrotg_(ca: *const Rcomplex, cb: *const Rcomplex, c: *mut f64, s: *mut Rcomplex);
    pub fn zscal_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zswap_(
        n: *const ::std::os::raw::c_int,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        c: *mut Rcomplex,
        ldc: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn ztbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    #[doc = "vectorizing function   f(x\\[1:n\\], ...) -> x\\[\\]  {overwriting x\\[\\]}."]
    pub fn Rdqags(
        f: integr_fn,
        ex: *mut ::std::os::raw::c_void,
        a: *mut f64,
        b: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut ::std::os::raw::c_int,
        ier: *mut ::std::os::raw::c_int,
        limit: *mut ::std::os::raw::c_int,
        lenw: *mut ::std::os::raw::c_int,
        last: *mut ::std::os::raw::c_int,
        iwork: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn Rdqagi(
        f: integr_fn,
        ex: *mut ::std::os::raw::c_void,
        bound: *mut f64,
        inf: *mut ::std::os::raw::c_int,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut ::std::os::raw::c_int,
        ier: *mut ::std::os::raw::c_int,
        limit: *mut ::std::os::raw::c_int,
        lenw: *mut ::std::os::raw::c_int,
        last: *mut ::std::os::raw::c_int,
        iwork: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn vmmin(
        n: ::std::os::raw::c_int,
        b: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        maxit: ::std::os::raw::c_int,
        trace: ::std::os::raw::c_int,
        mask: *mut ::std::os::raw::c_int,
        abstol: f64,
        reltol: f64,
        nREPORT: ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        fail: *mut ::std::os::raw::c_int,
    );
    pub fn nmmin(
        n: ::std::os::raw::c_int,
        Bvec: *mut f64,
        X: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        fail: *mut ::std::os::raw::c_int,
        abstol: f64,
        intol: f64,
        ex: *mut ::std::os::raw::c_void,
        alpha: f64,
        bet: f64,
        gamm: f64,
        trace: ::std::os::raw::c_int,
        fncount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
    );
    pub fn cgmin(
        n: ::std::os::raw::c_int,
        Bvec: *mut f64,
        X: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        fail: *mut ::std::os::raw::c_int,
        abstol: f64,
        intol: f64,
        ex: *mut ::std::os::raw::c_void,
        type_: ::std::os::raw::c_int,
        trace: ::std::os::raw::c_int,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
    );
    pub fn lbfgsb(
        n: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        x: *mut f64,
        l: *mut f64,
        u: *mut f64,
        nbd: *mut ::std::os::raw::c_int,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        fail: *mut ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
        factr: f64,
        pgtol: f64,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_char,
        trace: ::std::os::raw::c_int,
        nREPORT: ::std::os::raw::c_int,
    );
    pub fn samin(
        n: ::std::os::raw::c_int,
        pb: *mut f64,
        yb: *mut f64,
        fn_: optimfn,
        maxit: ::std::os::raw::c_int,
        tmax: ::std::os::raw::c_int,
        ti: f64,
        trace: ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
    );
    #[doc = "appl/interv.c: Also in Utils.h, used in package eco"]
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn dqrqty_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        qty: *mut f64,
    );
    pub fn dqrqy_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        qy: *mut f64,
    );
    pub fn dqrcf_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        b: *mut f64,
        info: *mut ::std::os::raw::c_int,
    );
    #[doc = "appl/pretty.c: for use in engine.c and util.c"]
    pub fn R_pretty(
        lo: *mut f64,
        up: *mut f64,
        ndiv: *mut ::std::os::raw::c_int,
        min_n: ::std::os::raw::c_int,
        shrink_sml: f64,
        high_u_fact: *const f64,
        eps_correction: ::std::os::raw::c_int,
        return_bounds: ::std::os::raw::c_int,
    ) -> f64;
    #[doc = "Also used in packages nlme, pcaPP"]
    pub fn optif9(
        nr: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        x: *mut f64,
        fcn: fcn_p,
        d1fcn: fcn_p,
        d2fcn: d2fcn_p,
        state: *mut ::std::os::raw::c_void,
        typsiz: *mut f64,
        fscale: f64,
        method: ::std::os::raw::c_int,
        iexp: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_int,
        ndigit: ::std::os::raw::c_int,
        itnlim: ::std::os::raw::c_int,
        iagflg: ::std::os::raw::c_int,
        iahflg: ::std::os::raw::c_int,
        dlt: f64,
        gradtl: f64,
        stepmx: f64,
        steptl: f64,
        xpls: *mut f64,
        fpls: *mut f64,
        gpls: *mut f64,
        itrmcd: *mut ::std::os::raw::c_int,
        a: *mut f64,
        wrk: *mut f64,
        itncnt: *mut ::std::os::raw::c_int,
    );
    pub fn dqrdc2_(
        x: *mut f64,
        ldx: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        p: *mut ::std::os::raw::c_int,
        tol: *mut f64,
        rank: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        pivot: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn dqrls_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        p: *mut ::std::os::raw::c_int,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        tol: *mut f64,
        b: *mut f64,
        rsd: *mut f64,
        qty: *mut f64,
        k: *mut ::std::os::raw::c_int,
        jpvt: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        work: *mut f64,
    );
}