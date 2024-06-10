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

pub struct CMakeBuilder {
    name: String,
    cmake_config: Config,

    install_directory: PathBuf,
    build_target: Option<String>
}

impl CMakeBuilder {

    pub fn clone(
        name: &str,
        url: &str,
        tag: &str,
    ) -> CMakeBuilder {

        let target_directory = target_directory();
        let clone_directory = target_directory.parent().unwrap()
            .join("git")
            .join(name);

        if clone_directory.exists() {
            Command::new("git")
                .arg("checkout").arg("-f").arg(tag)
                .current_dir(clone_directory.as_path())
                .status()
                .expect("Could not checkout tag, is git installed?");
        } else {
            fs::create_dir_all(clone_directory.as_path())
                .expect("Could not create directory, does the path exist?");

            Command::new("git")
                .arg("clone").arg(url)
                .arg("--branch").arg(tag)
                .arg("--recurse").arg(".")
                .current_dir(clone_directory.as_path())
                .status()
                .expect("Could not clone repo, is git installed?");
        }

        CMakeBuilder::from(name, clone_directory.as_path())
    }

    pub fn from(
        name: &str,
        path: &Path,
    ) -> CMakeBuilder {

        let absolute_path = fs::canonicalize(path)
            .expect("Path not found, is repo cloned?");

        let configure_directory = absolute_path
            .join(format!("cmake-bind-builder-{}", get_profile().as_str()));

        let install_directory = configure_directory
            .join("install");

        let mut project = CMakeBuilder {
            name: name.to_string(),
            cmake_config: Config::new(absolute_path),

            install_directory: install_directory.clone(),
            build_target: None
        };

        project.cmake_config.out_dir(configure_directory);

        // Allow some module to not be build before installing and install to our target directory
        project.cmake_config.define("CMAKE_SKIP_INSTALL_ALL_DEPENDENCY", "true");
        project.cmake_config.define("CMAKE_INSTALL_PREFIX", install_directory.to_str().unwrap());

        project
    }

    /// Sets the build-tool generator (`-G`) for this compilation.
    ///
    /// If unset, this crate will use the `CMAKE_GENERATOR` environment variable
    /// if set. Otherwise, it will guess the best generator to use based on the
    /// build target.
    pub fn generator<T: AsRef<OsStr>>(&mut self, generator: T) -> &mut CMakeBuilder {
        self.cmake_config.generator(generator);
        self
    }

    /// Sets the toolset name (-T) if supported by generator.
    /// Can be used to compile with CLang/LLV instead of msvc when Visual Studio generator is selected.
    ///
    /// If unset, will use the default toolset of the selected generator.
    pub fn generator_toolset<T: AsRef<OsStr>>(&mut self, toolset_name: T) -> &mut CMakeBuilder {
        self.cmake_config.generator_toolset(toolset_name);
        self
    }

    /// Adds a custom flag to pass down to the C compiler, supplementing those
    /// that this library already passes.
    pub fn cflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        self.cmake_config.cflag(flag);
        self
    }

    /// Adds a custom flag to pass down to the C++ compiler, supplementing those
    /// that this library already passes.
    pub fn cxxflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        self.cmake_config.cxxflag(flag);
        self
    }

    /// Adds a custom flag to pass down to the ASM compiler, supplementing those
    /// that this library already passes.
    pub fn asmflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut CMakeBuilder {
        self.cmake_config.asmflag(flag);
        self
    }

    /// Adds a new `-D` flag to pass to cmake during the generation step.
    pub fn define<K, V>(&mut self, k: K, v: V) -> &mut CMakeBuilder
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
    {
        self.cmake_config.define(k, v);
        self
    }

    /// Registers a dependency for this compilation on the native library built
    /// by Cargo previously.
    ///
    /// This registration will modify the `CMAKE_PREFIX_PATH` environment
    /// variable for the build system generation step.
    pub fn register_dep(&mut self, dep: &str) -> &mut CMakeBuilder {
        self.cmake_config.register_dep(dep);
        self
    }

    /// Sets the target triple for this compilation.
    ///
    /// This is automatically scraped from `$TARGET` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn target(&mut self, target: &str) -> &mut CMakeBuilder {
        self.cmake_config.target(target);
        self
    }

    /// Sets the host triple for this compilation.
    ///
    /// This is automatically scraped from `$HOST` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn host(&mut self, host: &str) -> &mut CMakeBuilder {
        self.cmake_config.host(host);
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
        self.cmake_config.profile(profile);
        self
    }

    /// Configures whether the /MT flag or the /MD flag will be passed to msvc build tools.
    ///
    /// This option defaults to `false`, and affect only msvc targets.
    pub fn static_crt(&mut self, static_crt: bool) -> &mut CMakeBuilder {
        self.cmake_config.static_crt(static_crt);
        self
    }

    /// Add an argument to the `cmake` configure step
    pub fn configure_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut CMakeBuilder {
        self.cmake_config.configure_arg(arg);
        self
    }

    /// Add an argument to the final `cmake` build step
    pub fn build_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut CMakeBuilder {
        self.cmake_config.build_arg(arg);
        self
    }

    /// Configure an environment variable for the `cmake` processes spawned by
    /// this crate in the `build` step.
    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut CMakeBuilder
        where
            K: AsRef<OsStr>,
            V: AsRef<OsStr>,
    {
        self.cmake_config.env(key, value);
        self
    }

    /// Forces CMake to always run before building the custom target.
    ///
    /// In some cases, when you have a big project, you can disable
    /// subsequents runs of cmake to make `cargo build` faster.
    pub fn always_configure(&mut self, always_configure: bool) -> &mut CMakeBuilder {
        self.cmake_config.always_configure(always_configure);
        self
    }

    /// Sets very verbose output.
    pub fn very_verbose(&mut self, value: bool) -> &mut CMakeBuilder {
        self.cmake_config.very_verbose(value);
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

        self.cmake_config.build_target(
            self.build_target.clone().unwrap_or("all".to_string()).as_str()
        );

        // Build and install by calling install command our selves
        let build_directory = self.cmake_config
            .build()
            .join("build");

        Command::new(cmake_executable())
            .arg("--install")
            .arg(".")
            .current_dir(build_directory)
            .status()
            .expect("Could not install repo, is cmake installed?");

        // Make a new object. Since we can't clone/copy cmake::Config :(
        let name = self.name.clone();
        let install_directory = self.install_directory.clone();
        let build_target = self.build_target.clone();

        CMakeBuilder {
            name,
            cmake_config: Config::new(""),

            install_directory,
            build_target
        }
    }

    pub (crate) fn get_install_directory(&self) -> &PathBuf {
        &self.install_directory
    }

    pub (crate) fn get_build_target(&self) -> &Option<String> { &self.build_target }
}