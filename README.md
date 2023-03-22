# Rust tutorial for the Seminar: Programming languages and Paradigms

## Setup

### Command-Line

Install the toolchain from this [link](https://rustup.rs/), it will give the setup for the
appropriate operating system.

Afterwards projects can be creating with `cargo init`, if given a name then a directory will be
created as well.

With `cargo run` from within the project you can execute the current application. `cargo build` only
builds the application, to run it, you need to run it manually `./target/debug/[binary name]`.

Adding `--release` flag to `build` or `run` creates an optmized version of the binary
