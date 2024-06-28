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

// The environmental variables that are usually set by R. These might be needed
// to set manually if we compile libR-sys outside of an R session.
//
// c.f., https://stat.ethz.ch/R-manual/R-devel/library/base/html/EnvVar.html
const ENVVAR_R_HOME: &str = "R_HOME";

// An R version (e.g., "4.1.2" or "4.2.0-devel"). When this is set, the actual R
// binary is not executed. This might be useful in some cases of cross-compile.
// c.f., https://github.com/extendr/libR-sys/issues/85
const ENVVAR_R_VERSION: &str = "LIBRSYS_R_VERSION";

// A path to a dir containing pre-computed bindings (default: "bindings").
const ENVVAR_BINDINGS_PATH: &str = "LIBRSYS_BINDINGS_PATH";

#[derive(Debug)]
struct InstallationPaths {
    r_home: PathBuf,
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
    full: String,
}

impl RVersionInfo {
    /// Returns the name for precompiled bindings, given R version and targets.
    /// e.g. `bindings-windows-x86_64-R4.4-devel.rs`
    fn get_r_bindings_filename(&self, target_os: &str, target_arch: &str) -> PathBuf {
        let devel_suffix = if self.devel { "-devel" } else { "" };
        let major = &self.major;
        let minor = &self.minor;
        PathBuf::from(format!(
            "bindings-{target_os}-{target_arch}-R{major}.{minor}{devel_suffix}.rs"
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

#[link(name = "kernel32")]
#[cfg(target_family = "windows")]
extern "system" {
    #[link_name = "GetConsoleCP"]
    fn get_console_code_page() -> u32;
    #[link_name = "MultiByteToWideChar"]
    fn multi_byte_to_wide_char(
        CodePage: u32,
        dwFlags: u32,
        lpMultiByteStr: *const u8,
        cbMultiByte: i32,
        lpWideCharStr: *mut u16,
        cchWideChar: i32,
    ) -> i32;
}

// convert bytes to wide-encoded characters on Windows
// from: https://stackoverflow.com/a/40456495/4975218
#[cfg(target_family = "windows")]
fn wide_from_console_string(bytes: &[u8]) -> Vec<u16> {
    assert!(bytes.len() < std::i32::MAX as usize);
    let mut wide;
    let mut len;
    unsafe {
        let cp = get_console_code_page();
        len = multi_byte_to_wide_char(
            cp,
            0,
            bytes.as_ptr() as *const u8,
            bytes.len() as i32,
            std::ptr::null_mut(),
            0,
        );
        wide = Vec::with_capacity(len as usize);
        len = multi_byte_to_wide_char(
            cp,
            0,
            bytes.as_ptr() as *const u8,
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
    // we must use --vanilla,
    // 1. user Rprofile may contain message into stdout
    // 2. prevent R startup message
    let out = Command::new(r_binary)
        .args(["-s", "--vanilla", "-e", script])
        .output()?;

    // if there are any errors we print them out, helps with debugging
    if !out.stderr.is_empty() {
        println!(
            "cargo:warning={}",
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

fn probe_r_paths() -> io::Result<InstallationPaths> {
    // First we locate the R home
    let r_home = get_r_home()?;

    // Now the library location. On Windows, it depends on the target architecture
    let library = get_r_library(&r_home);

    Ok(InstallationPaths {
        r_home,
        library,
    })
}

// Parse an R version (e.g. "4.1.2" and "4.2.0-devel") and return the RVersionInfo.
fn parse_r_version(r_version: String) -> Result<RVersionInfo, EnvVarError> {
    let full = r_version.clone();
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
            if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
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
        full,
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
        r_binary,
        r"cat(sprintf('%s.%s%s\n', R.version$major, R.version$minor, if(isTRUE(grepl('devel', R.version$status, fixed = TRUE))) '-devel' else ''))",
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
        // In the case of any error other than the absence of envvar, stop with
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

/// Retrieve bindings from cache, if available. Errors out otherwise.
fn retrieve_prebuild_bindings(version_info: &RVersionInfo) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let bindings_path = PathBuf::from(
        env::var_os(ENVVAR_BINDINGS_PATH).unwrap_or_else(|| OsString::from("bindings")),
    );

    // we try a few different file names, from more specific to less specific
    let bindings_file_full = version_info.get_r_bindings_filename(&target_os, &target_arch);
    let bindings_file_novers = PathBuf::from(format!("bindings-{target_os}-{target_arch}.rs"));

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
                "cargo:warning=using generic {}-{} libR-sys bindings. These may not work for R {}.{}.{}{}.",
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
            println!(
                "cargo:warning=Problem locating local R install: {:?}",
                error
            );
            exit(1);
        }
    };

    println!("cargo:rustc-env=R_HOME={}", r_paths.r_home.display());
    println!("cargo:r_home={}", r_paths.r_home.display()); // Becomes DEP_R_R_HOME for clients

    // TODO: r_library might not exist in some types of installation that
    // doesn't provide libR, R's shared library; in such a situation, just skip
    // setting `rustc-link-search`. Probably this setting itself is not used at
    // all except when compiled for testing, but we are not sure at the moment.
    if let Ok(r_library) = r_paths.library.canonicalize() {
        println!("cargo:rustc-link-search={}", r_library.display());
    }
    println!("cargo:rustc-link-lib=dylib=R");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // extract version info from R and output for use by downstream crates
    let version_info =
        get_r_version(ENVVAR_R_VERSION, &r_paths).expect("Could not obtain R version");
    set_r_version_vars(&version_info);

    retrieve_prebuild_bindings(&version_info);
}
