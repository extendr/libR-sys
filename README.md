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
1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.

Once `R` and `rust` are configured, the library can be easily built:
- **macOS/Linux**
  ```Shell
  cargo build
  ```

- **Windows (R >= 4.2)**
  
  On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.
  ```Shell
  cargo +stable-gnu build
  ```

- **Windows (R < 4.2)**
  
  <details>
  On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.
  ```Shell
  cargo +stable-msvc build --target x86_64-pc-windows-gnu # 64-bit
  cargo +stable-msvc build --target   i686-pc-windows-gnu # 32-bit
  ```
  </details>
 





To test the build, run `cargo test`.


- **macOS/Linux**
  ```bash
  cargo test
  ```

- **Windows (R >= 4.2)**
  
  On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.
  ```pwsh
  cargo +stable-gnu test
  ```

- **Windows (R < 4.2)**
  
  <details>
  On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.

  For 64-bit R,
  ```pwsh
  cargo +stable-msvc test --target x86_64-pc-windows-gnu
  ```
  For 32-bit R,
  ```pwsh
  cargo +stable-msvc test --target i686-pc-windows-gnu
  ```
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
  PATH=/usr/local/opt/llvm/bin:$PATH
  ```
  Build & test using
   ```shell
  cargo build --features use-bindgen
  cargo test  --features use-bindgen 
  ```
- **Windows (R >= 4.2)**
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
    &"$env:MSYS_ROOT\usr\bin\bash" -l -c "pacman -S --noconfirm mingw-w64-x86_64-clang mingw-w64-x86_64-toolchain"
    ```
    
  </details>

  Add the following to the `PATH` (using `PowerShell` syntax). 
  ```pwsh
  $env:PATH = "${env:R_HOME}\bin\x64;C:\rtools42\usr\bin;C:\rtools42\x86_64-w64-mingw32.static.posix\bin:${env:MSYS_ROOT}\mingw64\bin"
  ```
  then build & test with 
  ```pwsh
  cargo +stable-gnu build --features use-bindgen
  ```
  
- **Windows (R < 4.2)**
  <details>
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

  For 64-bit R, add the following to the `PATH` (using `PowerShell` syntax):
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\x64;$env:MSYS_ROOT\mingw64\bin"
  ```
  then build & test with 
  ```pwsh
  cargo +stable-msvc build --target x86_64-pc-windows-gnu --features use-bindgen
  cargo +stable-msvc  test --target x86_64-pc-windows-gnu --features use-bindgen
  ```

  For 32-bit R, 
  ```pwsh
  $env:PATH += ";$env:R_HOME\bin\i386;$env:MSYS_ROOT\mingw64\bin$env:MSYS_ROOT\mingw32\bin"
  ```
  and then build & test with 
  ```pwsh
  cargo +stable-msvc build --target i686-pc-windows-gnu --features use-bindgen
  cargo +stable-msvc  test --target i686-pc-windows-gnu --features use-bindgen
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
  </details>

## Toolchain setup on Windows

### Windows (R >= 4.2)

When building for Windows with R >= 4.2, the GNU toolchain is required. The
setup is tricky because the Rtools' toolchain is a bit different from the
assumption of Rust.

#### Install the `gnu` toolchain of Rust

```Shell
rustup toolchain install stable-gnu
```

#### Install Rtools42

Rtools42 can be downloaded from [here][rtools_website]. Alternatively, `Rtools`
will eventually be available on `chocolatey`.

[rtools_website]: https://cran.r-project.org/bin/windows/Rtools/rtools42/rtools.html

```Shell
## TODO: Rtools42 is not yet on chocolatey
# choco install rtools -y
```

#### Setup `R_HOME` and  `PATH` envvars

First, ensure that `R_HOME` points to `R` home, e.g. `C:\Program Files\R\R-4.2.0`
(in an R session, this should be automatically set by R).

Second, ensure that `PATH` is properly configured that the following executables
are available:

* the `R` binary to build against
* the compiler toolchain that is used for compiling the R itself, i.e., `Rtools`

Typically, the following paths need to be added to the head of `PATH` (using
`PowerShell` syntax).

```pwsh
$env:PATH = "${env:R_HOME}\bin\x64;C:\rtools42\usr\bin;C:\rtools42\x86_64-w64-mingw32.static.posix\bin;${env:PATH}"
```

Note that the above prepends, rather than appends, because otherwise the wrong
toolchain might be accidentally chosen if the `PATH` already contains another
version of `R` or compiler toolchain.

#### Tweak the toolchain

As noted above, since the Rtools' toolchain is a bit different from the
assumption of Rust, we need the following tweaks:

1. Change the linker name to `x86_64-w64-mingw32.static.posix-gcc.exe`.
2. Add empty `libgcc_s.a` and `libgcc_eh.a`, and add them to the compiler's
   library search paths via `LIBRARY_PATH` envvar.

The first tweak is needed because Rtools42 doesn't contain
`x86_64-w64-mingw32-gcc`, which `rustc` uses as the default linker for the
`x86_64-pc-windows-gnu` target. This can be done by adding `.cargo/config.toml`
with the following lines on the root directory of the project:

``` toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32.static.posix-gcc.exe"
```

Alternatively, you can inject this configuration via the corresponding
environmental variable, `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER`. See [the
Cargo Book] about how this works.

[The Cargo Book]: https://doc.rust-lang.org/cargo/reference/config.html#environment-variables

The second tweak is also required. `rustc` adds `-lgcc_eh` and `-lgcc_s` flags
to the compiler, but Rtools' GCC doesn't have `libgcc_eh` or `libgcc_a` due to
the compilation settings. So, in order to please the compiler, we need to add
empty `libgcc_eh` or `libgcc_a` to the library search paths. For more details,
please refer to [r-windows/rtools-packages].

[r-windows/rtools-packages]: https://github.com/r-windows/rtools-packages/blob/2407b23f1e0925bbb20a4162c963600105236318/mingw-w64-gcc/PKGBUILD#L313-L316

First, create a directory that contains empty `libgcc_eh` or `libgcc_a`.

``` ps1
# create a directory in an arbitrary location (e.g. libgcc_mock)
New-Item -Path libgcc_mock -Type Directory

# compile an empty C file
New-Item -Path libgcc_mock\gcc.c -Type File
x86_64-w64-mingw32.static.posix-gcc.exe -c libgcc_mock\gcc.c -o libgcc_mock\gcc.o

# create empty libgcc_eh.a and libgcc_s.a
x86_64-w64-mingw32.static.posix-ar.exe -r libgcc_mock\libgcc_eh.a libgcc_mock\gcc.o
x86_64-w64-mingw32.static.posix-ar.exe -r libgcc_mock\libgcc_s.a libgcc_mock\gcc.o
```

Then, add the directory to `LIBRARY_PATH` envvar. For example, this can be done
by adding the following lines to `.cargo/config.toml`:

``` toml
[env]
LIBRARY_PATH = "path/to/libgcc_mock"
```

### Windows (R < 4.2)

#### Install the `msvc` toolchain of Rust

When building for `Windows` with older versions of R, the `msvc` toolchain and
special `rust` targets should be added for compatibility with `R`:
```Shell
rustup toolchain install stable-msvc
rustup target add x86_64-pc-windows-gnu  # 64-bit
rustup target add   i686-pc-windows-gnu  # 32-bit
```

`stable-msvc` toolchain requires VS Build Tools. They are usually available on
the systems with an installation of Visual Studio. Build tools can be obtained
using an online [installer] (see also [these examples]) or using `chocolatey`.
Required workflow components are:
- Microsoft.VisualStudio.Component.VC.CoreBuildTools 
- Microsoft.VisualStudio.Component.VC.Tools.x86.x64 
- Microsoft.VisualStudio.Component.Windows10SDK.19041 (the latest version of the SDK available at the moment of writing this readme)

[installer]: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019
[these examples]: https://docs.microsoft.com/en-us/visualstudio/install/command-line-parameter-examples?view=vs-2019

If there is an installation of VS (or Build Tools) on the system, launch `Visual
Studio Installer` and ensure that either three required workflows are installed
as individual components, or the whole `Desktop Development with C++` workflow
pack is installed.

If neither VS Build Tools nor Visual Studio itself are installed, all the
necessary workflows can be easily obtained with the help of `chocolatey`:
```Shell
choco install visualstudio2019buildtools -y 
choco install visualstudio2019-workload-vctools -y -f --package-parameters "--no-includeRecommended --add Microsoft.VisualStudio.Component.VC.CoreBuildTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.19041"  
```

#### Install Rtools40v2

Rtools40 can be downloaded from [here][rtools40]. Alternatively, `Rtools` can be
installed using `chocolatey`

[rtools40]: https://cran.r-project.org/bin/windows/Rtools/rtools40.html

```Shell
choco install rtools --version=4.0.0.20220206 -y
```

Verify that the environment variable `RTOOLS40_HOME` is set up to point to the
`Rtools` root.

#### Setup `R_HOME` and  `PATH` envvars

First, ensure that `R_HOME` points to `R` home, e.g. `C:\Program Files\R\R-4.1.0`
(in an R session, this should be set by R).

Second, ensure that `PATH` is properly configured that the following executables
are available:

* the `R` binary to build against
* the compiler toolchain that is used for compiling the R itself, i.e., `Rtools`

Typically, they can be found in the following locations (using `PowerShell` syntax):

|         | R                         | Rtools                             |
| ------- | ------------------------- | ---------------------------------- |
| 64-bit  |  `$env:R_HOME\bin\x64`   | `$env:RTOOLS40_HOME\mingw64\bin` |
| 32-bit  |  `$env:R_HOME\bin\i386`  | `$env:RTOOLS40_HOME\mingw32\bin` |


Typically, the following paths need to be added to the head of `PATH` (using
`PowerShell` syntax) for 64-bit R.

```pwsh
$env:PATH = "${env:R_HOME}\bin\x64;${env:RTOOLS40_HOME}\mingw64\bin;${env:PATH}"
```

and for 32-bit R.

```pwsh
$env:PATH = "${env:R_HOME}\bin\i386;${env:RTOOLS40_HOME}\mingw32\bin;${env:PATH}"
```

Note that the above prepends, rather than appends, because otherwise the wrong
toolchain might be accidentally chosen if the `PATH` already contains another
version of `R` or compiler toolchain.
