

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

// todo: what is this #define?
#ifdef MAC_STUFF
  #include <R/Rversion.h>
#else
  #include <Rversion.h>
#endif
#include <R_ext/Parse.h>
#include <R_ext/Error.h>
