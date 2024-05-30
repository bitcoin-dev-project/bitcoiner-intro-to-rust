# Vectors and the Result Enum

Let's apply what we've learned and update the `read_version` function:

```rust
fn read_version(transaction_hex: &str) -> u32 {
    return 1;
}
```

There are two different ways we can approach this: 
1. Read the first 4 pairs of characters and do some base math to determine the u32 integer.
2. Convert the transaction into a collection of bytes first and then work with the methods available for that type.

Now that we have a basic understanding of hexadecimal format and bytes, we're going to take approach #2 and leverage an external library.
It will be easier and more efficient to work with an array of `u8` integers to traverse the byte sequence, rather than read from a string and do unnecessary math.
The library can handle the conversion for us.
It will also be easier to work with as we decode the rest of the transaction.

The first thing we want to do is convert our hexadecimal string into a collection of bytes.
We'll use the popular [`hex` crate](https://docs.rs/hex/latest/hex/).

Let's add a dependency to our `Cargo.toml` file.
Note the `hex` crate set to version `0.4` at the bottom under the `[dependencies]` section:

```toml
[package]
name = "bitcoin-transaction-decoder"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4"
```

Now, the hex library's top-level module `hex` should be available to use in our `main.rs` file.
*Note: As we'll see in future sections of this course, if we want to use submodules of a crate, we'll have to import them with a `use` statement.*

If we look through the documentation at https://docs.rs/hex/latest/hex/, we see that we can convert the hex to bytes by calling the `decode` method from the `hex` module like so:

```rust
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex);
    1 // no return needed as the last expression without a semi-colon is automatically returned
}
```

We'll keep returning `1` at the bottom of the function for now so the compiler doesn't complain a `u32` wasn't returned.
*Note: Rust will simply return the last expression without a semi-colon so the `return` keyword is not needed.*
Let's run this now and see what happens.
Execute `$ cargo run` from the terminal.

So far, so good.
That should compile fine.
Let's now get the first 4 bytes from the returned collection.
The returned data is a `vec` - short for Vector - which is something like a `list` in Python or an array in javascript.
Of course, it's more nuanced in Rust.
We'll dive deeper into some of the differences in the next few lessons.
With a `vec` we can grab the first 4 items by doing something like `vec[0..4]` where `0..4` represents a range from 0 to 4, not including 4.
This might be a pattern you're already familiar with.

So let's add that line.

```rust
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex);
    let version_bytes = transaction_bytes[0..4];
    1 // no return needed as the last expression without a semi-colon is automatically returned
}
```

What happens when we execute `$ cargo run`?

Well, we get an error.
Take some time to read through it.
You should see something like the following:

```console
error[E0608]: cannot index into a value of type `Result<Vec<u8>, FromHexError>`
```

Let's examine what `hex::decode` returns.
[Here is the doc](https://docs.rs/hex/latest/hex/fn.decode.html) for the `decode` method.
Remember, we want to work with a Vector of bytes so a `vec<u8>` is the data type we're looking for.
However, if we look at the return type of the `decode` function we see that the data structure we want is wrapped *inside* of a `Result` type.

The `Result` type is a common `enum` that you will see in Rust code.
Enums are a way to describe a mutually exclusive set of values for a particular variable.
If you think about it, the `hex::decode` function can fail to return a proper collection of bytes.
For example, what if one of the characters is not a hex character?
So we get two possibilities from a `Result`, an `Ok` response or an `Err` response.
The former represents a successful computation whereas the latter indicates an error occurred.

There are a few different ways to work with an enum.
Every `Result` enum has an [`unwrap`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) method that you can call.
This will return the underlying data type if the result is an `Ok` type or it will `panic` and the program will crash if the result is an `Err` type.
Crashing your program is probably not the best way to handle an error, unless you're confident that an `Err` result *should* not be possible.
For now we'll use the `unwrap` method.
We'll explore different ways of handling a `Result` and doing proper error handling later on in this course.

For now, let's update our function so that we are actually working with the underlying vector of bytes and not the wrapped `Result` type:

```rust
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = transaction_bytes[0..4];
    1 // no return needed as the last expression without a semi-colon is automatically returned
}
```

How does the program run now?
Let's see by running `cargo run`.

We're going to get another compile error and it's going to be a confusing one:

```console
error[E0277]: the size for values of type [u8] cannot be known at compilation time
```

This will make more sense as we develop a better understanding of the difference between arrays, slices and vectors in Rust as well as the difference between the stack and the heap.
Let's get a brief overview of these concepts in the next lesson.

### Quiz
*Notice the last line of this function.
What will the compiler complain is wrong with this function?
And why?*

```rust
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex);
    1;
}
```

### Additional Reading
* Vectors: https://doc.rust-lang.org/beta/book/ch08-01-vectors.html
* Enums: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
* Recoverable Errors with Result: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="06_pointers_and_slices.md">>>> Next Lesson: Pointers and The Slice Type</a></p>
</div>
