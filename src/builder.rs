use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use cmake::Config;
use crate::variables::is_release;

pub fn build_repo(
    path: &Path,
    targets: Vec<&str>,
    defines: HashMap<&str, &str>,
) -> PathBuf {

    let build_type = if is_release() {
        "Release"
    } else {
        "RelWithDebugInfo"
    };

    let configure_dir = if is_release() {
        path.join("cmake-build-release")
    } else {
        path.join("cmake-build-debug")
    };

    let mut config = Config::new(path);

    config.generator("Ninja");
    config.out_dir(configure_dir.clone());
    config.profile(build_type);

    for target in targets {
        config.build_target(target);
    }

    // If we are building the project, make sure we install all dependencies to the build folder
    let install_dir = configure_dir
        .join("install");

    config.define("CMAKE_SKIP_INSTALL_ALL_DEPENDENCY", "true");
    config.define("CMAKE_INSTALL_PREFIX", install_dir.to_str().unwrap());

    for (key, value) in defines {
        config.define(
            OsStr::new(key),
            OsStr::new(value)
        );
    }

    // After building, install our target to the configure directory.
    let build_dir = config.build();

    Command::new("ninja")
        .arg("install")
        .current_dir(build_dir)
        .status()
        .expect("Could not install repo, is Ninja installed?");

    return install_dir;
}


