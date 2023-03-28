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

### Visual Studio Code

- Install Rust via the [rustup installer](https://rustup.rs/) 
- Install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Create your first Rust program by typing `cargo new`
```rust
cargo new hello_world
```
- Build your Rust program by typing `cargo build`
```rust
cargo build
```
- Run your Rust program by typing `cargo run`
```rust
cargo run
```

### Intellij IDEA

Install the toolchain from this [link](https://rustup.rs/), it will give the setup for the
appropriate operating system.

#### Option 1 - configure project directly through Intellij IDEA 
Open Intellij IDEA, and press `Ctrl + Alt + S` to go to `IDE Settings -> Plugins`. From the `Marketplace`, select `Rust` and press `Install`. Then, press the `Restart IDE` button.

Go to `Projects -> New Project -> Rust`. In the setup settings, make sure that the `Toolchain` is set (should be done automatically). Otherwise, set `Toolchain location` to the `.cargo/bin` directory of where the toolchain was installed. 

The Rust standard library should set automatically, or a prompt should appear with the option to download it. Do so. 

Finally, select Project Template as `Binary`, and press `Next`. Select a project location and press `Create Project`.

#### Option 2 - configure project through command line, open with Intellij IDEA
Follow the command-line setup instructions above to create a project. Open Intellij IDEA, and press `Ctrl + Alt + S` to go to `IDE Settings -> Plugins`. From the `Marketplace`, select `Rust` and press `Install`. Then, press the `Restart IDE` button.

Once the IDE restarts, select `Open Existing Project` and select the rust project you created.

## Helpful Resources
  [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) is a very useful resource to learn more about the language.
