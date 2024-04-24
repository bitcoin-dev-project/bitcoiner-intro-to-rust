# File Organization and Modules

There's a lot going on in our `main.rs` file right now. Let's take one step towards organizing this a little better. We'll start by separating out the structs, struct implementations and traits which define the various transaction components and place those all in a separate file which we'll call `transaction.rs`. The file will be located in the same directory as our `main.rs` file, in `src/transaction.rs`.

```console
$ tree
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── transaction.rs

2 directories, 4 files
```

We can tell Rust that this entire file is essentially a separate module by adding a `mod transaction;` at the top of `main.rs`. When Rust sees `mod transaction;`, it will search for a `transaction.rs` file in the same directory and treat the entire contents of the file as a separate module with the name `transaction`.

Let's see what happens if try to build and run the program after making these changes. There's a lot of errors here. The first one we might see is related to the `Serialize` trait:

```console
error: cannot find derive macro `Serialize` in this scope
 --> src/transaction.rs:1:17
  |
1 | #[derive(Debug, Serialize)]
```

The `transaction.rs` is missing the `use` statement to bring the `Serialize` trait into scope. In fact, that statement is no longer needed in the `main.rs` file and should be moved to `transaction.rs`.

After changing that, if we try to build this again, we'll get more errors:

```console
error[E0412]: cannot find type `Amount` in this scope
  --> src/main.rs:11:50
   |
11 | fn read_amount(transaction_bytes: &mut &[u8]) -> Amount {
   |                                                  ^^^^^^ not found in this scope
   |
note: struct `crate::transaction::Amount` exists but is inaccessible
  --> src/transaction.rs:19:1
   |
19 | struct Amount(u64);
   | ^^^^^^^^^^^^^^^^^^^ not accessible
```

The structs are now no longer in scope. This is because they have all been moved to a separate module and need to be included in our `main.rs` file now with a `use` statement.

We can fix this by adding `use transaction::{Amount, Input, Output, Transaction};` at the top of `main.rs`.

That should now work, but we'll start seeing errors about private structs. Anything placed in a separate module is private and inaccessible from the outside unless it is explicitly declared as public.

So we need to make sure to declare the structs as public. Let's add the `pub` keyword in front of the `Amount`, `Input`, `Output`, and `Transaction` structs and see what happens.

The other compiler errors will go away, but now we will see an issue with private fields.

```console
error[E0423]: cannot initialize a tuple struct which contains private fields
  --> src/main.rs:16:5
   |
16 |     Amount(u64::from_le_bytes(buffer))
   |     ^^^^^^
   |
note: constructor is not visible here due to private fields
  --> src/transaction.rs:19:19
   |
19 | pub struct Amount(u64);
   |                   ^^^ private field
help: consider making the field publicly accessible
  --> src/transaction.rs:19:19
   |
19 | pub struct Amount(pub u64);
   |                   +++
```

Let's make sure to set all the fields public with the `pub` keyword as well.

This should work now!

We're going to make a modification, however. Sometimes we don't actually want to set a field public, but offer a public function for setting a private field instead. Let's look at `Amount` as an example. Instead of simply setting the `u64` field directly, we can write this differently so that it's even clearer what the code is doing and what is actually expected.

Let's start by removing the `pub` keyword from the `u64` field on `Amount`. Instead, we'll add a `from_sat` function within the `Amount` `impl` block. There is a distinction here, however. It's not a typical struct method, which applies to a particular instance. Instead, this is a **type-associated function**. It doesn't take `self` as an argument and is called on the type itself. It is often used to create a new instance of a struct. For example, something like `Amount::new()`. In this case, we'll create a function that will have a signature like this `pub fn Amount::from_sat(satoshis: u64) -> Amount`. Remember, it still needs to be public since we will call it in `main.rs`.

*transaction.rs*
```rust
#[derive(Debug)]
pub struct Amount(u64);

impl Amount {
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi)
    }
}
```

*main.rs*
```rust
fn read_amount(transaction_bytes: &mut &[u8]) -> Amount {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();

    Amount::from_sat(u64::from_le_bytes(buffer))
}
```

What do you think? Reads better doesn't it? Over time, as the `transaction.rs` file grows and gets more complex, it might make sense to split that into more files and modules. For example, in the Rust-Bitcoin library, there's a [file and module](https://github.com/rust-bitcoin/rust-bitcoin/blob/fe8ce059b4332a1438286e86a12166d86a3a1053/units/src/amount.rs) dedicated solely to the `Amount` struct field.

Let's work on finishing up reading our legacy, pre-segwit transaction and then start looking into better error handling.

<hr/>

<div>
    <p align="right"><a href="18_decoding_legacy_transaction.md">>>> Next Lesson: Decoding a Legacy Transaction</a></p>
</div>
