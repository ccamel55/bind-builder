use std::{env, fs};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use cmake::Config;
use crate::variables::{get_profile, target_directory};

fn cmake_executable() -> String {
    env::var("CMAKE")
        .unwrap_or_else(|_| String::from("cmake"))
}

/// Builder for cloning, configuring, building and installing a CMake project.
pub struct CMakeBuilder {
    name: String,
    cmake_config: Option<Config>,
    build_directory: Option<PathBuf>,
    install_directory: PathBuf,
    build_target: Option<String>
}

impl CMakeBuilder {

    /// Create a new `CMakeBuilder` from a git repository.
    ///
    /// This function uses the git command therefore it will inherit the git configuration and
    /// credentials from your system.
    pub fn clone(
        name: &str,
        url: &str,
        tag: &str,
    ) -> CMakeBuilder {

        let target_directory = target_directory();
        let clone_directory = target_directory.parent().unwrap()
            .join("git")
            .join(name);

        // Setup temp repository if it does not exist, instead of cloning we do this to
        // reduce the amount of stuff we have to pull.
        if !clone_directory.exists() {
            fs::create_dir_all(clone_directory.as_path())
                .expect("Could not create directory, does the path exist?");

            Command::new("git")
                .arg("init")
                .current_dir(clone_directory.as_path())
                .status()
                .expect("Could not init repo, is git installed?");

            Command::new("git")
                .arg("remote")
                .arg("add")
                .arg("origin")
                .arg(url)
                .current_dir(clone_directory.as_path())
                .status()
                .expect("Could not add remote, is git installed?");
        }

        Command::new("git")
            .arg("fetch")
            .arg("origin")
            .arg(tag)
            .current_dir(clone_directory.as_path())
            .status()
            .expect("Could not fetch repo, is git installed?");

        Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg(tag)
            .current_dir(clone_directory.as_path())
            .status()
            .expect("Could not checkout tag, is git installed?");

        Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .arg("--recursive")
            .current_dir(clone_directory.as_path())
            .status()
            .expect("Could not init submodules, is git installed?");

        CMakeBuilder::from(name, clone_directory.as_path())
    }

    /// Create a new `CMakeBuilder` from an existing cmake project.
    pub fn from(
        name: &str,
        path: &Path,
    ) -> CMakeBuilder {

        // Windows does not like canonicalize on some paths. It will result in cl.exe
        // failing to use the path.
        // https://github.com/rust-lang/rust/issues/42869
        // https://github.com/alexcrichton/cc-rs/issues/169
        let absolute_path = if cfg!(windows) {
            path.to_path_buf()
        } else {
            fs::canonicalize(path)
                .expect("Path not found, make sure the build directory exists.")
        };

        let configure_directory = absolute_path
            .join(format!("cmake-bind-builder-{}", get_profile().as_str()));

        let install_directory = configure_directory
            .join("install");

        let mut project = CMakeBuilder {
            name: name.to_string(),
            cmake_config: Some(Config::new(absolute_path)),
            build_directory: None,
            install_directory: install_directory.clone(),
            build_target: None
        };

        project.cmake_config.as_mut().unwrap().out_dir(configure_directory);
        project.cmake_config.as_mut().unwrap().define("CMAKE_SKIP_INSTALL_ALL_DEPENDENCY", "true");

        project
    }

    /// Create a new `CMakeBuilder` from an existing cmake build directory.
    pub fn from_build_directory(
        name: &str,
        build_path: &Path,
    ) -> CMakeBuilder {

        // Windows does not like canonicalize on some paths. It will result in cl.exe
        // failing to use the path.
        // https://github.com/rust-lang/rust/issues/42869
        // https://github.com/alexcrichton/cc-rs/issues/169
        let absolute_path = if cfg!(windows) {
            build_path.to_path_buf()
        } else {
            fs::canonicalize(build_path)
                .expect("Path not found, make sure the build directory exists.")
        };

        let install_directory = absolute_path
            .join(format!("cmake-bind-builder-{}", get_profile().as_str()))
            .join("install");

        let project = CMakeBuilder {
            name: name.to_string(),
            cmake_config: None,
            build_directory: Some(absolute_path),
            install_directory: install_directory.clone(),
            build_target: None
        };

        project
    }

    /// Sets the build-tool generator (`-G`) for this compilation.
    ///
    /// If unset, this crate will use the `CMAKE_GENERATOR` environment variable
    /// if set. Otherwise, it will guess the best generator to use based on the
    /// build target.
    pub fn generator<T: AsRef<OsStr>>(&mut self, generator: T) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.generator(generator);
        }

        self
    }

    /// Sets the toolset name (-T) if supported by generator.
    /// Can be used to compile with CLang/LLV instead of msvc when Visual Studio generator is selected.
    ///
    /// If unset, will use the default toolset of the selected generator.
    pub fn generator_toolset<T: AsRef<OsStr>>(&mut self, toolset_name: T) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.generator_toolset(toolset_name);
        }

        self
    }

    /// Adds a custom flag to pass down to the C compiler, supplementing those
    /// that this library already passes.
    pub fn cflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.cflag(flag);
        }

        self
    }

    /// Adds a custom flag to pass down to the C++ compiler, supplementing those
    /// that this library already passes.
    pub fn cxxflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.cxxflag(flag);
        }

        self
    }

    /// Adds a custom flag to pass down to the ASM compiler, supplementing those
    /// that this library already passes.
    pub fn asmflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.asmflag(flag);
        }

        self
    }

    /// Adds a new `-D` flag to pass to cmake during the generation step.
    pub fn define<K, V>(&mut self, k: K, v: V) -> &mut CMakeBuilder
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
    {
        if let Some(config) = self.cmake_config.as_mut() {
            config.define(k, v);
        }

        self
    }

    /// Registers a dependency for this compilation on the native library built
    /// by Cargo previously.
    ///
    /// This registration will modify the `CMAKE_PREFIX_PATH` environment
    /// variable for the build system generation step.
    pub fn register_dep(&mut self, dep: &str) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.register_dep(dep);
        }

        self
    }

    /// Sets the target triple for this compilation.
    ///
    /// This is automatically scraped from `$TARGET` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn target(&mut self, target: &str) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.target(target);
        }

        self
    }

    /// Sets the host triple for this compilation.
    ///
    /// This is automatically scraped from `$HOST` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn host(&mut self, host: &str) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.host(host);
        }

        self
    }

    /// Sets the `CMAKE_BUILD_TYPE=build_type` variable.
    ///
    /// By default, this value is automatically inferred from Rust's compilation
    /// profile as follows:
    ///
    /// * if `opt-level=0` then `CMAKE_BUILD_TYPE=Debug`,
    /// * if `opt-level={1,2,3}` and:
    ///   * `debug=false` then `CMAKE_BUILD_TYPE=Release`
    ///   * otherwise `CMAKE_BUILD_TYPE=RelWithDebInfo`
    /// * if `opt-level={s,z}` then `CMAKE_BUILD_TYPE=MinSizeRel`
    pub fn profile(&mut self, profile: &str) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.profile(profile);
        }

        self
    }

    /// Configures whether the /MT flag or the /MD flag will be passed to msvc build tools.
    ///
    /// This option defaults to `false`, and affect only msvc targets.
    pub fn static_crt(&mut self, static_crt: bool) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.static_crt(static_crt);
        }

        self
    }

    /// Add an argument to the `cmake` configure step
    pub fn configure_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.configure_arg(arg);
        }

        self
    }

    /// Add an argument to the final `cmake` build step
    pub fn build_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.build_arg(arg);
        }

        self
    }

    /// Configure an environment variable for the `cmake` processes spawned by
    /// this crate in the `build` step.
    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut CMakeBuilder
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
    {
        if let Some(config) = self.cmake_config.as_mut() {
            config.env(key, value);
        }

        self
    }

    /// Forces CMake to always run before building the custom target.
    ///
    /// In some cases, when you have a big project, you can disable
    /// subsequents runs of cmake to make `cargo build` faster.
    pub fn always_configure(&mut self, always_configure: bool) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.always_configure(always_configure);
        }

        self
    }

    /// Sets very verbose output.
    pub fn very_verbose(&mut self, value: bool) -> &mut CMakeBuilder {
        if let Some(config) = self.cmake_config.as_mut() {
            config.very_verbose(value);
        }

        self
    }

    /// Specify the build target for the final `cmake` build step, this will
    /// default to all.
    pub fn build_target(
        &mut self,
        target: &str
    ) -> &mut CMakeBuilder {
        self.build_target = Some(target.to_string());
        self
    }

    /// Run this configuration, compiling the library with all the configured
    /// options.
    ///
    /// This will run both the build system generator command and the
    /// command to build the library.
    pub fn build(&mut self) -> CMakeBuilder {

        let build_directory = match self.cmake_config.as_mut() {
            Some(config) => {
                config.build_target(
                    self.build_target.clone().unwrap_or("all".to_string()).as_str()
                )
                    // We also need to set CMAKE_INSTALL_PREFIX while building otherwise the
                    // cmake crate will default and override with an incorrect path.
                    .define("CMAKE_INSTALL_PREFIX", self.install_directory.clone().to_str().unwrap())

                    .build()
                    .join("build")
            },
            None => {
                self.build_directory.clone()
                    .expect("Could not find build directory argument, is it set?")
            }
        };

        Command::new(cmake_executable())
            // Actual install command
            .arg("--install")
            .arg(".")

            .arg("--prefix")
            .arg(self.install_directory.clone().to_str().unwrap())

            .current_dir(build_directory.clone())
            .status()
            .expect("Could not install repo, is cmake installed?");

        // Make a new object. Since we can't clone/copy cmake::Config :(
        let name = self.name.clone();
        let install_directory = self.install_directory.clone();
        let build_target = self.build_target.clone();

        CMakeBuilder {
            name,
            cmake_config: None,
            build_directory: Some(build_directory.clone()),
            install_directory,
            build_target
        }
    }

    pub (crate) fn get_install_directory(&self) -> &PathBuf {
        &self.install_directory
    }

    pub (crate) fn get_build_target(&self) -> &Option<String> { &self.build_target }
}