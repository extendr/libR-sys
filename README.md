# libR-sys

Low-level R library bindings

[![Github Actions Build Status](https://github.com/extendr/libR-sys/workflows/Tests/badge.svg)](https://github.com/extendr/libR-sys/actions)
[![crates.io](https://img.shields.io/crates/v/libR-sys.svg)](https://crates.io/crates/libR-sys)
[![Documentation](https://docs.rs/libR-sys/badge.svg)](https://docs.rs/libR-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

The recommended way to build this library is to use precompiled bindings, which are available for `Linux`, `macOS`, and `Windows`.

Alternatively, the library can be built from source, in which case it invokes `bindgen` crate, which has extra platform-specific dependencies (including `msys2` for `Windows`).

## Configuration

`libR-sys` recognizes the following environment variables:

- `LIBRSYS_R_VERSION` If set, it is used to determine the version of R, for which bindings should be generated. `LIBRSYS_R_VERSION` should be set to one of the supported values, e.g. `4.2.0` or `4.3.0-devel` (the pattern is `major.minor.patch[-devel]`). Malformed `LIBRSYS_R_VERSION` results in compilation error. If `LIBRSYS_R_VERSION` is unset, `R` is invoked and its `R.version` is used.

## Using precompiled bindings (recommended)

Two components are required to build the library:

1. [`R`](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [`Rust`](https://www.rust-lang.org/learn/get-started): It is recommended to install `Rust` using `rustup`; search path should include `Rust` binaries.

**Note: On Windows, only R >= 4.2 is supported**

Once `R` and `Rust` are configured, the library can be easily built:

```bash
# macOS & Linux
cargo build

# Windows
cargo build --target x86_64-pc-windows-gnu
```

To test the build, run `cargo test`.

```bash
# macOS & Linux
cargo test

# Windows
cargo test --target x86_64-pc-windows-gnu
```

## Building bindings from source (advanced)

**Note: On Windows, only R >= 4.2 is supported**

The bindings can be generated using [`bindgen`](https://github.com/rust-lang/rust-bindgen), special `Rust` crate.
`bindgen` usage is enabled via `use-bindgen` feature flag.

`bindgen` requires [`libclang`](https://clang.llvm.org/docs/Tooling.html), which should be installed first.
This library relies on `LIBCLANG_PATH` environment variable to determine path to the appropriate version of `libclang`.

The output folder for bindings can be configured using `LIBRSYS_BINDINGS_OUTPUT_PATH` environment variable, thus make sure it is set to e.g `bindings`.

- **Linux**

  Set `LIBCLANG_PATH` to the `lib` directory of your `llvm` installation, e.g.,
  `LIBCLANG_PATH=/usr/lib/llvm-3.9/lib`. Build & test using

  ```shell
  cargo build --features use-bindgen
  cargo test  --features use-bindgen 
  ```

- **macOS**

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
  PATH=/usr/local/opt/llvm/bin:$PATH
  ```

  Build & test using
  
  ```shell
  cargo build --features use-bindgen
  cargo test  --features use-bindgen 
  ```

- **Windows**
  On Windows, bindings can be generated using native `LLVM` installation and `Rtools` distribution.

  Install LLVM:

  ```powershell
  choco install llvm -y
  ```

  `LLVM` can be also installed using `winget`, `scoop`, or manually.

  To ensure LLVM is successfully installed and configured, run `clang --version`. If `clang` is not on the `PATH`, manually add path to `clang` installation to the `PATH` environement variable.

  Install `Rtools` if it is misisng:

  ```powershell
  choco install rtools -y
  ```

  Installing `Rtools` this way automatically sets `RTOOLS42_HOME` (or `RTOOLS43_HOME`) environment variable.

  Ensure that `R_HOME` environment variable is set to the `R` installation directory.

  Finally, point `libR-sys` to the include directory of `Rtools`:

  ```powershell
  $env:LIBRSYS_LIBCLANG_INCLUDE_PATH="$env:RTOOLS42_HOME\x86_64-w64-mingw32.static.posix\include"
  ```

  Now, the bindings can be build using the following command:

  ```powershell
  cargo build --target x86_64-pc-windows-gnu --features use-bindgen
  ```

## Running bindgen tests on Windows

Running bindgen tests on Windows requires a bit more setup.

First, add `Rtools` `bin` directory to the `PATH` (replace `RTOOLS42_HOME` with `RTOOLS43_HOME` if you are using `Rtools` 4.3):

```powershell
$env:PATH += ";$env:RTOOLS42_HOME\x86_64-w64-mingw32.static.posix\bin"
```

Second, patch `Rtools` version `4.2` and higher since there is a `gcc` static linking issue. `rustc` adds `-lgcc_eh` flag
to the compiler, but Rtools' GCC doesn't have `libgcc_eh` due to
the compilation settings. So, in order to please the compiler, `libgcc_eh` should be mocked and added to the library search paths. For more details, please refer to [r-windows/rtools-packages].

[r-windows/rtools-packages]: https://github.com/r-windows/rtools-packages/blob/2407b23f1e0925bbb20a4162c963600105236318/mingw-w64-gcc/PKGBUILD#L313-L316

Create a directory for missing library file and an empty file there:

``` powershell
# create a directory in an arbitrary location (e.g. libgcc_mock)
New-Item -Path libgcc_mock -Type Directory

# create empty libgcc_eh.a and libgcc_s.a
New-Item -Path libgcc_mock\libgcc_eh.a -Type File
```

Finally, configure `Rust` compiler and select appropriate linker (see [The Cargo Book]):

[The Cargo Book]: https://doc.rust-lang.org/cargo/reference/config.html#environment-variables

```powershell
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="x86_64-w64-mingw32.static.posix-gcc.exe"
$env:LIBRARY_PATH="libgcc_mock" # Replace it with the path to the directory created above
```

Alternatively, linker can be configured in `.cargo/config.toml`:

``` toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32.static.posix-gcc.exe"
```

Now, the tests can be run:

```powershell
cargo test --target x86_64-pc-windows-gnu --features use-bindgen
```

### Editor settings

<details>

Rust-analyzer might need some settings. For example, if you are using VS Code, you probably need to add the following options to `.vscode/settings.json`.

``` json
{
    // The target needs to be GNU
    "rust-analyzer.cargo.target": "x86_64-pc-windows-gnu",
    // Specify "use-bindgen" for developing R-devel.
    "rust-analyzer.cargo.features": [],
    "terminal.integrated.env.windows": {
        "R_HOME": "C:/Program Files/R/R-4.2.2",
        "PATH": "${env:R_HOME}/bin/x64;C:/rtools42/x86_64-w64-mingw32.static.posix/bin;C:/rtools42/usr/bin;${env:PATH}"
    }
}
```

</details>
