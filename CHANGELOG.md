# Changelog

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
