use std::env;
use std::path::PathBuf;
use crate::variables::Platform::{Linux, MacOS, Windows};

#[derive(PartialEq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
}

pub fn platform() -> Platform {
    let target = env::var("TARGET")
        .unwrap();

    if target.contains("windows") {
        return Windows
    } else if target.contains("linux") {
        return Linux
    } else if target.contains("apple-darwin") {
        return MacOS
    }

    panic!("Platform not supported: {}", target);
}

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .unwrap()
    )
}

pub fn is_release() -> bool {
    env::var("PROFILE").unwrap() == "release"
}

