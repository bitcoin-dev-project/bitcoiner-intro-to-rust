# Setup

Let's get started with our project.
First, you'll need to install Rust.
Some useful guides:
* https://doc.rust-lang.org/book/ch01-01-installation.html
* https://www.rust-lang.org/tools/install

*Note: this course reflects Rust version 1.75.0.
If you notice any discrepancies, check your Rust version and leave an issue on Github.*

After you have installed Rust and Cargo, you can set up a new rust project using `cargo new` followed by the name of the project.
Let's call this `transaction-decoder`.

```console
$ cargo new transaction-decoder
$ cd transaction-decoder
```

This will create a new application in the folder `transaction-decoder`:
```console
$ tree
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

The `Cargo.toml` file is one of the two metadata files that Cargo uses to manage things like the name, version and package dependencies.
It should look something like this:

```toml
[package]
name = "transaction-decoder"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

There is a `package` section which shows some package configuration options and a `dependencies` section which we'll use to manage any external libraries (or `crates`) that we'll want to use in our program.
Cargo uses this information to fetch and build package dependencies and keeps track of them in the `Cargo.lock` file. 

The next folder we'll look at is `src`, which contains the `main.rs` file.
This is the entry point of our Rust program.
We will call our program from the terminal using `$ cargo run`.
This will compile our program, create an executable file and then run it. 

Our `src/main.rs` file should look like this:
```rust
fn main() {
    println!("Hello, world!");
}
```

So if we run `$ cargo run` in the terminal (from the root directory of our project), we should see `Hello, world!` printed to the terminal.
We will also see in the output that cargo has compiled the program and run the executable file located in `target/debug/transaction-decoder`.

We're all set up!
So let's get started writing some Rust code!

### Additional Resources:
* A good overview of the `cargo build` and `cargo run` commands can be found here: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html.
* For more in-depth information on Cargo, check out the [online Cargo book](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html).

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="03_our_first_function.md">>>> Next Lesson: Our First Function</a></p>
</div>
