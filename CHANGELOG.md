# Changelog

## libR-sys 0.6.0

- Drop support for 32-bit Windows, which virtually means dropping support for R < 4.2.
- Fix failures with R installation that doesn't provide shared libraries.

## libR-sys 0.5.0

- Update the default precomputed bindings to R 4.3.0.
- Remove deprecated API `VECTOR_PTR` from bindings. [[#165]](https://github.com/extendr/libR-sys/pull/165)
- Use Rust types such as `u32` and `i16` instead of C types such as `c_uint` and `c_short` for `enum`s. [[#160]](https://github.com/extendr/libR-sys/pull/160)
- Remove mathematical constants (e.g., `M_PI`), which are also provided by Rust's `std` (e.g., `std::f64::consts::PI`). [[#160]](https://github.com/extendr/libR-sys/pull/160)
- Exclude "non-API" calls. [[#135]](https://github.com/extendr/libR-sys/pull/135)
- Refactor version logging in generated bindings. [[#159]](https://github.com/extendr/libR-sys/pull/159)
- Redefine R-devel's `Rcomplex`. [[#156]](https://github.com/extendr/libR-sys/pull/156)

## libR-sys 0.4.0

- Update the default precomuted bindings to R 4.2.0.
- [`Windows`] Dropped build-time dependency on `winapi`.
- Remove bindings for the symbols that are not part of R API. [[#96]](https://github.com/extendr/libR-sys/pull/96)
- Add bindings for the following header files:
  - `R_ext/Applic.h`: optimisation functions [[#117]](https://github.com/extendr/libR-sys/pull/117)
  - `R_ext/Random.h`: random number generator state wrappers [[#123]](https://github.com/extendr/libR-sys/pull/123)
  - `Rmath.h`: distribution functions [[#124]](https://github.com/extendr/libR-sys/pull/124)
- [`Linux`] Provide precomuted bindings for linux-aarch64 (aka ARM64). [[#133]](https://github.com/extendr/libR-sys/pull/133)

## libR-sys 0.3.0

- Drop support for 32-bit Windows with R >= 4.2. As
  [the release note of R 4.1.0](https://stat.ethz.ch/pipermail/r-announce/2021/000670.html)
  announced "the 4.1.x series will be the last to support 32-bit Windows,"
  there will be no 32-bit version of R as of R 4.2.0.  
  To be clear, libR-sys (and extendr) crate will keep supporting 32-bit on R <
  4.2 for a year or so.
- libR-sys no longer sets `DEP_R_R_VERSION_STRING` environmental variable.

## libR-sys 0.2.2

- Update the default precomuted bindings to R 4.1.0.
- Provide bindings for `R_ext/Altrep.h` and `R_ext/GraphicsEngine.h`.

## libR-sys 0.2.1

- Output R version info to downstream crates using variables `R_VERSION_MAJOR`,
 `R_VERSION_MINOR`, `R_VERSION_PATCH`, `R_VERSION_DEVEL`, and `R_VERSION_STRING`.

- Added precomputed bindings for Apple Silicon.

- Added contributing guidelines and code of conduct.

## libR-sys 0.2.0

- Provide precomputed bindings. Computations of bindings on the fly now only
  happens when the `use-bindgen` feature is enabled.

## libR-sys 0.1.10

- Minor fixes.

## libR-sys 0.1.9

- Remove need for pkg-config.
