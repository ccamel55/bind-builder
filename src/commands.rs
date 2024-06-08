use std::fmt::Display;
use std::path::Path;

pub (crate) fn print_warning<T: Display>(message: T) {
    println!("cargo:warning={}", message);
}

pub (crate) fn add_library_search_path(path: &Path) {
    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
}

pub (crate) fn link_static_library<T: Display>(lib_name: T) {
    println!("cargo:rustc-link-lib=static={}", lib_name);
}

pub (crate) fn link_shared_library<T: Display>(lib_name: T) {
    println!("cargo:rustc-link-lib=dylib={}", lib_name);
}