use std::env;
use std::path::PathBuf;
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

// Credits: https://github.com/Rust-SDL2/rust-sdl2/blob/master/sdl2-sys/build.rs#L388C1-L408C2
pub (crate) fn target_directory() -> PathBuf {

    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = out_directory();

    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }

        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }

    out_dir.pop();
    out_dir.pop();

    out_dir
}