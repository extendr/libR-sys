# Windows (R < 4.2) 

## Using precompiled bindings (recommended)

Two components are required to build the library:

1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.

Once `R` and `rust` are configured, the library can be easily built:

On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.

```Shell
cargo build --target x86_64-pc-windows-gnu # 64-bit
cargo build --target   i686-pc-windows-gnu # 32-bit
```

To test the build, run `cargo test`.

On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.

For 64-bit R,

```pwsh
cargo test --target x86_64-pc-windows-gnu
```

For 32-bit R,

```pwsh
cargo test --target i686-pc-windows-gnu
```

## Building bindings from source (advanced)

The bindings can be generated using [bindgen](https://github.com/rust-lang/rust-bindgen), special `rust` crate. 
`bindgen` usage is enabled via `use-bindgen` feature flag.

`bindgen` requires [libclang](https://clang.llvm.org/docs/Tooling.html), which should be installed first. 
This library relies on `LIBCLANG_PATH` environment variable to determine path to the appropriate version of `libclang`.

The output folder for bindings can be configured using `LIBRSYS_BINDINGS_OUTPUT_PATH` environment variable.

On Windows, bindings can be generated using native `LLVM` installation and `Rtools` distribution.

Install LLVM:

```powershell
choco install llvm -y
```

`LLVM` can be also installed using `winget`, `scoop`, or manually.

To ensure LLVM is successfully installed and configured, run `clang --version`. If `clang` is not on the `PATH`, manually add path to `clang` installation to the `PATH` environement variable.

Install `Rtools` `Rtools40` can be downloaded from [here][rtools40]. Alternatively, `Rtools` can be
installed using `chocolatey`

[rtools40]: https://cran.r-project.org/bin/windows/Rtools/rtools40.html

```powershell
choco install rtools --version=4.0.0.20220206 -y
```

Verify that the environment variable `RTOOLS40_HOME` is set up to point to the
`Rtools` root.

Ensure that `R_HOME` environment variable is set to the `R` installation directory.

Finally, point `libR-sys` to the include directory of `Rtools`:

```powershell
$env:LIBRSYS_LIBCLANG_INCLUDE_PATH="$env:RTOOLS40_HOME\mingw64\x86_64-w64-mingw32\include"  # 64-bit
$env:LIBRSYS_LIBCLANG_INCLUDE_PATH="$env:RTOOLS40_HOME\mingw32\i686-w64-mingw32\include"  # 32-bit
```

Now, the bindings can be build using the following command:

```powershell
cargo build --target x86_64-pc-windows-gnu --features use-bindgen   # 64-bit
cargo build --target x86_64-pc-windows-gnu --features use-bindgen   # 32-bit
```

To run tests, update `PATH` variable:

```powershell
$env:PATH += ";$env:RTOOLS40_HOME\mingw64\bin"  # 64-bit
$env:PATH += ";$env:RTOOLS40_HOME\mingw32\bin"  # 32-bit
```

Then run tests:

```powershell
cargo test --target x86_64-pc-windows-gnu --features use-bindgen   # 64-bit
cargo test --target i686-pc-windows-gnu --features use-bindgen   # 32-bit
```

## Toolchain setup on Windows

### Install the `msvc` toolchain of Rust

When building for `Windows` with older versions of R, the `msvc` toolchain and
special `rust` targets should be added for compatibility with `R`:

```powershell
rustup toolchain install stable-msvc
rustup target add x86_64-pc-windows-gnu  # 64-bit
rustup target add   i686-pc-windows-gnu  # 32-bit
```
