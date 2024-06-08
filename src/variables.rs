use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::variables::Platform::{Linux, MacOS, Windows};

#[derive(PartialEq)]
pub (crate) enum Platform {
    Windows,
    Linux,
    MacOS,
}

pub (crate) fn platform() -> Platform {
    let target = env::var("TARGET")
        .unwrap();

    if target.contains("windows") { return Windows }
    else if target.contains("linux") { return Linux }
    else if target.contains("apple-darwin") { return MacOS }

    panic!("Platform not supported: {}", target);
}

pub (crate) fn static_library_extension() -> &'static str {
    match platform() {
        Windows => ".lib",
        Linux   => ".a",
        MacOS   => ".a",
    }
}

pub (crate) fn shared_library_extension() -> &'static str {
    match platform() {
        Windows => ".dll",
        Linux   => ".so",
        MacOS   => ".dylib",
    }
}

pub (crate) fn out_directory() -> PathBuf {
    PathBuf::from(
        env::var("OUT_DIR")
            .unwrap()
    )
}

pub (crate) fn get_profile() -> String {
    env::var("PROFILE").unwrap()
}

// Credits cxx-build
// https://docs.rs/cxx-build/latest/src/cxx_build/target.rs.html#10-49
pub(crate) fn target_directory(out_dir: &Path) -> PathBuf {

    if let Some(target_dir) = env::var_os("CARGO_TARGET_DIR") {
        let target_dir = PathBuf::from(target_dir);
        return if target_dir.is_absolute() {
            target_dir
        } else {
            out_dir.to_path_buf()
        };
    }

    // fs::canonicalize on Windows produces UNC paths which cl.exe is unable to
    // handle in includes.
    // https://github.com/rust-lang/rust/issues/42869
    // https://github.com/alexcrichton/cc-rs/issues/169
    let mut also_try_canonical = cfg!(not(windows));
    let mut dir = out_dir.to_owned();

    loop {
        if dir.join(".rustc_info.json").exists()
            || dir.join("CACHEDIR.TAG").exists()
            || dir.file_name() == Some(OsStr::new("target"))
            && dir
            .parent()
            .map_or(false, |parent| parent.join("Cargo.toml").exists())
        {
            return dir
        }

        if dir.pop() {
            continue;
        }

        if also_try_canonical {
            if let Ok(canonical_dir) = out_dir.canonicalize() {
                dir = canonical_dir;
                also_try_canonical = false;
                continue;
            }
        }

        return out_dir.to_path_buf()
    }
}
