#ifndef WRAPPER_COMPLEX_H
#define WRAPPER_COMPLEX_H
#include <R_ext/Complex.h>

Rcomplex create_rcomplex(double real, double imag);
double Rcomplex_real(const Rcomplex *z);
double Rcomplex_imaginary(const Rcomplex *z);
void Rcomplex_set_real(Rcomplex *z, double real);
void Rcomplex_set_imaginary(Rcomplex *z, double imaginary);

/// Returns [`Rcomplex`] from real part and imaginary part.
Rcomplex create_rcomplex(double real, double imag) {
  return (Rcomplex) { .r = real, .i = imag };
}

/// Returns the real part
double Rcomplex_real(const Rcomplex *z) {
    return z->r;
}

/// Returns the imaginary part
double Rcomplex_imaginary(const Rcomplex *z) {
    return z->i;
}

/// Set the real part
void Rcomplex_set_real(Rcomplex *z, double real) {
    z->r = real;
}

/// Set the imaginary part
void Rcomplex_set_imaginary(Rcomplex *z, double imaginary) {
    z->i = imaginary;
}

#endif /* WRAPPER_COMPLEX_H */