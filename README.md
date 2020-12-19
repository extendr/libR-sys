# libR-sys

Low-level R library bindings

[![Github Actions Build Status](https://github.com/extendr/libR-sys/workflows/Tests/badge.svg)](https://github.com/extendr/libR-sys/actions)
[![crates.io](http://meritbadge.herokuapp.com/libR-sys)](https://crates.io/crates/libR-sys)
[![Documentation](https://docs.rs/libR-sys/badge.svg)](https://docs.rs/libR-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

The recommended way to build this library is to use precomputed bindings, which are available for `Linux`, `MacOS`, and `Windows` (`32`- and `64`-bit).

Alternatively, the library can be built from source, in which case it invokes `bindgen` crate, which has extra platform-specific dependencies (including `msys2` on `Windows`).


## Using precomputed bindings (recommended)

Two components are required to build the library:
1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.


When building for `Windows`, special `rust` targets should be added for compatibility with `R`:
- **Windows**
  ```Shell
  rustup target add x86_64-pc-windows-gnu  # 64-bit
  rustup target add i686-pc-windows-gnu    # 32-bit
  ```

Once `R` and `rust` are configured, the library can be easily build:
- **MacOS/Linux**
    ```bash
    cargo build
    ```
- **Windows**
    ```Shell
    cargo build --target x86_64-pc-windows-gnu # 64-bit
    cargo build --target i686-pc-windows-gnu   # 32-bit
    ```


To test the build, run `cargo test`. Note the `--test-threads=1`, without this flag `R` integration tests will fail:

- **MacOs/Linux**
    ```bash
    cargo test -- --nocapture --test-threads=1
    ```
- **Windows**
    **NOTE:** currently `Windows` tests are unstable, but still can be executed with additional effort:
    <details>
    <summary>Running tests on Windows</summary>

    First, locate the installation of `R` and ensure that environment variable `R_HOME` points to it.
    The standard value for the latest `R` version is `C:\Program Files\R\R-4.0.3`.

    In order to run tests, `PATH` variable should be temporarily prepended with the path to correct `R.dll`.

    - **64-bit**
      - **CMD**
        ```Shell
        set OLD_PATH=%PATH%                        # Captures current PATH
        set PATH=%R_HOME%\bin\x64;%PATH%           # Prepends with correct R directory
        cargo test --target x86_64-pc-windows-gnu -- --nocapture --test-threads=1
        set PATH=%OLD_PATH%                        # Resets PATH to the original value
        ```
      - **PowerShell**
        ```PowerShell
        $OLD_PATH=$env:PATH                        # Captures current PATH
        $env:PATH="$env:R_HOME\bin\x64;$env:PATH"  # Prepends with correct R directory
        cargo test --target x86_64-pc-windows-gnu -- --nocapture --test-threads=1
        $env:PATH=$OLD_PATH                        # Resets PATH to the original value
        ```
    - **32-bit**
      - **CMD**
        ```Shell
        set OLD_PATH=%PATH%                        # Captures current PATH
        set PATH=%R_HOME%\bin\i386;%PATH%          # Prepends with correct R directory
        cargo test --target i686-pc-windows-gnu -- --nocapture --test-threads=1
        set PATH=%OLD_PATH%                        # Resets PATH to the original value
        ```
      - **PowerShell**
        ```PowerShell
        $OLD_PATH=$env:PATH                        # Captures current PATH
        $env:PATH="$env:R_HOME\bin\i386;$env:PATH" # Prepends with correct R directory
        cargo test --target i686-pc-windows-gnu -- --nocapture --test-threads=1
        $env:PATH=$OLD_PATH                        # Resets PATH to the original value
        ```


    </details>



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

Ensure the preferred R binaries, are part of the `PATH`, e.g. `C:\R\R-4.0.3\bin\x64`.
For information on how to add environment variables on Windows, [see here](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.1#saving-changes-to-environment-variables).

Ensure that `rust` `msvc` toolchains are available:
```Shell
rustup toolchain add stable-x86_64-pc-windows-msvc
rustup toolchain add stable-i686-pc-windows-msvc
```

Add the `mingw` targets that are used to build R:

```Shell
rustup target add x86_64-pc-windows-gnu --toolchain stable-x86_64-pc-windows-msvc
rustup target add i686-pc-windows-gnu --toolchain stable-i686-pc-windows-msvc
```
Install `MSYS2`. 
Using `scoop` it is done by

```Shell
scoop install msys2
```

Alternatively, `chocolatey` can be used (from elevated prompt):
```Shell
choco install msys2
```

To complete the installation, run `msys2` once, then restart the terminal.

Run `msys2` again, and install `Clang` and `mingw`-toolchain via

```Shell
pacman -S --noconfirm mingw-w64-x86_64-clang mingw-w64-x86_64-toolchain
pacman -S --noconfirm mingw32/mingw-w64-i686-clang mingw-w64-i686-toolchain
```

If not, or if it is missing, set `R_HOME` to default `R` location (e.g. `C:\Program Files\R\R-4.0.3`).

- **CMD**
    ```Shell
    set R_HOME="C:\Program Files\R\R-4.0.3"
    ```
- **PowerShell**
    ```PowerShell
    $env:R_HOME="C:\Program Files\R\R-4.0.3"
    ```



Set `MSYS_ROOT` to the root of your `msys2` installation.
If `scoop` was used then the path would be:
- **CMD**
    ```Shell
    set MSYS_ROOT="%USERPROFILE%\scoop\apps\msys2\current"
    ```
- **PowerShell**
    ```PowerShell
    $env:MSYS_ROOT="$env:USERPROFILE\scoop\apps\msys2\current"
    ```
If `chocolatey` was used then the path would be:
- **CMD**
    ```Shell
    set MSYS_ROOT="C:\tools\msys64"
    ```
- **PowerShell**
    ```PowerShell
    $env:MSYS_ROOT="C:\tools\msys64"
    ```

Ensure `PATH` variable contains `%MSYS_ROOT%\usr\bin`, `%MSYS_ROOT%\mingw32\bin`, `%MSYS_ROOT%\mingw64\bin`.
For example,
- **CMD**
  ```Shell
  set PATH=%MSYS_ROOT%\usr\bin;%MSYS_ROOT%\mingw32\bin;%MSYS_ROOT%\mingw64\bin;%PATH%
  ```
- **PowerShell**
  ```PowerShell
  $env:PATH="$env:MSYS_ROOT\usr\bin;$env:MSYS_ROOT\mingw32\bin;$env:MSYS_ROOT\mingw64\bin;$env:PATH"
  ```

Add environment variable `LIBCLANG_PATH` with the value pointing to where the
clang binaries are placed. This depends on whether the target architecture is `32` or `64` bit.

To build `64`-bit library, run
- **CMD**
    ```Shell
    set LIBCLANG_PATH=%MSYS_ROOT%\mingw64\bin 
    ```
- **PowerShell**
  ```PowerShell
  $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw64\bin"
  ```
and then
```Shell
cargo +stable-x86_64-pc-windows-msvc build --target=x86_64-pc-windows-gnu
```

To build `32-bit`, run
- **CMD**
    ```Shell
    set LIBCLANG_PATH=%MSYS_ROOT%\mingw32\bin 
    ```
- **PowerShell**
  ```PowerShell
  $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw32\bin"
  ```
and then
```Shell
cargo +stable-i686-pc-windows-msvc build --target=i686-pc-windows-gnu
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
