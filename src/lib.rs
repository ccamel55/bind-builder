use std::ops::Add;
use crate::commands::{add_library_search_path, link_shared_library, link_static_library, print_warning};
use crate::types::local_library::LocalLibrary;
use crate::variables::{shared_library_extension, static_library_extension};

pub mod types;

pub (crate) mod variables;
pub (crate) mod commands;

const LIBRARY_NAME_PREFIX: &str = "lib";

// todo: check naming conventions for windows MSVC
fn get_static_library_name(library_name: &str) -> String {
    LIBRARY_NAME_PREFIX.to_string()
        .add(library_name)
        .add(static_library_extension())
}

// note: rustc can't link against specific versions of .so
fn get_shared_library_name(library_name: &str) -> String  {
    LIBRARY_NAME_PREFIX.to_string()
        .add(library_name)
        .add(shared_library_extension())
}

// pub fn use_relative_r_path() {
//     match platform() {
//         Linux => println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN"),
//         MacOS => println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path"),
//         _ => {}
//     }
// }

// Extend cc::Build to accept our library
pub trait BindBuild {

    #[allow(private_bounds)]
    fn bind_library(
        &mut self,
        library: LocalLibrary
    ) -> &mut cc::Build;
}

impl BindBuild for cc::Build{

    #[allow(private_bounds)]
    fn bind_library(
        &mut self,
        library: LocalLibrary
    ) -> &mut cc::Build {

        // Remove duplicates and invalid entries
        let mut include_directories = library
            .get_include_directories()
            .clone();

        include_directories.dedup();
        include_directories.retain(|x| x.is_dir());

        let mut library_directories = library
            .get_library_directories()
            .clone();

        library_directories.dedup();
        library_directories.retain(|x| x.is_dir());

        let mut link_targets = library
            .get_link_targets()
            .clone();

        link_targets.dedup();

        let static_libraries: Vec<String> =  link_targets.iter()
            .filter(|x| {
                // todo: optimise this
                for library_directory in library_directories.iter() {
                    if library_directory
                        .join(get_static_library_name(x))
                        .exists() {
                        return true
                    }
                }
                false
            })
            .cloned()
            .collect();

        let shared_libraries: Vec<String> =  link_targets.iter()
            .filter(|x| {
                // todo: optimise this
                for library_directory in library_directories.iter() {
                    if library_directory
                        .join(get_shared_library_name(x))
                        .is_file() {
                        return true
                    }
                }
                false
            })
            .cloned()
            .collect();

        assert!(
            !static_libraries.is_empty() || !shared_libraries.is_empty(),
            "Library does not contain linkable targets."
        );

        // Add header include path
        self.includes(include_directories);

        // Add library search path
        for library_directory in library_directories {
            add_library_search_path(library_directory.as_path())
        }

        // Link respective libraries
        for static_library in static_libraries {
            link_static_library(static_library)
        }

        // todo: option to use built shared library/copy them to program path and link r path
        for shared_library in shared_libraries {
            print_warning(format!("Linking shared library: {}", shared_library));
            link_shared_library(shared_library)
        }

        self
    }
}
