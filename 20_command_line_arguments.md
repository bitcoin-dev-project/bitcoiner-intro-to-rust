# Command Line Arguments

Up until now we've hardcoded the transaction hex in our `main()` function. Let's look into turning our program into a command line application that can accept any transaction hex from the user. We'll use the popular [clap crate](https://docs.rs/clap/latest/clap/index.html) to parse command line arguments. There's a great tutorial for getting started [here](https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html).

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

Let's look at what we're doing here more closely and then we'll walk through some examples from the command line:
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
    match transaction_decoder_20::run(transaction_decoder_20::get_arg()) {
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

Usage: transaction_decoder_19 <RAW_TRANSACTION>

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

Usage: transaction_decoder_19 <RAW_TRANSACTION>

For more information, try '--help'.
```

Ok, that's helpful. Let's pass in our raw transaction hex now:
```shell
$ cargo run -- 0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000
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