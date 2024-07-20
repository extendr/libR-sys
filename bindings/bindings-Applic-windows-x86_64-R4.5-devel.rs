/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.5.0-devel */

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
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
    #[doc = "appl/interv.c: Also in Utils.h, used in former package eco"]
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    #[doc = "appl/dqrutl.f: interfaces to dqrsl"]
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
    #[doc = "appl/pretty.c: for use in engine.c and util.c\nFIXME: move out of this header"]
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
    #[doc = "Foremerly used in package nlme, still used by pcaPP"]
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
    #[doc = "find qr decomposition, dqrdc2() is basis of R's qr(),\nalso used by nlme and many other packages."]
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
