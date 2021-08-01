# libR-sys

Low-level R library bindings

[![Github Actions Build Status](https://github.com/extendr/libR-sys/workflows/Tests/badge.svg)](https://github.com/extendr/libR-sys/actions)
[![crates.io](https://img.shields.io/crates/v/libR-sys.svg)](https://crates.io/crates/libR-sys)
[![Documentation](https://docs.rs/libR-sys/badge.svg)](https://docs.rs/libR-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

The recommended way to build this library is to use precomputed bindings, which are available for `Linux`, `macOS`, and `Windows` (`32`- and `64`-bit).

Alternatively, the library can be built from source, in which case it invokes `bindgen` crate, which has extra platform-specific dependencies (including `msys2` for `Windows`).


## Using precomputed bindings (recommended)

Two components are required to build the library:
1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.

Once `R` and `rust` are configured, the library can be easily built:
- **macOS/Linux**
  ```Shell
  cargo build
  ```

- **Windows**
  
  ```Shell
  cargo build --target x86_64-pc-windows-gnu # 64-bit
  cargo build --target   i686-pc-windows-gnu # 32-bit
  ```

  
  <details>
    <summary>If cargo is not configured</summary>

    When building for `Windows`, the default host should be `stable-msvc` and special `rust` targets should be added for compatibility with `R`:
    ```Shell
    rustup default stable-msvc
    rustup target add x86_64-pc-windows-gnu  # 64-bit
    rustup target add   i686-pc-windows-gnu  # 32-bit
    ```

    `stable-msvc` toolchain requires VS Build Tools. They are usually available on the systems with an installation of Visual Studio.
    Build tools can be obtained using an online [installer](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019) (see also [these examples](https://docs.microsoft.com/en-us/visualstudio/install/command-line-parameter-examples?view=vs-2019)) or using `chocolatey`.
    Required workflow components are:
    - Microsoft.VisualStudio.Component.VC.CoreBuildTools 
    - Microsoft.VisualStudio.Component.VC.Tools.x86.x64 
    - Microsoft.VisualStudio.Component.Windows10SDK.19041 (the latest version of the SDK available at the moment of writing this readme)

    If there is an installation of VS (or Build Tools) on the system, launch `Visual Studio Installer` and ensure that either three required workflows are installed as individual components, or the whole `Desktop Development with C++` workflow pack is installed.

    If neither VS Build Tools nor Visual Studio itself are installed, all the necessary workflows can be easily obtained with the help of `chocolatey`:
    ```Shell
    choco install visualstudio2019buildtools -y 
    choco install visualstudio2019-workload-vctools -y -f --package-parameters "--no-includeRecommended --add Microsoft.VisualStudio.Component.VC.CoreBuildTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.19041"  
    ```
  </details>
 





To test the build, run `cargo test`.


- **macOS/Linux**
  ```bash
  cargo test
  ```
- **Windows**
  
  On Windows, both `R` and `Rtools` should be available on `PATH` for tests to succeed. Ensure that `R_HOME` points to `R` home, e.g. `C:\Program Files\R\R-4.1.0`. 
  For `x64`, append the following to the `PATH` (using `PowerShell` syntax):
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\x64;$env:RTOOLS40_HOME\mingw64\bin"
  ```
  then test with 
  ```pwsh
  cargo test --target x86_64-pc-windows-gnu
  ```

  For `x86`, 
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\i386;$env:RTOOLS40_HOME\mingw32\bin"
  ```
  and then test with 
  ```pwsh
  cargo test --target i686-pc-windows-gnu
  ```
  <details>
    <summary>If Rtools40v2 is missing</summary>

    Rtools can be downloaded from [here](https://cran.r-project.org/bin/windows/Rtools/). Alternatively, `Rtools` can be installed using `chocolatey`
    
    ```Shell
    choco install rtools -y
    ```

    Verify that the environment variable `RTOOLS40_HOME` is set up to point to the `Rtools` root.
  </details>

## Building bindings from source (advanced)

The bindings can be generated using [bindgen](https://github.com/rust-lang/rust-bindgen), special `rust` crate. 
`bindgen` usage is enabled via `use-bindgen` feature flag.

`bindgen` requires [libclang](https://clang.llvm.org/docs/Tooling.html), which should be installed first. 
This library relies on `LIBCLANG_PATH` environment variable to determine path to the appropriate version of `libclang`.

The output folder for bindings can be configured using `LIBRSYS_BINDINGS_OUTPUT_PATH` environment variable.

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
  PATH=/usr/local/opt/llvm/bin:${PATH}
  ```
  Build & test using
   ```shell
  cargo build --features use-bindgen
  cargo test  --features use-bindgen 
  ```
- **Windows**
  
  Binding generation on Windows happens with the help of `MSYS2`.
  Make sure the environment variable `MSYS_ROOT` points to `MSYS2` root, e.g., `C:\tools\msys64`.

  <details>
    <summary>Installing and configuring MSYS2</summary>

    Install `MSYS2`. Here is an example using  `chocolatey`:
    ```Shell
    choco install msys2 -y
    ```
    Set up `MSYS_ROOT` environment variable.
    Install `clang` and `mingw`-toolchains (assuming `PowerShell` syntax)

    ```pwsh
    &"$env:MSYS_ROOT\usr\bin\bash" -l -c "pacman -S --noconfirm mingw-w64-x86_64-clang mingw-w64-x86_64-toolchain"      # 64-bit
    &"$env:MSYS_ROOT\usr\bin\bash" -l -c "pacman -S --noconfirm mingw32/mingw-w64-i686-clang mingw-w64-i686-toolchain"  # 32-bit
    ```
    
  </details>

  For `x64`, append the following to the `PATH` (using `PowerShell` syntax):
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\x64;$env:MSYS_ROOT\mingw64\bin"
  ```
  then build & test with 
  ```pwsh
  cargo build --target x86_64-pc-windows-gnu --features use-bindgen
  cargo  test --target x86_64-pc-windows-gnu --features use-bindgen
  ```

  For `x86`, 
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\i386;$env:MSYS_ROOT\mingw64\bin$env:MSYS_ROOT\mingw32\bin"
  ```
  and then build & test with 
  ```pwsh
  cargo build --target i686-pc-windows-gnu --features use-bindgen
  cargo  test --target i686-pc-windows-gnu --features use-bindgen
  ```

  <details>
  <summary>Generating x86 bindings using 32-bit process (optional)</summary>

  Add 32-bit `Rust` toolchain and configure target:

  ```pwsh
  rustup toolchain install stable-i686-pc-windows-msvc
  rustup target add i686-pc-windows-gnu --toolchain stable-i686-pc-windows-msvc
  ```
  Configure environment variables:
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\i386;$env:MSYS_ROOT\mingw32\bin"
  ```

  Build & test using specific toolchain
  ```pwsh
  cargo +stable-i686-pc-windows-msvc build --target i686-pc-windows-gnu --features use-bindgen
  cargo +stable-i686-pc-windows-msvc  test --target i686-pc-windows-gnu --features use-bindgen
  ```
  </details>

## Conditional compilation depending on R installation

extendr crate provides these environmental variables that you can use in `build.rs`:

- `DEP_R_R_VERSION_MAJOR`: The major part of the R version (e.g. `4` in version `4.1.0`)
- `DEP_R_R_VERSION_MINOR`: The minor part of the R version (e.g. `1` in version `4.1.0`)
- `DEP_R_R_VERSION_PATCH`: The patch part of the R version (e.g. `0` in version `4.1.0`)
- `DEP_R_R_VERSION_DEVEL`: `true` if the R is a development version, otherwise `false`
- `DEP_R_R_VERSION_STRING`: The full version string (e.g. `R version 4.1.0 (2021-05-18)`)
- `DEP_R_R_HOME`: The R home directory

### Example `build.rs`

``` rust
use std::env;

fn main() {
    // Set R_HOME envvar, and refer to it on compile time by env!("R_HOME")
    let r_home = env::var("DEP_R_R_HOME").unwrap();
    println!("cargo:rustc-env=R_HOME={}", r_home);

    // Enable cfg setting to conditionally compile a code using a feature
    // available only on newer versions of R
    let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();
    if &*major >= "4" && &*minor >= "1" {
        println!("cargo:rustc-cfg=use_a_feature");
    }
}
```
