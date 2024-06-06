# bind-builder

Rust build dependency that helps you build cxx bindings.

**Very WIP**

## Requirements

This build script assumes that your C++ project has install directories setup for each target you wish
to generate bindings for.

See [cmake documentation](https://cmake.org/cmake/help/latest/command/install.html) to learn more. 

### Dependencies

- git
- cmake
- ninja
- a C++ compiler

## Todo:

- Handle repo's properly (pull, don't just clone each time)
- Support dynamic libraries (link against them, copy them with binary)

## References

- https://github.com/dtolnay/cxx
- https://github.com/Rust-SDL2/rust-sdl2/tree/master