# bind-builder

Rust build dependency that helps you build and link native libraries.

### Requirements

 - `cmake` must be installed and available in the system path.
 - `git` if you wish to clone repositories.
 - `c`/`c++` build tools.

## Usage

Before you start, make sure that the cmake project you want to build has install targets setup
for the libraries you want to link against. See
[cmake documentation](https://cmake.org/cmake/help/latest/command/install.html) to learn more.

### Example

```rust
let project = CMakeBuilder::clone(
    "some-repo",
    "git@github.com:user/repo.git",
    "tag"
    )
    .generator("Ninja")
    .build();

let library = LocalLibrary::from(project)
    .link_target("some_library")
    .link_system_target("some_system_library")
    .get();

cxx_build::bridge("src/bindings.rs")
    .cpp(true)
    .static_flag(true)
    .std("c++20")
    .file("src/cpp_crate.cpp")
    .include(Path::new("src"))
    .bind_library(library)
    .compile("rust-cxx-testing");
```

If you are linking against shared libraries, and building for Linux or MacOS, you will need to
explicitly set the `@rpath` to contain the binaries current directory.

This can be done by adding the following to your final artifact's `build.rs`:

```rust
#[cfg(target_os="macos")]
println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

#[cfg(target_os="linux")]
println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
```
