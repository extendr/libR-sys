# Windows (R < 4.2) 

## Using precompiled bindings (recommended)

Two components are required to build the library:
1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.

Once `R` and `rust` are configured, the library can be easily built:
  
On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.
```Shell
cargo +stable-msvc build --target x86_64-pc-windows-gnu # 64-bit
cargo +stable-msvc build --target   i686-pc-windows-gnu # 32-bit
```

To test the build, run `cargo test`.

On Windows, the toolchain setup is a bit complex. Please refer to the "Toolchain setup on Windows" section below.

For 64-bit R,
```pwsh
cargo +stable-msvc test --target x86_64-pc-windows-gnu
```
For 32-bit R,
```pwsh
cargo +stable-msvc test --target i686-pc-windows-gnu
```

## Building bindings from source (advanced)

The bindings can be generated using [bindgen](https://github.com/rust-lang/rust-bindgen), special `rust` crate. 
`bindgen` usage is enabled via `use-bindgen` feature flag.

`bindgen` requires [libclang](https://clang.llvm.org/docs/Tooling.html), which should be installed first. 
This library relies on `LIBCLANG_PATH` environment variable to determine path to the appropriate version of `libclang`.

The output folder for bindings can be configured using `LIBRSYS_BINDINGS_OUTPUT_PATH` environment variable.

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

## Toolchain setup on Windows

### Install the `msvc` toolchain of Rust

When building for `Windows` with older versions of R, the `msvc` toolchain and
special `rust` targets should be added for compatibility with `R`:
```Shell
rustup toolchain install stable-msvc
rustup target add x86_64-pc-windows-gnu  # 64-bit
rustup target add   i686-pc-windows-gnu  # 32-bit
```

### Install Rtools40v2

Rtools40 can be downloaded from [here][rtools40]. Alternatively, `Rtools` can be
installed using `chocolatey`

[rtools40]: https://cran.r-project.org/bin/windows/Rtools/rtools40.html

```Shell
choco install rtools --version=4.0.0.20220206 -y
```

Verify that the environment variable `RTOOLS40_HOME` is set up to point to the
`Rtools` root.

### Setup `R_HOME` and  `PATH` envvars

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
