# Vectors and the Result Type

Let's apply what we've learned and update the `read_version` function:

```
fn read_version(transactionhex: &str) -> u32 {
    return 1;
}
```

There are two different ways we can approach this: 
1. Read the first 4 pairs of characters and do some base math to determine the u32 integer.
2. Convert the transaction into a collection bytes first and then just work with bytes.

Now that we have a basic understanding of hexadecimal format and bytes, we're going to take approach 2 and leverage an external library. It will be easier and more efficient to work with an array of `u8` integers to traverse the byte data, rather than read from a string and do unnecessary math. The library can handle the conversion for us. It will also be easier to work with as we continue to traverse our transaction and decode the rest of it. 

So the first thing we want to do is convert our hexadecimal string into a collection of bytes.

Let's add a dependency to our `Cargo.toml` file. Note the `hex` crate set to verison `0.4` at the bottom:
```
[package]
name = "bitcoin-transaction-decoder"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4"
```

Now, we want to go back to our `main.rs` file and bring the `hex` library into scope so that we can use the module's methods. We can do so by adding a line at the top of our file:
`use hex;`

If we look through the documentation at https://docs.rs/hex/latest/hex/, we see that we can convert the hex to bytes by calling the `decode` method from the `hex` module like so:
```
use hex;

fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex);
    1 // no return needed as the last expression without a semi-colon is automatically returned
}
```

We'll keep returning `1` at the bottom of the function for now so the compiler doesn't complain a `u32` wasn't returned. *Note: Rust will simply return the last expression without a semi-colon so the `return` keyword is not needed.*

Let's run this now and see what happens. Run `$ cargo run` from the terminal. 

So far, so good. That should compile fine. Let's now get the first 4 bytes from the returned collection. The returned data is a `vec` - short for Vector - which is something like a `list` in Python or an array in javascript. Of course, it's more nuanced in Rust. We'll dive deeper into some different data collection types in the next lesson. But with a `vec` we can grab the first 4 items doing something like `vec[0..4]` where `0..4` represents a range from 0 to 4, not including 4. So let's add that line.

```
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex);
    let version_bytes = transaction_bytes[0..4];
    1
}
```

What happens when we run `$ cargo run`? Well, we get an error. Take some time to read through it. You should see something like the following:
```
error[E0608]: cannot index into a value of type `Result<Vec<u8>, FromHexError>`
```

Let's examine what `hex::decode` returns. Here is the doc for the `decode` method: https://docs.rs/hex/latest/hex/fn.decode.html. Remember, we want to work with a Vector of bytes so a `vec<u8>` is the data type we're looking for. However, if we look at the return type of the `decode` function we see that the data structure we want is wrapped *inside* of a `Result` type. 

The `Result` type is a common `enum` that you will see in Rust code. Enums are a way to describe a mutually exclusive set of options for a particular variable. If you think about it, it's possible that the `hex::decode` function fails to return a proper collection of bytes. For example, what if one of the characters is not a hex character? So we get two possibilities from a `Result`, an `Ok` response or an `Err` response.

So how should we work with this? There are a few different ways to work with it. We'll explore different ways of handling a `Result` later on in this course, but for now, we'll use the `unwrap` method. Every `Result` enum has an `unwrap` method that you can call. This will return the underlying data type if the result is an `Ok` type or it will `panic` and the program will crash if the result is an `Err` type. Crashing your program is probably not the best way to handle an error, unless you're confident that an `Err` result *should* not be possible. But we'll look into different ways of handling later on. Here is the doc for the `unwrap` method: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap

For now, let's update this so that we are actually working with the underlying vector of bytes and not the wrapped `Result` type:
```
fn read_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = transaction_bytes[0..4];
    1
}
```

How does the program run now? Let's see by running `cargo run`. 

We're going to get another compilation error and it's going to be a confusing one:
`error[E0277]: the size for values of type [u8] cannot be known at compilation time`

This will make more sense as we develop a better understanding of the difference between arrays, slices and vectors in Rust as well as the difference between the stack and the heap. We'll get a better handle on these concepts as we continue on in the course, but let's get a brief overview of these concepts in the next lesson.

### Quiz
*Notice the last line of this function. What will the compiler complain is wrong with this function? And why?*
```
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

### [Next Lesson: Pointers](06_pointers_and_slices.md)
