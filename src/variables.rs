use std::env;
use std::path::PathBuf;

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .unwrap()
    )
}

pub fn is_release() -> bool {
    env::var("PROFILE").unwrap() == "release"
}