# bind-builder

Rust build dependency that helps you build and link native libraries.

### Note 

To view the output from the build script, run `cargo build -vv`.

To allow your binary to search for shared libraries in its directory, add the following to the binaries `build.rs`:

```rust
#[cfg(target_os="macos")]
println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

#[cfg(target_os="linux")]
println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
```

## Requirements

This build script assumes that your C++ project has install directories setup for each target you wish
to generate bindings for.

See [cmake documentation](https://cmake.org/cmake/help/latest/command/install.html) to learn more. 

### Dependencies

- git
- cmake
- ninja (recommended)
- a C++ compiler

## References

- https://github.com/dtolnay/cxx
- https://github.com/Rust-SDL2/rust-sdl2/tree/master