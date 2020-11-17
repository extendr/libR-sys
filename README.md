# libR-sys

Low-level R library bindings

[![Travis Build Status](https://api.travis-ci.org/extendr/libR-sys.svg?branch=master)](https://travis-ci.org/extendr/libR-sys)
[![Crates.io](http://meritbadge.herokuapp.com/libR-sys)](https://crates.io/crates/libR-sys)
[![License: Apache-2.0](https://img.shields.io/crates/l/rustr.svg)](#License)

[API Documentation](https://extendr.github.io/libR-sys/master/libR_sys/index.html)

## Installation

To build this library and test that everything is working as expected, run the following two commands in the library's top-level directory:

```bash
cargo build
cargo test
```

The build script has the following two dependencies:

1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [libclang](https://clang.llvm.org/docs/Tooling.html): Depending on your operating system, you may need to set the `LIBCLANG_PATH` environment variable or add `llvm-config` to your search path.

### Linux-specific instructions

Set `LIBCLANG_PATH` to the `lib` directory of your `llvm` installation. E.g.,
`LIBCLANG_PATH=/usr/lib/llvm-3.9/lib`.

### Windows-specific instructions

Ensure the preferred R binaries, are part of the `PATH`, e.g. `C:/R/R-4.0.2/bin/x64`.
For information on how to add environment variables on Windows, [see here](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.1#saving-changes-to-environment-variables).

Add the mingw toolchains that are used to build R:

```bash
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
```

The default toolchain must be `(stable|beta|nightly)-gnu`, e.g. to set the default
toolchain to be nightly mingw:

```bash
rustup default nightly-gnu
```

Install `MSYS2`. Using scoop it is done by

```bash
scoop install msys2
```

To complete the installation, run `msys2` once, then restart the terminal.

Run `msys2` again, and install Clang and mingw-toolchain via

```bash
pacman -S --noconfirm mingw-w64-x86_64-clang mingw-w64-x86_64-toolchain
pacman -S --noconfirm mingw32/mingw-w64-i686-clang mingw-w64-i686-toolchain
```

Add environment variable `LIBCLANG_PATH` with the value pointing to where the
clang binaries are placed. If scoop was used then the path would be:
`%USERPROFILE%\scoop\apps\msys2\current\mingw64\bin`.

Then from now on, in order to build this, use:

```bash
mingw64
cargo build
cargo test
```

In order to build it without having to enter a `mingw64`/`mingw32` shell, then add these
paths to `PATH`:

```bash
%USERPROFILE%\scoop\apps\msys2\current\usr\bin
%USERPROFILE%\scoop\apps\msys2\current\mingw64\bin # change this to mingw32
```

### Mac-specific instructions

Install `llvm-config` via [homebrew](https://brew.sh/) with:

```bash
brew install llvm
```

Add it to your search path via:

```bash
echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.bash_profile
```

If you want to compile `libR-sys` from within RStudio, you may also have to add the following line to your `.Renviron` file:

```bash
PATH=/usr/local/opt/llvm/bin:${PATH}
```
