use std::fmt::Display;
use std::path::Path;
use crate::variables::platform;
use crate::variables::Platform::{Linux, MacOS};

// fn static_lib_ext() -> &'static str {
//     return match platform() {
//         Windows => {
//             "lib"
//         },
//         _ => {
//             "a"
//         }
//     }
// }
//
// fn shared_lib_ext() -> &'static str {
//     return match platform() {
//         Windows => {
//             "dll"
//         },
//         Linux => {
//             "so"
//         },
//         MacOS => {
//             "dylib"
//         }
//     }
// }

pub fn use_relative_r_path() {
    match platform() {
        Linux => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        },
        MacOS => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        }
        _ => {}
    }
}

pub fn add_install_dir(
    path: &Path,
) {
    // Where to look for libs
    println!("cargo:rustc-link-search=native={}", path.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", path.join("lib64").to_str().unwrap());
}

pub fn link_static_lib<T: Display>(lib_name: T) {
    println!("cargo:rustc-link-lib=static={}", lib_name);
}

