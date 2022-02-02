use std::{
    env,
    ffi::OsString,
    fs,
    io,
    io::{ Error, ErrorKind },
    path::{ Path, PathBuf },
    process::{ exit, Command },
};

#[cfg(target_family = "unix")]
use std::{
    os::unix::ffi::OsStrExt,
    ffi::OsStr,
};

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStringExt;

#[allow(dead_code)]
struct InstallationPaths {
    r_home: PathBuf,
    include: PathBuf,
    library: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug)]
struct RVersionInfo {
    major: String,
    minor: String,
    patch: String,
    devel: String,
    version_string: String,
}

#[derive(Debug)]
enum EnvVarError {
    EnvVarNotPresent,
    InvalidEnvVar(&'static str),
    RInvocationError(io::Error),
    InvalidROutput(&'static str)
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
        len = winapi::um::stringapiset::MultiByteToWideChar(cp, 0, bytes.as_ptr() as *const i8, bytes.len() as i32, std::ptr::null_mut(), 0);
        wide = Vec::with_capacity(len as usize);
        len = winapi::um::stringapiset::MultiByteToWideChar(cp, 0, bytes.as_ptr() as *const i8, bytes.len() as i32, wide.as_mut_ptr(), len);
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
    if let Some(r_home) = env::var_os("R_HOME") {
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
    };

// Get the path to the R library
fn get_r_library(r_home: &Path) -> PathBuf {
    let pkg_target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (cfg!(windows), pkg_target_arch.as_str()) {
        // For Windows
        (true, "x86_64") => Path::new(r_home).join("bin").join("x64"),
        (true, "i386") => Path::new(r_home).join("bin").join("i386"),
        (true, _) => panic!("Unknown architecture"),
        // For Unix-alike
        (false, _) => Path::new(r_home).join("lib"),
    }
}

// Get the path to the R include either from an envvar or by executing the actual R binary.
fn get_r_include(r_home: &Path, library: &Path) -> io::Result<PathBuf> {
    // If the environment variable R_INCLUDE_DIR is set we use it
    if let Some(include) = env::var_os("R_INCLUDE_DIR") {
        return Ok(PathBuf::from(include));
    }

    // Otherwise, we try to execute `R` to find the include dir. Here,
    // we're using the R home we found earlier, to make sure we're consistent.
    let r_binary = if cfg!(windows) {
        Path::new(&library).join("R.exe")
    } else {
        Path::new(&r_home).join("bin").join("R")
    };

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

fn str_vec_to_version(input: Vec<String>) -> Result<RVersionInfo, EnvVarError> {
    fn is_str_digit(s: &String) -> bool {
        s.chars().all(|c| c.is_digit(10))
    }

    if input.len() < 3 || input.len() > 4 {
        return Err(EnvVarError::InvalidEnvVar("Invalid format"));
    }

    let mut result = RVersionInfo {
        major: "".into(),
        minor: "".into(),
        patch: "".into(),
        devel: "".into(),
        version_string: "".into(), // Skip this assignment for now
    };

    if is_str_digit(&input[0]) {
        result.major = (&input[0]).to_string();
    } else {
        return Err(EnvVarError::InvalidEnvVar("Cannot find R major version"))
    }

    if is_str_digit(&input[1]) {
        result.minor = (&input[1]).to_string();
    } else {
        return Err(EnvVarError::InvalidEnvVar("Cannot find R minor version"))
    }

    if is_str_digit(&input[2]) {
        result.patch = (&input[2]).to_string();
    } else {
        return Err(EnvVarError::InvalidEnvVar("Cannot find R patch level"))
    }

    if input.len() == 4 {
        if input[3] == "devel" {
            result.devel = "-devel".into();
        } else {
            return Err(EnvVarError::InvalidEnvVar("Cannot find R development status"));
        }
    }

    Ok(result)
}


fn get_r_version_from_env(r_version_env_var:&str) -> Result<RVersionInfo, EnvVarError> {
    std::env::var(r_version_env_var)
        // Any error arising from reading env var is converted to this value
        .map_err(|_| EnvVarError::EnvVarNotPresent)
        .map(|v| {
            v.split(&['.', '-'][..])
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .and_then(str_vec_to_version)
}

fn get_r_version_from_r(r_paths: &InstallationPaths) -> Result<RVersionInfo, EnvVarError> {
    let r_binary = if cfg!(windows) {
        Path::new(&r_paths.library).join("R.exe")
    } else {
        Path::new(&r_paths.r_home).join("bin").join("R")
    };

    // This R script prints two lines; the first line contains the R version ,
    // and the second line is `TRUE` when the installed R is the development
    // build.
    //
    // Example 1) R 4.1.2 (released version)
    //
    // ```
    // 4.1.2
    // FALSE
    // ```
    //
    // Example 2) R 4.2.0 (devel version)
    //
    // ```
    // 4.1.2
    // TRUE
    // ```
    let out = r_command(
        &r_binary,
        r#"cat(sprintf("%s.%s\n", R.version$major, R.version$minor)); cat(isTRUE(grepl("devel", R.version$status, fixed = TRUE)))"#,
    )
        .map_err(EnvVarError::RInvocationError)?;

    let out = out.as_os_str().to_string_lossy().into_owned();
    let mut lines = out.lines();

    // Process the first line of the output
    let r_vers = match lines.next() {
        Some(v) => v,
        None => return Err(EnvVarError::InvalidROutput("Cannot find R version")),
    };

    let mut r_vers = r_vers.split('.');
    let (major, minor, patch) = match (r_vers.next(), r_vers.next(), r_vers.next()) {
        // If the line contains all the three versions as expected, use the result
        (Some(major), Some(minor), Some(patch)) => {
            (major.to_string(), minor.to_string(), patch.to_string())
        }
        // Otherwise, return the corresponding errors.
        (None, _, _) => return Err(EnvVarError::InvalidROutput("Cannot find R major version")),
        (_, None, _) => return Err(EnvVarError::InvalidROutput("Cannot find R minor version")),
        (_, _, None) => return Err(EnvVarError::InvalidROutput("Cannot find R patch level")),
    };

    // Process the second line of the output
    let devel = match lines.next() {
        Some("TRUE") => "-devel".to_string(),
        Some(_) => "".to_string(),
        _ => {
            return Err(EnvVarError::InvalidROutput(
                "Cannot find R development status",
            ))
        }
    };

    let version_string = match lines.next() {
        Some(line) => line.to_string(),
        _ => return Err(EnvVarError::InvalidROutput("Cannot find R version string")),
    };

    Ok(RVersionInfo {
        major,
        minor,
        patch,
        devel,
        version_string,
    })
}

fn get_r_version(r_version_env_var: &str, r_paths: &InstallationPaths) -> Result<RVersionInfo, EnvVarError> {
    match get_r_version_from_env(r_version_env_var)
    {
        Ok(v) => Ok(v),
        Err(EnvVarError::InvalidEnvVar(e)) => Err(EnvVarError::InvalidEnvVar(e)),
        Err(_) => get_r_version_from_r(r_paths)
    }
}

fn set_r_version_vars(ver: &RVersionInfo) {
    println!("cargo:r_version_major={}", ver.major); // Becomes DEP_R_R_VERSION_MAJOR for clients
    println!("cargo:r_version_minor={}", ver.minor); // Becomes DEP_R_R_VERSION_MINOR for clients
    println!("cargo:r_version_patch={}", ver.patch); // Becomes DEP_R_R_VERSION_PATCH for clients
    if ver.devel.is_empty() {
        println!("cargo:r_version_devel=false"); // Becomes DEP_R_R_VERSION_DEVEL for clients
    } else {
        println!("cargo:r_version_devel=true");
    }
    println!(r#"cargo:r_version_string="{}""#, ver.version_string); // Becomes DEP_R_R_VERSION_STRING for clients
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

    println!("Generating bindings for target: {}, os: {}, architecture: {}", target, target_os, target_arch);
    // Point to the correct headers
    bindgen_builder = bindgen_builder.clang_args(&[
        format!("-I{}", r_paths.include.display()),
        format!("--target={}", target)
    ]);

    // allow injection of an alternative include path to libclang
    if let Some(alt_include) = env::var_os("LIBRSYS_LIBCLANG_INCLUDE_PATH") {
        bindgen_builder = bindgen_builder.clang_arg(
            format!("-I{}", PathBuf::from(alt_include).display()),
        );
    }

    // Blacklist some types on i686
    // https://github.com/rust-lang/rust-bindgen/issues/1823
    // https://github.com/rust-lang/rust/issues/54341
    // https://github.com/extendr/libR-sys/issues/39
    if target_os == "windows" && target_arch == "x86" {
        bindgen_builder = 
            bindgen_builder
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
    if let Some(alt_target) = env::var_os("LIBRSYS_BINDINGS_OUTPUT_PATH") {
        let out_path = PathBuf::from(alt_target);
        // if folder doesn't exist, try to create it
        if !out_path.exists() {
            fs::create_dir(&out_path)
                .expect(&format!("Couldn't create output directory for bindings: {}", out_path.display()));
        }

        let out_file = out_path.join(
                format!(
                    "bindings-{}-{}-R{}.{}{}.rs",
                    target_os, target_arch, version_info.major, version_info.minor, version_info.devel
                )
            );

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
        env::var_os("LIBRSYS_BINDINGS_PATH")
        .unwrap_or(OsString::from("bindings"))
    );
    
    // we try a few different file names, from more specific to less specific
    let bindings_file_full = PathBuf::from(
        format!(
            "bindings-{}-{}-R{}.{}{}.rs",
            target_os, target_arch, version_info.major, version_info.minor, version_info.devel
        )
    );
    let bindings_file_novers = PathBuf::from(
        format!("bindings-{}-{}.rs", target_os, target_arch)
    );

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
        PathBuf::from(env::var_os("OUT_DIR").unwrap())
            .join("bindings.rs")
    ).expect("No precomputed bindings available!");
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
    let version_info = get_r_version("LIBR_SYS_R_VERSION", &r_paths).expect("Could not obtain R version");
    set_r_version_vars(&version_info);


    #[cfg(feature = "use-bindgen")]
        generate_bindings(&r_paths, &version_info);
    #[cfg(not(feature = "use-bindgen"))]
        retrieve_prebuild_bindings(&version_info);
}
