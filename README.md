# Port of the Processing Library for C

Processing_rs is inspired by the creative coding library [Processing](https://processing.org/).

Despite the name (which I should probably change), this library is not meant to be a prot of processing for Rust. It serves to provide a port of processing to C (mainly for my own enjoyment, and learning purpose).

Currently, it doesn't work. Mainly because I'm dumb, but also because I'm learning graphics programming with WebGPU.

# Why Rust?
I wrote this in the to be the cool kid on the block, and also because I hate C++. (But seriously) I feel Rust provides a better and safer way of writing code and provides a bunch of crates which I can use cuz I'm lazy.

This project uses the following crates:
- [wgpu](https://crates.io/crates/wgpu): For using the WebGPU API.
- [winit](https://crates.io/crates/winit): For cross-platform window creation.
- [env_logger](https://crates.io/crates/env_logger): For better logging of errors
- [pollster](https://crates.io/crates/pollster): For running the winit event loop in a blocking way

# How to run
1. Clone the repo and compile it using `cargo build --release`.
2. Copy the libprocessing.a file from `target/release` to your desired folder.
3. Copy the bindings.h file from `ffi-test` to your desired folder.
4. Ask your C compiler to link `libprocessing.a` by adding -lprocessing as a flag. Alternatively, look for a example CmakeLists.txt file in the ffi-test folder.
5. Include the `bindings.h` file in your C executable. Compile and run!

Note: Currently, I've only tested this library on Linux. To work, following libraries are also needed in addition to libprocessing:
- m (generally comes installed along with gcc)
- [freetype](https://stackoverflow.com/questions/21216129/install-gd-library-and-freetype-on-linux)
- [fontconfig](https://installati.one/install-fontconfig-ubuntu-22-04/)

# Planned features
 - [ ] Get basic rendering working ---> I'm here
 - [ ] Draw basic shapes like triangle, rectangle, circle, etc.
 - [ ] Events like click, hover, etc.
 - [ ] Draw curves like bezier, etc.
 - [ ] Math utilities like vectors, matrices, etc. 
 - [ ] Showing images (local and from the web)
 - [ ] 3D shit like translations, lighting, etc.