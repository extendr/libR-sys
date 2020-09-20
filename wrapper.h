

// Define this for R_CStackLimit
//#define HAVE_UINTPTR_T
#define CSTACK_DEFNS

// Currently, I'm adding these on as-needed basis
// but we may simply throw the whole lot in in the future.
#include <Rinterface.h>
#include <Rinternals.h>
#include <Rembedded.h>
#include <R/Rversion.h>
#include <R_ext/Parse.h>
#include <R_ext/Error.h>
