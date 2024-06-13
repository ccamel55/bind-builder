//! Build dependency for downloading, building and linking native libraries.
//!
//! This crate expands on `cc` and `cmake` to provide a more streamlined way of distributing crates
//! that depend on both static and shared libraries.
//!

use std::fs;
use std::ops::Add;
use std::path::Path;
use crate::commands::{add_library_search_path, link_shared_library, link_static_library};
use crate::types::local_library::LocalLibrary;
use crate::variables::{platform, Platform, shared_library_extension, static_library_extension, target_directory};

pub mod types;

pub (crate) mod variables;
pub (crate) mod commands;

const LIBRARY_NAME_PREFIX: &str = "lib";

fn get_static_library_name(library_name: &str) -> String {
    // Omit the prefix for Windows since there is no convention for library names.
    if platform() == Platform::Windows {
        return library_name.to_string()
            .add(static_library_extension());
    }

    LIBRARY_NAME_PREFIX.to_string()
        .add(library_name)
        .add(static_library_extension())
}

fn get_shared_library_name(library_name: &str) -> String  {
    // Omit the prefix for Windows since there is no convention for library names.
    if platform() == Platform::Windows {
        return library_name.to_string()
            .add(shared_library_extension());
    }

    LIBRARY_NAME_PREFIX.to_string()
        .add(library_name)
        .add(shared_library_extension())
}

fn copy_shared_object(
    target_directory: &Path,
    library_path: &Path,
) {
    if !target_directory.exists() {
        fs::create_dir_all(target_directory)
            .expect("Could not create target directory.");
    }

    if !library_path.exists() {
        panic!("Could not find shared object: {:?}", library_path);
    }

    fs::copy(
        library_path,
        target_directory.join(library_path.file_name().unwrap())
    ).expect("Could not copy shared object.");
}

/// Trait for integrating a `LocalLibrary` into `cc::Build`.
pub trait BindBuild {

    /// Binds a `LocalLibrary` to the `cc::Build` instance.
    fn bind_library(
        &mut self,
        library: LocalLibrary
    ) -> &mut cc::Build;
}

impl BindBuild for cc::Build {

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

        self.includes(include_directories);

        let mut library_directories = library
            .get_library_directories()
            .clone();

        library_directories.dedup();
        library_directories.retain(|x| x.is_dir());

        for library_directory in library_directories.iter() {
            add_library_search_path(library_directory.as_path())
        }

        let mut link_targets = library
            .get_link_targets()
            .clone();

        link_targets.dedup();

        let target_directory = target_directory();

        // Always prefer static libraries over shared libraries
        for library in link_targets.iter() {
            for library_directory in library_directories.iter() {
                let static_library_path = library_directory
                    .join(get_static_library_name(library));

                let shared_library_path = library_directory
                    .join(get_shared_library_name(library));

                if static_library_path.exists() {
                    link_static_library(library);
                } else if shared_library_path.exists() {
                    // Copy shared object to target directory
                    copy_shared_object(
                        target_directory.as_path(),
                        shared_library_path.as_path()
                    );
                    link_shared_library(library);
                }
            }
        }

        // Link against any system libraries.
        let mut system_link_targets = library
            .get_system_link_targets()
            .clone();

        system_link_targets.dedup();

        for library in system_link_targets.iter() {
            link_shared_library(library);
        }

        self
    }
}
