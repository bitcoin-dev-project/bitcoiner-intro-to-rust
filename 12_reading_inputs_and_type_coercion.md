# Reading Inputs and Type Coercion

Each input in a transaction contains the following information: 
* the previous output txid (32 bytes length)
* the previous output index (4 bytes length, represented as a u32 integer)
* a ScriptSig (variable length preceded by compact size integer)
* a sequence number (4 bytes length, represented as a u32 integer).

The ScriptSig can be have variable length and so is preceded by a compact size integer which indicates the length of the field in bytes.
Prior to Segwit, the ScriptSig was where a digital signature would be provided for unlocking the funds of the referenced output (as indicated by the previous output txid and previous output index).
Now, for Segwit transactions, this field is empty with a compact size length of 0x00 as the signature is no longer contained in the input data, but is instead "*segregated*" from the rest of the transaction in a separate witness field.
For more information on SegWit, see [this section](https://github.com/bitcoinbook/bitcoinbook/blob/6d1c26e1640ae32b28389d5ae4caf1214c2be7db/ch06_transactions.adoc#segregated-witness) from Mastering Bitcoin, Chapter 6.

We already have the input length so we know how many times to read the input information.
We'll start by using a for loop and iterate over a range.
Since we don't need the range index number, we can just replace the unused variable with an underscore, `_`.
More details on loops in Rust can be [found here](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for).

```rust
fn main() {
...
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(&mut bytes_slice);
    let input_length = read_compact_size(&mut bytes_slice);
    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
    }
...
}
```

Let's implement the `read_txid` function.
We know we're looking to read the next 32 bytes, which displayed in hex format will be the transaction id we show to a user.
There's one catch, however.
Whenever we display transaction ids to users, we display them in *big-endian* format.
However, those ids are stored internally in blocks as *little-endian*.
This means we have to reverse the id before showing it to the user.
A description about why this is can be [found here](https://github.com/bitcoinbook/bitcoinbook/blob/6d1c26e1640ae32b28389d5ae4caf1214c2be7db/ch06_transactions.adoc#internal_and_display_order) in Mastering Bitcoin, Chapter 6.

Before we show the implementation below, why don't you take a stab at the function signature?
If you're feeling confident, write out the whole function and then compare your answer here.
Some hints:
1. Let's not worry about the hex display just yet.
Let's just return the appropriate bytes in *big endian*.
Remember, by default it is *little-endian*.
2. *We know the exact size of the amount of bytes we want to return, so what kind of data structure is appropriate for a fixed-size amount of bytes?
Do we need to store anything on the heap?
Or can we just store this data on the stack?*

<hr/>

```rust
fn read_txid(transaction_bytes: &mut &[u8]) -> [u8; 32] {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse(); // txids are formatted in big endian
    buffer
}
```

So all we have to do here is read 32 bytes and store that into an array, which is a fixed size amount of bytes.
Of course, we need to reverse the bytes so that they are in big-endian.
Pretty simple right? So what's next? The next 4 bytes will give us the index of that transaction that we're spending.

If you think about it, this is identical to what we did to get the version.
We read 4 bytes and returned the u32 integer which represented the version.
The index is the same.
So perhaps instead of calling that function `read_version`, we can rename it to `read_u32` to make it more generic and just call that here:

```rust
...
fn read_u32(transaction_bytes: &mut &[u8]) -> u32 { // changed from read_version
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
...
...
    let version = read_u32(&mut bytes_slice); // call read_u32
    let input_length = read_compact_size(&mut bytes_slice);

    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice); // call read_u32
    }
...
```

Next let's get the size of our ScriptSig by reading the compactSize.

```rust
    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice); // call read_u32
        let script_size = read_compact_size(&mut bytes_slice);
    }
```

### Type Coercions

Now that we have the `script_size`, we know how many bytes to read, but this gets us into an interesting problem.
When we create a buffer to read bytes into, we always have to provide a fixed size array.
However, the `script_size` is dynamic and cannot be known at compile time.
It can only be determined at runtime.
You're not able to do something like this as the compiler will complain:

```rust
    let mut buffer = [0; script_size];
```

Let's look closer at the `read` method and what type of argument it accepts.
If we look at the documentation, it accepts the argument of type `&mut [u8]`.
https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read

This is interesting.
Technically, it only accepts a mutable reference to a slice.
But we've actually been passing in a mutable reference to an array!
Remember an array is a fixed size of type `[u8; n]` and not a slice of type `[u8]`.
So how has this been working at all?
I thought we had to be explicit with types in Rust?

Well, under the hood, Rust is making an implicit conversion.
It does this in a few different cases.
In the case of an array, there is something known as an **Unsized Coercion**, in which it will automatically convert a sized type (such as an array, `[T; n]`) into an unsized type (a slice, `[T]`).

There is also something known as a **Deref Coercion**, which we can take advantage of here and which is something we alluded to in chapter 9.
Basically, if a type implements the `Deref` trait, Rust will implicitly call the `deref` method on it until it gets the type that matches the argument's required type.

So going back to reading our script, what we want is a dynamically-sized buffer to read into.
A vector would work just fine.
But can we use it?
Can we pass it into the `read` method as an argument?
It turns out we can!
In Rust, a Vec [implements the `DerefMut`](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#2769) trait which dereferences to a slice.
So we can initialize a `Vec` filled with 0s of the size of the script and then pass that into the `read` method as a mutable reference (`&mut Vec<u8>`).
It will then be dereferenced to a slice and match the correct argument type, which is `&mut [u8]`.

We'll create a new function called `read_script` which will return a `Vec<u8>`:

```rust
fn read_script(transaction_bytes: &mut &[u8]) -> Vec<u8> {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer
}
```

Lastly, we need to read the the last 4 bytes for the sequence number.
A description of what the sequence number represents can be found in Mastering Bitcoin, Chapter 6.

```rust
...
    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);
    }
...
```

Alright, now that we have each of the components of an input, what should we do with it?
It makes sense to collect all this data together into one unified structure rather than just separate variables.
The right type for this is Rust's `Struct` type, which we'll explore in the next lesson.
Onwards!

### Additional Reading
* Implicit Deref Coercions: https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
* Unsized Coercions: https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions
* Sequence: https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#sequence

<hr/>

<div>
    <p align="right"><a href="13_structs.md">>>> Next Lesson: Structs</a></p>
</div>
