# Error Handling

Until now, we've dealt with the `Result` type by simply calling `unwrap`. This will `panic!` and cause our program to crash anytime the `Result` is of the `Err` type. Let's look a little more closely at what the `Result` type actually is.

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

So let's start by creating a `lib.rs` and move all of our program logic into there. We'll rename the `main` function to `run` and this will return a `Result` enum. All other functions nad statements will be the same except they are now moved into `lib.rs`.

Here is the modified `run` function signature along with the last line. Everything else in the function will remain the same (as represented by the `...`):

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
4. The `Err` variant is of the type `dyn Error`. The `dyn Error` keyword indicates a **trait object**. What this means is that the expected return type is *any* type that implements the `Error` trait.
5. Instead of calling `unwrap` on the `hex::decode` and `to_string_pretty` methods, we are now appending the question mark operator, `?`. This does something similar to `unwrap` in that it unwraps the value wrapped in the `Ok` variant OR it will *return* the `Err` variant. In other words, in the case of an error the function execution will stop and the `run` method will return this `Err` variant. More info on the question mark operator [here](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator).
6. We wrap the the `json` string inside an `Ok` variant to match our function's return type. Remember, it is expecting an `Ok` or an `Err` variant with their wrapped values of specific types. We can't simply return the `String` type.

Let's update our `main.rs` file which is now currently empty:

```rust
fn main() {
    let raw_transaction = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    match transaction_decoder_19::run(raw_transaction.to_string()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
```

Let's review the changes:
1. The name of our library is `transaction_decoder_19`. This can be found in our `Cargo.toml` which is the same as the `package_name`, which might be different for you. We're calling the `run` method on our library and passing in the `raw_transaction` String. We have to convert the text in quotes to a `String` type with the `to_string` method. Remember, whenever we write text in quotes, Rust interprets as the `&str` type.
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

The `dyn Error` doesn't have a known size at compile time. Our `run` function might return multiple different types of errors. For example, if `hex::decode` fails, it returns an enum, `FromHexError`. If `serde_json::to_string_pretty` fails, it returns a struct, `serde_json::Error`. Rust doesn't know which one and therefore doesn't know the size at compile time for the return type. Whenever we don't know the size of something at compile time, we need to allocate data to the heap and return a pointer reference. We can use `Box` to do this. Very simply, `Box` allocates data on the heap and then returns a pointer reference. By setting the return error type as a `Box`, all of our different errors in the code block will be wrapped in `Box`. This way, all of the different error types will have the same size. They still point to different places in memory and so Rust will determine how to handle them at runtime instead of compile time.

So let's fix this and modify the function signature by `Box`ing our errors: `pub fn run(raw_transaction_hex: String) -> Result<String, Box<dyn Error>>`.

If we run `cargo run` now, everything should work and print the same result! Great!

Let's test an error case. In our `main.rs`, we'll comment out the original transaction hex and replace it with the letters "abc". This should fail to `hex::decode` because it has an odd length (remember, every two hex characters is 1 byte). 

```rust
fn main() {
    // let raw_transaction = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let raw_transaction = "abc";
    match transaction_decoder_18::run(raw_transaction.to_string()) {
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

First of all, this is a nicer error message. It's not a harsh program crash with unnecessary information about our code printed out. We do get an error here that makes sense, but there's not a lot of context. Clearly, `hex::decode` returned the `Err` result, but it would nice to add some more context to this error message. We can do that with the help of the [`map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) method, which is available on the `Result` enum. 

*lib.rs*
```rust
...

let transaction_bytes = hex::decode(raw_transaction_hex).map_err(|e| format!("Hex decoding error: {}", e))?;

...
```

The `map_err` method takes a function which passes the error message as an argument. We then return a `String` which modifies the error message by adding some additional text in front. 

Let's run `cargo run` again and see how this looks:

```console
Hex decoding error: Odd number of digits
```

Pretty neat! Alright, let's finish up our error handling by returning a `Result` for any function that might `panic!`. All we have to do is modify our function signatures and replace any `unwrap` calls with a `?`. Then, anywhere those functions are called, we need to handle them with the `?` operator as well. This way, all errors will essentially *bubble* up to the user and won't cause our program to crash. 

Why don't you go ahead and make those changes? Check out the `code` folder of this course to compare your changes to mine. Make sure to update the unit tests as well! *Hint: You don't need to use `Box<dyn Error>` for every function. Some of them will return only one type of error and that can be determined at compile time.*

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="20_command_line_arguments.md">>>> Next Lesson: Command Line Arguments</a></p>
</div>
