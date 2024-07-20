/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: clang version 18.1.6 */
/* r version: 4.2.3 */

pub const SINGLESXP: u32 = 302;
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
extern "C" {
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
}
