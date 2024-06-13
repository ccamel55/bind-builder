use std::path::{Path, PathBuf};
use crate::types::cmake_builder::CMakeBuilder;

const DEFAULT_LIBRARY_DIRECTORIES: [&str; 2] = [
    "lib",
    "lib64",
];

const DEFAULT_INCLUDE_DIRECTORIES: [&str; 1] = [
    "include",
];

/// Local library configuration.
///
/// This contains all the information required to link against a local library.
#[derive(Clone)]
pub struct LocalLibrary {
    install_directory: PathBuf,

    link_targets: Vec<String>,
    system_link_targets: Vec<String>,

    include_directories: Vec<PathBuf>,
    library_directories: Vec<PathBuf>,
}

impl LocalLibrary {

    /// Create a new `LocalLibrary` instance from a specific path.
    ///
    /// This is useful when you want to ship binaries with your crate.
    pub fn new(install_directory: &Path) -> LocalLibrary {

        let mut local_library = LocalLibrary {
            install_directory: install_directory.into(),

            link_targets: Vec::new(),
            system_link_targets: Vec::new(),

            include_directories: Vec::new(),
            library_directories: Vec::new(),
        };

        // Add default include and library directories.
        for include_directory in DEFAULT_INCLUDE_DIRECTORIES {
            local_library.add_include_directory(Path::new(include_directory));
        }

        for library_directory in DEFAULT_LIBRARY_DIRECTORIES {
            local_library.add_library_directory(Path::new(library_directory));
        }

        local_library
    }

    /// Create a new `LocalLibrary` instance from a `CMakeBuilder`.
    pub fn from(
        repository: CMakeBuilder,
    ) -> LocalLibrary {

        let install_directory = match repository.get_install_directory().exists() {
            true => repository.get_install_directory(),
            false => panic!("Could not find install directory, is repository built?")
        };

        let build_target = repository.get_build_target().clone().unwrap_or("all".to_string());
        let mut local_library = LocalLibrary::new(install_directory);

        if build_target.to_lowercase() != "all"{
            local_library.link_target(build_target.as_str());
        }

        local_library
    }

    /// Add a directory that will be searched for include files.
    ///
    /// The path should be relative to the installation directory.
    pub fn add_include_directory(
        &mut self,
        path: &Path,
    ) -> &mut LocalLibrary {

        // Check include directory exists
        let include_directory = self.install_directory.join(path);
        if include_directory.exists() && include_directory.is_dir() {
            self.include_directories.push(include_directory)
        }

        self
    }

    /// Add a directory that will be searched for library files.
    ///
    /// The path should be relative to the installation directory.
    pub fn add_library_directory(
        &mut self,
        path: &Path,
    ) -> &mut LocalLibrary {

        // Check library directory exists
        let library_directory = self.install_directory.join(path);
        if library_directory.exists() && library_directory.is_dir() {
            self.library_directories.push(library_directory)
        }

        self
    }

    /// Add a target to link against.
    ///
    /// Before linking, the crate will check if the library exists. If it finds a static and shared
    /// library with the same name, it will always prefer the static library.
    ///
    /// When linking against a shared library, the shared object will be copied to the target
    /// directory.
    pub fn link_target(
        &mut self,
        target: &str,
    ) -> &mut LocalLibrary {
        self.link_targets.push(target.to_string());
        self
    }

    /// Add a system target to link against.
    ///
    /// Unlike `link_target`, this will not check if the library exists and will always assume that
    /// the library is shared and available on the system.
    ///
    /// system link targets will not be copied to the target directory.
    pub fn link_system_target(
        &mut self,
        target: &str,
    ) -> &mut LocalLibrary {
        self.system_link_targets.push(target.to_string());
        self
    }

    /// Finalize the `LocalLibrary` configuration.
    pub fn get(&self) -> LocalLibrary {
        self.clone()
    }

    pub (crate) fn get_link_targets(&self) -> &Vec<String> {
        &self.link_targets
    }

    pub (crate) fn get_system_link_targets(&self) -> &Vec<String> {
        &self.system_link_targets
    }

    pub (crate) fn get_include_directories(&self) -> &Vec<PathBuf> {
        &self.include_directories
    }

    pub (crate) fn get_library_directories(&self) -> &Vec<PathBuf> {
        &self.library_directories
    }
}