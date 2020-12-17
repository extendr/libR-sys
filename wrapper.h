#include <stddef.h> // for ptrdiff_t

// R_xlen_t is defined as int on 32-bit platforms, and
// that confuses Rust. Keeping it always as ptrdiff_t works
// fine even on 32-bit.
/// <div rustbindgen replaces="R_xlen_t"></div>
typedef ptrdiff_t R_xlen_t_rust;

// Define this for R_CStackLimit
// #define HAVE_UINTPTR_T
#define CSTACK_DEFNS

// Currently, I'm adding these on as-needed basis
// but we may simply throw the whole lot in in the future.
#include <Rinternals.h>

#ifndef _WIN32
  #define R_INTERFACE_PTRS
  #include <Rinterface.h>
#else
    extern uintptr_t R_CStackLimit; /* C stack limit */
#endif
#include <Rembedded.h>

#include <Rversion.h>
#include <R_ext/Parse.h>
#include <R_ext/Error.h>
