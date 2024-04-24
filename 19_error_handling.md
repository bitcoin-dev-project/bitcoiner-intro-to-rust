# Error Handling

Until now, we've dealt with the `Result` type by simply calling `unwrap`. This will `panic!` and cause our program to crash anytime the `Result` is of the `Err` variant. Let's look a little more closely at what the `Result` type actually is.

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
*source: https://doc.rust-lang.org/std/result*

You may not be familiar with this as we haven't really used it in practice just yet, but this is an `Enum` type and not a `Struct` type that you're now familiar with. 

An `Enum` is different from a `Struct`. A `Struct` groups together related fields and data. An `Enum`, on the other hand, presents a set of possible values of which only one is used at a time. In the case of the `Result` type, there is an `Ok` variant and an `Err` variant. Each variant wraps a value.

We typically access the `Enum` variant values with a `match` statement or with the help of methods available on the particular `Enum`. Like `Structs`, `Enums` also have methods which are defined in an `impl` block. 

For example, if we look at the `unwrap` method, this actually returns the value wrapped in the `Ok` variant. However, if the variant returned is of the `Err` type, the program will panic and print the `&str` wrapped in that variant. 

```rust
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.unwrap(), 2);

...

let x: Result<u32, &str> = Err("emergency failure");
x.unwrap(); // panics with `emergency failure`
```
*source: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap*

So what is a better way to handle errors in our program? What we really want is for our functions to return `Result` types and then allow the functions calling them to better handle those `Result` variants and return a friendly error message to the user instead of crashing the program. 

Let's make some changes to our program. The first thing we're going to do is turn our program into a library. The `main` function in `main.rs` is the only function that does not support a return type. The purpose of the `main` function is to be the entrypoint into our program when we run `cargo run`. So what we want is for our `main.rs` to call our library and treat it as an external crate. This is not unlike the way we import external crates by adding them to our `Cargo.toml` and bring them into scope with `use` statements. 

For additional reading on understanding the differences between `lib.rs` and `main.rs` I found this [Stack Overflow question](https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs) to be helpful.

So let's start by creating a `lib.rs` and move all of our program logic into there. 

```console
$ tree
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs
    ├── main.rs
    └── transaction.rs

2 directories, 5 files
```

We'll rename the `main` function to `run` and this will return a `Result` enum. All other functions and statements will be the same except they are now moved into `lib.rs`.

Here is the modified `run` function signature along with the last line.

*lib.rs*
```rust
pub fn run(raw_transaction_hex: String) -> Result<String, dyn Error> {
    let transaction_bytes = hex::decode(raw_transaction_hex)?;
    ...
    
    let json = serde_json::to_string_pretty(&transaction)?;
    Ok(json)
}
```

Let's break this down a bit:
1. We've added `pub` at the front to ensure that this function is callable from the outside, such as from our `main.rs` file. This is the public entrypoint into our library. 
2. We're now taking the raw transaction hex string as an argument.
3. The function return type is a `Result` enum. The `Ok` variant must be a `String` type.
4. The `Err` variant is of the type `dyn Error`. The `dyn Error` keyword indicates a **trait object**. What this means is that the expected return type is *any* type that implements the [std `Error` trait](https://doc.rust-lang.org/std/error/trait.Error.html).
5. Instead of calling `unwrap` on the `hex::decode` and `to_string_pretty` methods, we are now appending the [question mark operator, `?`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator). This does something similar to `unwrap` in that it returns the value wrapped in the `Ok` variant. However, in the case of an error it will *return* the `Err` variant instead of panicking. In other words, in the case of an error, the function execution will stop and the `run` method will return this `Err` variant for our `Result` enum.
6. We wrap the the `json` string inside an `Ok` variant to match our function's return type. Remember, it is expecting an `Ok` or an `Err` variant with their wrapped values of specific types. We can't simply return the `String` type.

Let's update our `main.rs` file which is now currently empty:

```rust
fn main() {
    let raw_transaction = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    match transaction_decoder::run(raw_transaction.to_string()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
```

Let's review the changes:
1. The name of our library is `transaction_decoder`. This can be found in our `Cargo.toml` which is the same as the `package_name`, which might be different for you. We're calling the `run` method on our library and passing in the `raw_transaction` `String`. We have to convert the text in quotes to a `String` type with the `to_string` method. Remember, whenever we write text in quotes, Rust interprets as the `&str` type.
2. We use the `match` pattern to handle the `Result`. Note the use of [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) which prints to `io::stderr` instead of `io::stdout`. 

Let's attempt to build and run our program now with `cargo run`.

If you did this correctly, you should get one particular compiler error:

```console
error[E0277]: the size for values of type `(dyn StdError + 'static)` cannot be known at compilation time
   --> src/lib.rs:84:44
    |
84  | pub fn run(raw_transaction_hex: String) -> Result<String, dyn Error> {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `(dyn StdError + 'static)`
note: required by a bound in `Result`
   --> /Users/shaanbatra/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:502:20
    |
502 | pub enum Result<T, E> {
    |                    ^ required by this bound in `Result`

```

The `dyn Error` doesn't have a known size at compile time. Let's understand this a bit. 

Our `run` function might return many different types of errors. For example, if `hex::decode` fails, it returns an enum, [`FromHexError`](https://docs.rs/hex/latest/hex/enum.FromHexError.html). If `serde_json::to_string_pretty` fails, it returns a struct, [`serde_json::Error`](https://docs.rs/serde_json/latest/serde_json/struct.Error.html). 

Rust doesn't know which one and therefore doesn't know the size at compile time for the return type. Whenever we don't know the size of something at compile time, we need to allocate data to the heap and return a pointer reference. We can use `Box` to do this. Very simply, `Box` allocates data on the heap and then returns a pointer reference. By setting the return error type as a `Box`, all of our different errors in the code block will be implicitly wrapped in `Box` as well. This way, all of the different error types will have the same size since they will all be pointers. They still point to different places in memory and so Rust will determine how to handle them at runtime instead of compile time.

So let's fix this and modify the function signature by `Box`ing our errors: `pub fn run(raw_transaction_hex: String) -> Result<String, Box<dyn Error>>`.

If we run `cargo run` now, everything should work and print the same result! Great!

Let's test an error case. In our `main.rs`, we'll comment out the original transaction hex and replace it with the letters "abc". This should fail to `hex::decode` because it has an odd length (remember, every two hex characters is 1 byte). 

```rust
fn main() {
    // let raw_transaction = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let raw_transaction = "abc";
    match transaction_decoder::run(raw_transaction.to_string()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
```

Let's see what happens if we do that.

We'll get the following error printed to the terminal.

```console
Odd number of digits
```

First of all, this is a nicer error message. It's not a harsh program crash with unnecessary information about our code printed out. We do get an error here that makes sense, but there's not a lot of context is there? Clearly, our `hex::decode` method returned the `Err` result, but it would be nice to add some more context to this error message. We can do that with the help of the [`map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) method, which is available on the `Result` enum. 

*lib.rs*
```rust
...

    let transaction_bytes = hex::decode(raw_transaction_hex).map_err(|e| format!("Hex decoding error: {}", e))?;

...
```

The `map_err` method takes a closure which passes the error message as an argument. We then return a `String` which modifies the error message by adding some additional text in front. You'll notice that `format!` works similarly to `println!` in that it takes a formatting string which replaces the brackets `{}` with the arguments that follow. However, instead of printing, it simply returns a `String`. 

Let's run `cargo run` again and see how this looks:

```console
Hex decoding error: Odd number of digits
```

Pretty neat! Alright, let's finish up our error handling by returning a `Result` for any function that might `panic!`. All we have to do is modify our function signatures and replace any `unwrap` calls with a `?`. Then, anywhere those functions are called, we need to handle them with the `?` operator as well. This way, all errors will essentially *bubble* up to the user and won't cause our program to crash. 

Why don't you go ahead and make those changes? Check out the `code` folder of this course to compare your changes to mine. Make sure to update the unit tests as well! *Hint: You don't need to use `Box<dyn Error>` for every function. Some of them will return only one type of error and that can be determined at compile time.*

Next up, let's look at command line arguments and see how we can accept any input from the terminal. We no longer have to hardcode our transaction example in the code! 

<hr/>

<div>
    <p align="right"><a href="20_command_line_arguments.md">>>> Next Lesson: Command Line Arguments</a></p>
</div>
