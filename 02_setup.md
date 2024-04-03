# Setup

Let's get started with our project. After you have installed Rust and Cargo, you can set up a new rust project using `cargo new` followed by the name of the project. Let's call this `bitcoin-transaction-decoder`. 

`$ cargo new bitcoin-transaction-decoder`

This will create a new application under the folder `bitcoin-transaction-decoder` and will create two files in there:
```rust
Cargo.toml
src
```

The `Cargo.toml` file is one of the two metadata files that Cargo uses to manage things like the name, version and package dependencies. It should look something like this:

```rust
[package]
name = "bitcoin-transaction-decoder"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

There is a `package` section which shows some package configuration options and a `dependencies` which we'll use to manage any external libraries (or `crates`) that we'll want to use in our program. Cargo will use this to fetch and build package dependencies and keep track of them in the `Cargo.lock` file. For in-depth information on Cargo, check out the online Cargo book: https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html.

The next folder we'll look at is `src`, which contains the `main.rs` file. This is our entry point into our Rust program. We will call our program from the terminal using `$ cargo run` which will compile our program and run the resulting executable file. A good brief overview of the `cargo build` and `cargo run` commands can be found here: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html.

Our `src/main.rs` file should look like this:
```rust
fn main() {
    println!("Hello, world!");
}
```

So if we run `$ cargo run` from the terminal (from the root directory of our project), we should see `Hello, world!` printed to the terminal. We will also see in the output that it has compiled the program and run the executable file located in `target/debug/bitcoin-transaction-decoder`.

We're all set up! So let's get started writing some Rust code!

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="03_our_first_function.md">>>> Next Lesson: Our First Function</a></p>
</div>
