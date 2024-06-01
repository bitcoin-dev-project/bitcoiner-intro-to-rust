# Unit Testing

The typical way to add unit tests to our program is to first add the `#[cfg(test)]` annotation and then place our test code under a separate module, as identified by the `mod` keyword.
Here's an example:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

So let's start setting up our tests by adding the following after the `main` function:
```rust
#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_read_compact_size() {
        // unimplemented
    }
}
```

Notice how we call `use super::read_compact_size`.
We have to bring the function we're testing into scope because we are in a separate `unit_tests` module and need access to private functions.
In Rust, unless we add the `pub` keyword to a function, it is private.

We identify each test function with the `#[test]` annotation.
We can have other functions in the test module without that annotation.
This means they won't be run as tests, but could be useful as helper functions for other tests in the module.

Now, if you run `cargo test` from the command line instead of `cargo run`, your tests will run and return results to the terminal.
You might see something like the following:

```console
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs (target/debug/deps/transaction_decoder_11-283cd8d491efdffc)

running 1 test
test unit_tests::test_reading_compact_size ... ok

test result: ok.
1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Pretty cool.
Let's start adding some logic for testing our `read_compact_size` function.
We'll start simple.

```rust
#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 2_u64);
    }
}
```

If you look at the `assert_eq!` statement, we intentionally set the expected length to `2_u64` instead of the correct one, `1_u64`.
This is to make sure our tests are running properly and will fail when run.
We'll then make the correction and ensure the test passes.

If you run `cargo test`, you should see the test failures:

```console
   Compiling transaction_decoder_11 v0.1.0 (/Users/shaanbatra/Projects/bitcoiner-intro-to-rust/code/transaction_decoder_11)
    Finished test [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests src/main.rs (target/debug/deps/transaction_decoder_11-283cd8d491efdffc)

running 1 test
test unit_tests::test_reading_compact_size ... FAILED

failures:

---- unit_tests::test_reading_compact_size stdout ----
thread 'unit_tests::test_reading_compact_size' panicked at src/main.rs:59:9:
assertion `left == right` failed
  left: 1
 right: 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    unit_tests::test_reading_compact_size

test result: FAILED.
0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin transaction_decoder_11`
```

Take a moment to get familiar with the terminal output.
The area you want to pay attention to is where it compares the `left` and `right` values of the assertion statement.
The two should be equal but are not.
It also displays the values, `1` and `2`.
We know the correct length the function should return is `1` so let's update our test to reflect that.

```rust
#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 1_u64);
    }
}
```

Now if you run `cargo test` again, you will see that all tests pass with the output line at the end, `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`

So far so good.
Let's add another testing scenario.

```rust
#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64);
    }
}
```

We added another scenario for when the first byte is `0xfd` or the integer `253`.
In this case, we want to read the next two bytes to determine the input length.
So if we pass this slice in, we should get the correct length.
What is the length represented by the bytes `0` and `1`?
Well we can use base math to confirm.
Remember, bytes are in little endian, so the `0` is in the least significant position and the `1` is in the most significant position: `1*256_u64.pow(1) + 0*256_u64.pow(0)` or just `256_u64`.

You also might have noticed that we declared the same variables, `bytes` and `length`.
This is called [*shadowing*](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) in Rust as the second declaration will "*overshadow*" the first.
It's easier to write it this way sometimes, rather than come up with new variable names for each scenario.

Let's add a few more scenarios to test the other arms of the match statement:

```rust
#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64.pow(7));
    }
}
```

Let's add another scenario from a real world example.
We're going to use an example that was mentioned in the [Learn Me A Bitcoin](https://learnmeabitcoin.com/explorer/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1) tutorial site.
It's a [transaction](https://mempool.space/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1) with 20,000 inputs which was confirmed in 2015.
It is 840,000 vbytes large and paid 0 fees!
It was all 0 inputs and 0 output as well so no amount of Bitcoin was transferred.
Interesting.

Here is the raw transaction hex: https://mempool.space/api/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1/hex

If we look at the first few bytes, we can see the version followed by `fd`: `01000000fd204e`

`fd` indicates that the input length comes from the next two bytes.
So the bytes, `0x20` and `0x4e` should evaluate to 20,000.
Let's confirm this in our test.

```rust
fn test_reading_compact_size() {
    let mut bytes = [1_u8].as_slice();
    let length = read_compact_size(&mut bytes);
    assert_eq!(length, 1_u64);

    let mut bytes = [253_u8, 0, 1].as_slice();
    let length = read_compact_size(&mut bytes);
    assert_eq!(length, 256_u64);

    let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
    let length = read_compact_size(&mut bytes);
    assert_eq!(length, 256_u64.pow(3));

    let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
    let length = read_compact_size(&mut bytes);
    assert_eq!(length, 256_u64.pow(7));

    // https://mempool.space/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1
    // fd is 253
    // transaction has 20,000 empty inputs
    let hex = "fd204e";
    let decoded = hex::decode(hex).unwrap();
    let mut bytes = decoded.as_slice();
    let length = read_compact_size(&mut bytes);
    let expected_length = 20_000_u64;
    assert_eq!(length, expected_length);
}
```

Run this with `cargo test` and all the tests should pass!

Great! We've learned about unit testing.
We'll keep this in mind as we write more functions with complex logic.
Let's keep it moving and keep reading the transaction.

### Additional Reading
* Test Organization: https://doc.rust-lang.org/book/ch11-03-test-organization.html

<hr/>

<div>
    <p align="right"><a href="12_reading_inputs_and_type_coercion.md">>>> Next Lesson: Reading Inputs and Type Coercion</a></p>
</div>
