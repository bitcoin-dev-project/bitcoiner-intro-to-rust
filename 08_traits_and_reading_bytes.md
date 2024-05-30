# Traits and Reading Bytes

One way you might consider reading the rest of the data from the transaction is to use various ranges.
For example, consider the following code:

```rust
let transaction_bytes = hex::decode(transaction_hex).unwrap();
let version = u32::from_le_bytes(&transaction_bytes[0..4]);
let number_of_inputs = u32::from_le_bytes(&transaction_bytes[5..6]);
```

Notice how we're grabbing different ranges of `transaction_bytes`.
We have to repeatedly reference `transaction_bytes` and we have to keep track of the start and end indexes for each component.
This is not ideal because we can easily make mistakes.

*Note: there's an indexing mistake in the code above, can you see what it is?*
<!--
Since vec[0..4] notation is not inclusive to the end of the index, we are skipping one byte: the element at index 4.
-->

Transactions are presented in hex format for a reason.
They are designed to be serialized as byte streams that can be transmitted over the network and read one byte at a time in order.
A better solution would be to use a function that keeps track of the indices and allows us to request the number of bytes we require.
Rust's standard library's [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait allows for exactly this.
The slice data type in Rust implements the `Read` trait.
What does this mean? It gives us a method, `read`, which will read some bytes from the slice and return that data in an array.
When we call `read` again, it will start from where it left off.
In other words, the `read` trait includes the machinery to keep track of the current position we are reading in the stream and to manage the pointer as it proceeds.
This means we don't need to keep track of any indexes ourselves.

Let's walk through how this works at a high level with a quick example and then dive deeper into what traits are and how they work.

In order to use a trait method we have to first bring it into scope with a `use` statement.
In this case, we want to bring the `Read` trait into scope with `use std::io::Read`.
The next thing we want to do is use the `read` method as intended based on the example from the [documentation](https://doc.rust-lang.org/std/io/trait.Read.html#examples).

You can follow along with this example in [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=627e8e7c10d530819a80f189cedded13).
```rust
use std::io::Read;

fn main() {
    let mut bytes_slice: &[u8] = [1, 0, 0, 0, 2].as_slice();
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();

    let version = u32::from_le_bytes(buffer);

    println!("Version: {}", version);
    println!("Bytes slice: {:?}", bytes_slice);
}
```

The `mut` keyword before `bytes_slice` tells Rust the variable is mutable.
If we don't provide that keyword in a variable declaration, then the compiler will complain that we're attempting to change the value of an immutable variable, which is not allowed.

You might also notice the `&mut` keyword in the argument to the `read` method.
This indicates that we're passing in `buffer` as a *mutable reference*.
We'll talk more about this means in the next chapter so for now let's not worry about that nuance. 

When we run this, it will print the following:
```console
Version: 1
Bytes slice: [2]
```

And this is what we'd expect.
The Version is `1` and the `bytes_slice` variable has been updated and no longer contains the first 4 bytes. 

You may notice that the way this works is that you have to first create an array with a fixed size.
Calling `read` will then extract the number of bytes equal to the size of the array, store that into a buffer and then update our slice.

## What are traits?

Traits are a way to define shared behavior.
You can think of them as a template for a particular set of behaviors.
For example, the `Read` trait provides a template for types that want to "read data".
It lays out an *abstract interface* for a type: what kind of behavior is expected from the type and which functions are available to exercise that behavior.

Let's take a closer look at [the documentation for the `Read` trait](https://doc.rust-lang.org/std/io/trait.Read.html).
It defines a required method, `read`, which has the following function signature: `fn read(&mut self, buf: &mut [u8]) -> Result<usize>;`. 

```rust
...

pub trait Read {
    // Required method
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

...
```

You'll notice there's no function body, just the signature.
It means the `read` method itself is not actually implemented with any logic in the trait declaration.
We expect the types that "implement" this trait to actually provide the function logic for any *required* method, or trait methods, that have no implementation.

A trait can also provide other methods that a type can get access to once it has implemented the trait.
These are known as *provided* methods and are considered *default* implementations since they can also be overwritten.
You'll notice, for example, that there is a [`read_exact` method](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact) which is implemented with a call to the [`default_read_exact`](https://doc.rust-lang.org/src/std/io/mod.rs.html#558) method.
`default_read_exact` by itself is implemented with a call the the `read` method.

As long as a type implements the `Read` trait by providing a `read` method, it will have access to these other *provided* methods.
A type can also choose to override some or all of these *provided* methods as well and have its own custom implementations (e.g. for performance reasons).

Now if we look at the `slice` type documentation, we can see that it [*implements* the `Read` trait](https://doc.rust-lang.org/std/primitive.slice.html#impl-Read-for-%26%5Bu8%5D) and provides the function logic for the `read` method.
Let's take a look at [the source code](https://doc.rust-lang.org/src/std/io/impls.rs.html#235-250):

```rust
...

#[stable(feature = "rust1", since = "1.0.0")]
impl Read for &[u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

...
```

Don't worry if you don't understand what all of this means just yet!
Simply notice how we *implement* a trait with the `impl` keyword.
So `impl Read for &[u8]` is the code block that provides the function logic for the trait.
The other thing to notice is how the function signature for `read` matches the trait's function signature.

The idea here is that different types, not just the `&[u8]` type can implement the `Read` trait by providing the function logic for any required method and then be expected to have similar behavior and get access to the trait's provided methods.
The function logic itself for each type might differ, but given the template they are expected to take in the same arguments, return the same type and generally do the same thing, which in this case is to read some data and modify `self` and the buffer.

Again, you might notice some patterns in the code above that you are not yet familiar with, such as the `&mut` keyword and asterisk `*` before `self` at the bottom of the function.
We'll go into more detail about what these mean in the next lesson.

Let's now update our program to print out the version number leveraging the `Read` trait.
We can convert the `transaction_bytes` `Vec` to a `slice` type using the `as_slice` method.
Here is the modified `read_version` function.

```rust
use std::io::Read;

fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();

    // Read contents of bytes_slice into a buffer
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}

fn main() {
    let version = read_version("010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000");
    println!("Version: {}", version);
}
```

And voila, this will print `Version: 1` as expected!
Great job so far! 

How do we grab the modified `bytes_slice` and continue decoding the transaction?
What we probably want to do is pass in the `bytes_slice` into this function as an argument and continue using it in the `main` function.
We'll talk more about that and associated Rust concepts of references and borrowing in the next section.

### Quiz
1. *Take another look at the `Read` trait and the implementation of the `Read` trait for a slice in the documentation.
What are the required and provided methods for the trait?
What provided methods are being overwritten by the slice?*
2. *Consider the following block of code in which we create a Vec and then attempt to print it out:*
```rust
fn main() {
    let vec: Vec::<u8> = vec![0, 0, 0, 0, 0];
    println!("Vec: {}", vec);
}
```

*The compiler will return an error that the Vec cannot be formatted with the default formatter.*
*1. Which trait is not implemented for the Vec that is required for it to be printed?*
*2. How else can you print out the vector for debugging purposes?*
*3. Try and implement the correct trait for Vec so that it can be printed for standard display purposes.*

### Additional Reading
* The Rust Book on Traits: https://doc.rust-lang.org/book/ch10-02-traits.html

----------------------------------------------------------------------------------------------------------------------------------------------------

<div style="text-align: right">
    <p align="right"><a href="09_00_mutable_references.md">>>> Next Lesson: Mutable References</a></p>
</div>
