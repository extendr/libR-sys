use std::{
    env,
    ffi::{OsStr, OsString},
    fs, io,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::{exit, Command},
};

#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStringExt;

//
// Environmental variables
//

// The environmental variables that are usually set by R. These might be needed
// to set manually if we compile libR-sys outside of an R session.
//
// c.f., https://stat.ethz.ch/R-manual/R-devel/library/base/html/EnvVar.html
const ENVVAR_R_INCLUDE_DIR: &str = "R_INCLUDE_DIR";
const ENVVAR_R_HOME: &str = "R_HOME";

// An R version (e.g., "4.1.2" or "4.2.0-devel"). When this is set, the actual R
// binary is not executed. This might be useful in some cases of cross-compile.
// c.f., https://github.com/extendr/libR-sys/issues/85
const ENVVAR_R_VERSION: &str = "LIBRSYS_R_VERSION";

// A path to a dir containing pre-computed bindings (default: "bindings").
const ENVVAR_BINDINGS_PATH: &str = "LIBRSYS_BINDINGS_PATH";

// A path to libclang toolchain. If this is set, the path is added to the
// compiler arguments on executing bindgen.
#[cfg(feature = "use-bindgen")]
const ENVVAR_LIBCLANG_INCLUDE_PATH: &str = "LIBRSYS_LIBCLANG_INCLUDE_PATH";

// A path to an output dir of bindings in addition to the default "bindings"
// dir. If this is set, generated bindings are also put there.
#[cfg(feature = "use-bindgen")]
const ENVVAR_BINDINGS_OUTPUT_PATH: &str = "LIBRSYS_BINDINGS_OUTPUT_PATH";

#[allow(dead_code)]
struct InstallationPaths {
    r_home: PathBuf,
    include: PathBuf,
    library: PathBuf,
}

impl InstallationPaths {
    fn get_r_binary(&self) -> PathBuf {
        if cfg!(windows) {
            Path::new(&self.library).join("R.exe")
        } else {
            Path::new(&self.r_home).join("bin").join("R")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct RVersionInfo {
    major: String,
    minor: String,
    patch: String,
    devel: bool,
}

impl RVersionInfo {
    fn get_r_bindings_filename(&self, target_os: &str, target_arch: &str) -> PathBuf {
        let devel_suffix = if self.devel { "-devel" } else { "" };
        PathBuf::from(format!(
            "bindings-{}-{}-R{}.{}{}.rs",
            target_os, target_arch, self.major, self.minor, devel_suffix
        ))
    }
}

#[derive(Debug)]
enum EnvVarError {
    EnvVarNotPresent,
    InvalidEnvVar(&'static str),
    RInvocationError(io::Error),
    InvalidROutput(&'static str),
}

// frustratingly, something like the following does not exist in an
// OS-independent way in Rust
#[cfg(target_family = "unix")]
fn byte_array_to_os_string(bytes: &[u8]) -> OsString {
    let os_str = OsStr::from_bytes(bytes);
    os_str.to_os_string()
}

// convert bytes to wide-encoded characters on Windows
// from: https://stackoverflow.com/a/40456495/4975218
#[cfg(target_family = "windows")]
fn wide_from_console_string(bytes: &[u8]) -> Vec<u16> {
    assert!(bytes.len() < std::i32::MAX as usize);
    let mut wide;
    let mut len;
    unsafe {
        let cp = winapi::um::consoleapi::GetConsoleCP();
        len = winapi::um::stringapiset::MultiByteToWideChar(
            cp,
            0,
            bytes.as_ptr() as *const i8,
            bytes.len() as i32,
            std::ptr::null_mut(),
            0,
        );
        wide = Vec::with_capacity(len as usize);
        len = winapi::um::stringapiset::MultiByteToWideChar(
            cp,
            0,
            bytes.as_ptr() as *const i8,
            bytes.len() as i32,
            wide.as_mut_ptr(),
            len,
        );
        wide.set_len(len as usize);
    }
    wide
}

#[cfg(target_family = "windows")]
fn byte_array_to_os_string(bytes: &[u8]) -> OsString {
    // first, use Windows API to convert to wide encoded
    let wide = wide_from_console_string(bytes);
    // then, use `std::os::windows::ffi::OsStringExt::from_wide()`
    OsString::from_wide(&wide)
}

// Execute an R script and return the captured output
fn r_command<S: AsRef<OsStr>>(r_binary: S, script: &str) -> io::Result<OsString> {
    let out = Command::new(r_binary)
        .args(&["-s", "-e", script])
        .output()?;

    // if there are any errors we print them out, helps with debugging
    if !out.stderr.is_empty() {
        println!(
            "> {}",
            byte_array_to_os_string(&out.stderr)
                .as_os_str()
                .to_string_lossy()
        );
    }

    Ok(byte_array_to_os_string(&out.stdout))
}

// Get the path to the R home either from an envvar or by executing the actual R binary on PATH.
fn get_r_home() -> io::Result<PathBuf> {
    // If the environment variable R_HOME is set we use it
    if let Some(r_home) = env::var_os(ENVVAR_R_HOME) {
        return Ok(PathBuf::from(r_home));
    }

    // Otherwise, we try to execute `R` to find `R_HOME`. Note that this is
    // discouraged, see Section 1.6 of "Writing R Extensions"
    // https://cran.r-project.org/doc/manuals/r-release/R-exts.html#Writing-portable-packages
    let rout = r_command("R", r#"cat(normalizePath(R.home()))"#)?;
    if !rout.is_empty() {
        Ok(PathBuf::from(rout))
    } else {
        Err(Error::new(ErrorKind::Other, "Cannot find R home."))
    }
}

// Get the path to the R library
fn get_r_library(r_home: &Path) -> PathBuf {
    let pkg_target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (cfg!(windows), pkg_target_arch.as_str()) {
        // For Windows
        (true, "x86_64") => Path::new(r_home).join("bin").join("x64"),
        (true, "x86") => Path::new(r_home).join("bin").join("i386"),
        (true, _) => panic!("Unknown architecture"),
        // For Unix-alike
        (false, _) => Path::new(r_home).join("lib"),
    }
}

// Get the path to the R include directory either from an envvar or by executing the actual R binary.
fn get_r_include(r_home: &Path, library: &Path) -> io::Result<PathBuf> {
    // If the environment variable R_INCLUDE_DIR is set we use it
    if let Some(include) = env::var_os(ENVVAR_R_INCLUDE_DIR) {
        return Ok(PathBuf::from(include));
    }

    // Otherwise, we try to execute `R` to find the include dir. Here,
    // we're using the R home we found earlier, to make sure we're consistent.
    let r_binary = InstallationPaths {
        r_home: r_home.to_path_buf(),
        include: PathBuf::new(), // get_r_binary() doesn't use `include` so fill with an empty PathBuf.
        library: library.to_path_buf(),
    }
    .get_r_binary();

    let rout = r_command(&r_binary, r#"cat(normalizePath(R.home('include')))"#)?;
    if !rout.is_empty() {
        Ok(PathBuf::from(rout))
    } else {
        Err(Error::new(ErrorKind::Other, "Cannot find R include."))
    }
}

fn probe_r_paths() -> io::Result<InstallationPaths> {
    // First we locate the R home
    let r_home = get_r_home()?;

    // Now the library location. On Windows, it depends on the target architecture
    let library = get_r_library(&r_home);

    // Finally the include location. It may or may not be located under R home
    let include = get_r_include(&r_home, &library)?;

    Ok(InstallationPaths {
        r_home,
        include,
        library,
    })
}

// Parse an R version (e.g. "4.1.2" and "4.2.0-devel") and return the RVersionInfo.
fn parse_r_version(r_version: String) -> Result<RVersionInfo, EnvVarError> {
    // First, split "<major>.<minor>.<patch>-devel" to "<major>.<minor>.<patch>" and "devel"
    let (r_version, devel) = match *r_version.split('-').collect::<Vec<&str>>().as_slice() {
        [r_version, devel] => (r_version, Some(devel)),
        [r_version] => (r_version, None),
        // if the length is more than 2 or 0, the version is in invalid format
        _ => return Err(EnvVarError::InvalidEnvVar("Invalid format")),
    };

    // Split "<major>.<minor>.<patch>" to "<major>", "<minor>", and "<patch>"
    let r_version_split = r_version
        .split('.')
        .map(|s| {
            // Good:
            //   - "4.1.2"
            //
            // Bad:
            //   - "4.1.foo" (some part contains any non-digit characters)
            //   - "4.1." (some part is missing)
            if !s.is_empty() && s.chars().all(|c| c.is_digit(10)) {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<Option<&str>>>();

    let (major, minor, patch) = match *r_version_split.as_slice() {
        // if any of the first three items doesn't exist, the format is invalid
        [] | [None, ..] => return Err(EnvVarError::InvalidEnvVar("Cannot find R major version")),
        [_, None, ..] => return Err(EnvVarError::InvalidEnvVar("Cannot find R minor version")),
        [_, _, None, ..] => return Err(EnvVarError::InvalidEnvVar("Cannot find R patch level")),
        // if all of the first three items exist, the format is valid
        [Some(major), Some(minor), Some(patch)] => {
            (major.to_string(), minor.to_string(), patch.to_string())
        }
        // if the length is longer than 3, the format is invalid
        _ => return Err(EnvVarError::InvalidEnvVar("Invalid format")),
    };

    let devel = match devel {
        Some("devel") => true,
        Some(_) => {
            return Err(EnvVarError::InvalidEnvVar(
                "Cannot find R development status",
            ))
        }
        None => false,
    };

    Ok(RVersionInfo {
        major,
        minor,
        patch,
        devel,
    })
}

fn get_r_version_from_env(r_version_env_var: &str) -> Result<RVersionInfo, EnvVarError> {
    std::env::var(r_version_env_var)
        // Any error arising from reading env var is converted to this value
        .map_err(|_| EnvVarError::EnvVarNotPresent)
        .and_then(parse_r_version)
}

fn get_r_version_from_r(r_paths: &InstallationPaths) -> Result<RVersionInfo, EnvVarError> {
    let r_binary = r_paths.get_r_binary();

    // This R script prints the R version to stdout.
    //
    // Example 1) R 4.1.2 (released version)
    //
    // ```
    // 4.1.2
    // ```
    //
    // Example 2) R 4.2.0 (development version)
    //
    // ```
    // 4.2.0-devel
    // ```
    let out = r_command(
        &r_binary,
        r#"cat(sprintf('%s.%s%s\n', R.version$major, R.version$minor, if(isTRUE(grepl('devel', R.version$status, fixed = TRUE))) '-devel' else ''))"#,
    )
        .map_err(EnvVarError::RInvocationError)?;

    let out = out.as_os_str().to_string_lossy().into_owned();
    let mut lines = out.lines();

    // Process the first line of the output
    match lines.next() {
        Some(v) => parse_r_version(v.to_string()),
        None => Err(EnvVarError::InvalidROutput("Cannot find R version")),
    }
}

fn get_r_version(
    r_version_env_var: &str,
    r_paths: &InstallationPaths,
) -> Result<RVersionInfo, EnvVarError> {
    // Try looking for the envvar first.
    match get_r_version_from_env(r_version_env_var) {
        // If the envvar is found and it can be parsed as a valid RVersionInfo, use it.
        Ok(v) => Ok(v),
        // If the envvar is not present, then use the actual R binary to get the version.
        Err(EnvVarError::EnvVarNotPresent) => get_r_version_from_r(r_paths),
        // In the case of any error other than the absense of envvar, stop with
        // that error because it means the envvar is set and something is wrong.
        e @ Err(_) => e,
    }
}

fn set_r_version_vars(ver: &RVersionInfo) {
    println!("cargo:r_version_major={}", ver.major); // Becomes DEP_R_R_VERSION_MAJOR for clients
    println!("cargo:r_version_minor={}", ver.minor); // Becomes DEP_R_R_VERSION_MINOR for clients
    println!("cargo:r_version_patch={}", ver.patch); // Becomes DEP_R_R_VERSION_PATCH for clients
    println!("cargo:r_version_devel={}", ver.devel); // Becomes DEP_R_R_VERSION_DEVEL for clients
}

#[cfg(feature = "use-bindgen")]
/// Generate bindings by calling bindgen.
fn generate_bindings(r_paths: &InstallationPaths, version_info: &RVersionInfo) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindgen_builder = bindgen::Builder::default()
        // These constants from libm break bindgen.
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let target = env::var("TARGET").expect("Could not get the target triple");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    println!(
        "Generating bindings for target: {}, os: {}, architecture: {}",
        target, target_os, target_arch
    );
    // Point to the correct headers
    bindgen_builder = bindgen_builder.clang_args(&[
        format!("-I{}", r_paths.include.display()),
        format!("--target={}", target),
    ]);

    // allow injection of an alternative include path to libclang
    if let Some(alt_include) = env::var_os(ENVVAR_LIBCLANG_INCLUDE_PATH) {
        bindgen_builder =
            bindgen_builder.clang_arg(format!("-I{}", PathBuf::from(alt_include).display()));
    }

    // Blacklist some types on i686
    // https://github.com/rust-lang/rust-bindgen/issues/1823
    // https://github.com/rust-lang/rust/issues/54341
    // https://github.com/extendr/libR-sys/issues/39
    if target_os == "windows" && target_arch == "x86" {
        bindgen_builder = bindgen_builder
            .blacklist_item("max_align_t")
            .blacklist_item("__mingw_ldbl_type_t");
    }

    // Finish the builder and generate the bindings.
    let bindings = bindgen_builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings to default output path!");

    // Also write the bindings to a folder specified by LIBRSYS_BINDINGS_OUTPUT_PATH, if defined
    if let Some(alt_target) = env::var_os(ENVVAR_BINDINGS_OUTPUT_PATH) {
        let out_path = PathBuf::from(alt_target);
        // if folder doesn't exist, try to create it
        if !out_path.exists() {
            fs::create_dir(&out_path).expect(&format!(
                "Couldn't create output directory for bindings: {}",
                out_path.display()
            ));
        }

        let bindings_file_full = version_info.get_r_bindings_filename(&target_os, &target_arch);
        let out_file = out_path.join(bindings_file_full);

        bindings
            .write_to_file(&out_file)
            .expect(&format!("Couldn't write bindings: {}", out_file.display()));
    }
}

#[allow(dead_code)]
/// Retrieve bindings from cache, if available. Errors out otherwise.
fn retrieve_prebuild_bindings(version_info: &RVersionInfo) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let bindings_path = PathBuf::from(
        env::var_os(ENVVAR_BINDINGS_PATH).unwrap_or_else(|| OsString::from("bindings")),
    );

    // we try a few different file names, from more specific to less specific
    let bindings_file_full = version_info.get_r_bindings_filename(&target_os, &target_arch);
    let bindings_file_novers = PathBuf::from(format!("bindings-{}-{}.rs", target_os, target_arch));

    let mut from = bindings_path.join(bindings_file_full);
    if !from.exists() {
        from = bindings_path.join(bindings_file_novers);
        if !from.exists() {
            panic!(
                "Cannot find libR-sys bindings file for R {}.{}.{}{} on {} in {}. Consider compiling with --features use-bindgen.",
                version_info.major, version_info.minor, version_info.patch, version_info.devel, target_os, bindings_path.display()
            )
        } else {
            println!(
                "Warning: using generic {}-{} libR-sys bindings. These may not work for R {}.{}.{}{}.",
                target_os, target_arch, version_info.major, version_info.minor, version_info.patch, version_info.devel
            );
        }
    }

    fs::copy(
        &from,
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"),
    )
    .expect("No precomputed bindings available!");
    println!("cargo:rerun-if-changed={}", from.display());
}

fn main() {
    let r_paths = probe_r_paths();

    let r_paths = match r_paths {
        Ok(result) => result,
        Err(error) => {
            println!("Problem locating local R install: {:?}", error);
            exit(1);
        }
    };

    println!("cargo:rustc-env=R_HOME={}", r_paths.r_home.display());
    println!("cargo:r_home={}", r_paths.r_home.display()); // Becomes DEP_R_R_HOME for clients

    // make sure cargo links properly against library
    println!("cargo:rustc-link-search={}", r_paths.library.display());
    println!("cargo:rustc-link-lib=dylib=R");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // extract version info from R and output for use by downstream crates
    let version_info =
        get_r_version(ENVVAR_R_VERSION, &r_paths).expect("Could not obtain R version");
    set_r_version_vars(&version_info);

    #[cfg(feature = "use-bindgen")]
    generate_bindings(&r_paths, &version_info);
    #[cfg(not(feature = "use-bindgen"))]
    retrieve_prebuild_bindings(&version_info);
}
