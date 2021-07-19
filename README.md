# libR-sys

Low-level R library bindings

[![Github Actions Build Status](https://github.com/extendr/libR-sys/workflows/Tests/badge.svg)](https://github.com/extendr/libR-sys/actions)
[![crates.io](http://meritbadge.herokuapp.com/libR-sys)](https://crates.io/crates/libR-sys)
[![Documentation](https://docs.rs/libR-sys/badge.svg)](https://docs.rs/libR-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

The recommended way to build this library is to use precomputed bindings, which are available for `Linux`, `MacOS`, and `Windows` (`32`- and `64`-bit).

Alternatively, the library can be built from source, in which case it invokes `bindgen` crate, which has extra platform-specific dependencies (including `msys2` for `Windows`).


## Using precomputed bindings (recommended)

Two components are required to build the library:
1. [R](https://cran.r-project.org/): It needs to be installed and available in the search path.
2. [rust](https://www.rust-lang.org/learn/get-started): It is recommended to install `rust` using `rustup`; search path should include `rust` binaries.


### **Windows**
On Windows, install [`Rtools40v2`](https://cran.r-project.org/bin/windows/Rtools/).
Alternatively, both `R` and `Rtools` can be installed using `chocolatey`

```Shell
choco install R rtools -y
```
Verify that the environment variable `RTOOLS40_HOME` is set up to point to the `Rtools` root.
Also, ensure that `R_HOME` points to `R` home, e.g. `C:\Program Files\R\R-4.1.0`. 


When building for `Windows`, the default host should be `stable-msvc` and special `rust` targets should be added for compatibility with `R`:
```Shell
rustup default stable-msvc
rustup target add x86_64-pc-windows-gnu  # 64-bit
rustup target add i686-pc-windows-gnu    # 32-bit
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

Once `R` and `rust` are configured, the library can be easily built:

### **MacOS/Linux**

```Shell
cargo build
```
### **Windows**
```Shell
cargo build --target x86_64-pc-windows-gnu # 64-bit
cargo build --target i686-pc-windows-gnu   # 32-bit
```


To test the build, run `cargo test`. Note the `--test-threads=1`, without this flag `R` integration tests will fail:

### **MacOs/Linux**
```bash
cargo test -- --nocapture --test-threads=1
```
### **Windows**
On Windows, 


## Building bindings from source (advanced)

The bindings can be generated using [bindgen](https://github.com/rust-lang/rust-bindgen), special `rust` crate. 
`bindgen` usage is enabled via `use-bindgen` feature flag.

`bindgen` requires [libclang](https://clang.llvm.org/docs/Tooling.html), which should be installed first. 
This library relies on `LIBCLANG_PATH` environment variable to determine path to the appropriate version of `libclang`.

The output folder for bindings can be configured using `LIBRSYS_BINDINGS_OUTPUT_PATH` environment variable.
### Linux-specific instructions

Set `LIBCLANG_PATH` to the `lib` directory of your `llvm` installation. E.g.,
`LIBCLANG_PATH=/usr/lib/llvm-3.9/lib`.

### Windows-specific instructions

Ensure the preferred R binaries are part of the `PATH`, e.g. `C:\R\R-4.0.3\bin\x64`.
For information on how to add environment variables on Windows, [see here](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.1#saving-changes-to-environment-variables).

Ensure that `rust` `msvc` toolchains are available:
```Shell
rustup toolchain add stable-x86_64-pc-windows-msvc     # 64-bit
rustup toolchain add stable-i686-pc-windows-msvc       # 32-bit
```

Add the `mingw` targets that are used to build R:

```Shell
rustup target add x86_64-pc-windows-gnu --toolchain stable-x86_64-pc-windows-msvc # 64-bit
rustup target add i686-pc-windows-gnu --toolchain stable-i686-pc-windows-msvc     # 32-bit
```
Install `MSYS2` using e.g. `scoop` or `chocolatey` (or any other method):
- **scoop**
  ```Shell
  scoop install msys2
  ```
- **chocolatey**
  ```Shell
  choco install msys2
  ```

To complete the installation, run `msys2` once, then restart the terminal.

Run `msys2` again, and install `Clang` and `mingw`-toolchain via

```Shell
pacman -S --noconfirm mingw-w64-x86_64-clang mingw-w64-x86_64-toolchain      # 64-bit
pacman -S --noconfirm mingw32/mingw-w64-i686-clang mingw-w64-i686-toolchain  # 32-bit
```

If misconfigured or missing, set `R_HOME` to default `R` location (e.g. `C:\Program Files\R\R-4.0.3`).


Set `MSYS_ROOT` to the root of your `msys2` installation.
- **scoop**
  - **CMD**
      ```Shell
      set MSYS_ROOT="%USERPROFILE%\scoop\apps\msys2\current"
      ```
  - **PowerShell**
      ```PowerShell
      $env:MSYS_ROOT="$env:USERPROFILE\scoop\apps\msys2\current"
      ```
- **chocolatey**
  - **CMD**
      ```Shell
      set MSYS_ROOT=C:\tools\msys64
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

Add environment variable `LIBCLANG_PATH` with the value pointing to where the `clang` binaries are placed. This depends on whether the target architecture is `32` or `64` bit.

- **64 bit**
  - Set `LIBCLANG_PATH`
    - **CMD**
        ```Shell
        set LIBCLANG_PATH=%MSYS_ROOT%\mingw64\bin 
        ```
    - **PowerShell**
      ```PowerShell
      $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw64\bin"
      ```
  - then build (CMD/PowerShell)
    ```Shell
    cargo +stable-x86_64-pc-windows-msvc build --target=x86_64-pc-windows-gnu --features use-bindgen
    ``` 

- **32 bit**
  - Set `LIBCLANG_PATH`
    - **CMD**
        ```Shell
        set LIBCLANG_PATH=%MSYS_ROOT%\mingw32\bin 
        ```
    - **PowerShell**
      ```PowerShell
      $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw32\bin"
      ```
  - then build (CMD/PowerShell)
    ```Shell
    cargo +stable-i686-pc-windows-msvc build --target=i686-pc-windows-gnu --features use-bindgen
    ``` 

The output can be tested (with some additional steps to allow simultaneous `32` and `64` bit targets).

<details>
  <summary>Running tests on Windows</summary>

  - **64-bit**
    - **CMD**
      ```Shell
      set OLD_PATH=%PATH%                        # Captures current PATH
      set PATH=%R_HOME%\bin\x64;%PATH%           # Prepends with correct R directory
      set LIBCLANG_PATH=%MSYS_ROOT%\mingw64\bin  # Path to libclang
      cargo +stable-x86_64-pc-windows-msvc test --target x86_64-pc-windows-gnu --features use-bindgen -- --nocapture --test-threads=1
      set PATH=%OLD_PATH%                        # Resets PATH to the original value
      ```
    - **PowerShell**
      ```PowerShell
      $OLD_PATH=$env:PATH                              # Captures current PATH
      $env:PATH="$env:R_HOME\bin\x64;$env:PATH"        # Prepends with correct R directory
      $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw64\bin"  # Path to libclang
      cargo +stable-x86_64-pc-windows-msvc test --target x86_64-pc-windows-gnu --features use-bindgen -- --nocapture --test-threads=1
      $env:PATH=$OLD_PATH                              # Resets PATH to the original value
      ```
  - **32-bit**
    - **CMD**
      ```Shell
      set OLD_PATH=%PATH%                        # Captures current PATH
      set PATH=%R_HOME%\bin\i386;%PATH%          # Prepends with correct R directory
      set LIBCLANG_PATH=%MSYS_ROOT%\mingw32\bin  # Path to libclang
      cargo +stable-i686-pc-windows-msvc test --target i686-pc-windows-gnu --features use-bindgen -- --nocapture --test-threads=1
      set PATH=%OLD_PATH%                        # Resets PATH to the original value
      ```
    - **PowerShell**
      ```PowerShell
      $OLD_PATH=$env:PATH                              # Captures current PATH
      $env:PATH="$env:R_HOME\bin\i386;$env:PATH"       # Prepends with correct R directory
      $env:LIBCLANG_PATH="$env:MSYS_ROOT\mingw32\bin"  # Path to libclang
      cargo +stable-i686-pc-windows-msvc test --target i686-pc-windows-gnu --features use-bindgen -- --nocapture --test-threads=1
      $env:PATH=$OLD_PATH                              # Resets PATH to the original value
      ```


  </details>


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
