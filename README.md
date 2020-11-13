# libR-sys

Low-level R library bindings

[![Travis Build Status](https://api.travis-ci.org/extendr/libR-sys.svg?branch=master)](https://travis-ci.org/extendr/libR-sys)
[![Crates.io](http://meritbadge.herokuapp.com/libR-sys)](https://crates.io/crates/libR-sys)
[![License: Apache-2.0](https://img.shields.io/crates/l/rustr.svg)](#License)

[API Documentation](https://extendr.github.io/libR-sys/master/libR_sys/index.html)


# Installation

To build this library and test that everything is working as expected, run the following two commands in the library's top-level directory:

```
cargo build
cargo test
```

The build script has the following two dependencies:

1. [R.](https://cran.r-project.org/) It needs to be installed and available in the search path.
2. [libclang.](https://clang.llvm.org/docs/Tooling.html) Depending on your operating system, you may need to set the `LIBCLANG_PATH` environment variable or add `llvm-config` to your search path.

## Linux-specific instructions

Set `LIBCLANG_PATH` to the `lib` directory of your `llvm` installation. E.g., 
`LIBCLANG_PATH=/usr/lib/llvm-3.9/lib`.

## Windows-specific instructions

... coming soon ...

## Mac-specific instructions

Install `llvm-config` via [homebrew](https://brew.sh/) with:
```
brew install llvm
```

Add it to your search path via:
```
echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.bash_profile
```

If you want to compile `libR-sys` from within RStudio, you may also have to add the following line to your `.Renviron` file:
```
PATH=/usr/local/opt/llvm/bin:${PATH}
```