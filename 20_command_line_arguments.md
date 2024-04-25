# Command Line Arguments

Up until now we've hardcoded the transaction hex in our `main()` function. Let's now turn our program into a command line application that can accept any transaction hex from the user.

Our goal is to do something like the following, which will return the decoded transaction:

```shell
$ cargo run -- 010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000
```

We'll use the popular [clap crate](https://docs.rs/clap/latest/clap/index.html) to parse command line arguments. There's a good tutorial for getting started [here](https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html).

First, we'll add `clap` to our `Cargo.toml`:
```toml
[package]
name = "transaction_decoder_20"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.115"
sha2 = "0.10.8"
clap = "4.5.4"
```

Next, we'll bring the relevant modules into scope in our *lib.rs* file with `use clap::{arg, value_parser, Command};`. Let's now add a function for parsing the user-provided command line arguments:

*lib.rs*
```rust
pub fn get_arg() -> String {
    let matches = Command::new("Bitcoin Transaction Decoder")
        .version("1.0")
        .about("Decodes a raw transaction")
        .arg(
            arg!([RAW_TRANSACTION])
                .value_parser(value_parser!(String))
                .required(true)
        )
        .get_matches();

    matches
        .get_one::<String>("RAW_TRANSACTION")
        .cloned()
        .expect("raw transaction is required")
}
```

Let's look at what we're doing here more closely and then we'll walk through some examples with the command line:
1. We're using the `Command` builder to set up our command line program. We can chain different methods to provide more information about our program and what arguments it expects.
2. We establish the `RAW_TRANSACTION` argument with the provided `arg!` macro. This will be a required argument so we set `.required(true)` and it will be parsed as a string. If we wanted to, we could set up additional command line arguments and flags here.
3. When we call `.get_matches()`, this will return a struct [`ArgMatches`](https://docs.rs/clap/latest/clap/struct.ArgMatches.html), which is a container for the parsed results. If there are any errors when `.get_matches` is called, such as if a required argument has not been provided, this method will exit the program and print out the error. 
4. In order to access the parsed `String` from our `ArgMatches`, we call the `get_one` method on the field we're looking for, which we've named `RAW_TRANSACTION`. This returns an [`Option<&T>`](https://docs.rs/clap/latest/clap/struct.ArgMatches.html#method.get_one), which in this case is an `Option<&String>`. An `Option` is an enum with two variants, a `Some(value)` or `None`. So there's either a value contained in `Some` or no value at all.
5. The Option wraps a `&String`, but what we want is a `String` and not `&String`. So we can leverage the `Option` enum's [`cloned` method](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#method.cloned) to convert this option to an `Option<String>`.
6. Lastly, we call `.expect` on our `Option`. This will return the contained `Some` value and panic if it's `None`. We normally don't want to panic, but in this case it makes sense to do so. Technically the `Option` should always be the `Some` variant at this point unless something very unexpected has happened. `get_matches` will fail and print an error message if no argument is provided.

Next we simply need to update our `main()` function in `main.rs` to call this public method and pass in the result to our `run` method:

*main.rs*
```rust
fn main() {
    match transaction_decoder::run(transaction_decoder::get_arg()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
```

Pretty straightforward, right? Let's test this out from the command line:

```shell
$ cargo run -- -help
```

We can run our command line program by typing `cargo run` followed by two dashes, `--` a space and then either our RAW_TRANSACTION argument or a built-in named argument. `-help` is built-in with our `clap` setup and provides basic information about our command line program.

You should see something like this:

```shell
Decodes a raw transaction

Usage: transaction_decoder <RAW_TRANSACTION>

Arguments:
  <RAW_TRANSACTION>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```

So, `--help` and `--version` are two named, optional arguments we have available. There is a required positional argument called `RAW_TRANSACTION`. Let's see what happens if we don't pass in any arguments:

```shell
$ cargo run --
```

We should get an error:
```shell
error: the following required arguments were not provided:
  <RAW_TRANSACTION>

Usage: transaction_decoder <RAW_TRANSACTION>

For more information, try '--help'.
```

Ok, that's helpful. Let's pass in our raw transaction hex now:
```shell
$ cargo run -- 010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000
```

And that works! It prints out our decoded transaction. Nice!

What happens if we pass in a bad hex string:
```shell
$ cargo run -- abc
```

We'll get the expected error message:
```shell
Hex decoding error: Odd number of digits
```

Pretty cool! You now have a command line program. Take some time to go through the `clap` documentation and the different tutorials to familiarize yourself with all of the functionality available. 

<hr/>

<div>
    <p align="right"><a href="21_refactoring_and_rust_bitcoin.md">>>> Next Lesson: Refactoring and the Rust-Bitcoin Library</a></p>
</div>
