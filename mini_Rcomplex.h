
// R 4.3 redefined `Rcomplex` to a union for compatibility with Fortran.
// But the old definition is compatible both the union version
// and the struct version.
// See: <https://github.com/extendr/extendr/issues/524>
/// <div rustbindgen replaces="Rcomplex"></div>
typedef struct
{
    double r;
    double i;
} R_complex_impl;
