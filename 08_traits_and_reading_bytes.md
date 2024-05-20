# Traits and Reading Bytes

One way you might consider reading the rest of the data from the transaction is to use various ranges. For example, consider the following code:

```rust
let transaction_bytes = hex::decode(transaction_hex).unwrap();
let version = u32::from_le_bytes(&transaction_bytes[0..4]);
let number_of_inputs = u32::from_le_bytes(&transaction_bytes[5..6]);
```

Notice how we're grabbing different ranges of `transaction_bytes`. We have to repeatedly reference `transaction_bytes` and we have to keep track of the start and end indexes for each component. This is not ideal. Transactions are presented in hex format for a reason. They are designed to be serialized as byte streams that can be transmitted over the network and read one byte at a time in order.

One way to read a byte stream is to leverage Rust's standard library's [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait. The slice data type in Rust implements the `Read` trait. What does this mean? Well, as we will see, it gives us a method, `read`, which will read some bytes from the slice and then store that data into a array. When we call `read` again, it will start from where it left off. In other words, it keeps track of where we are in the stream and modifies the pointer as it reads. This means we don't need to keep track of any indexes.

## Traits

We've mentioned traits a few times now, but haven't gone into detail about what they are and how they work. We'll get some more practice with them later on, but for now it's enough to understand that traits are a way to define shared behavior. You can think of them as a template for a particular behavior. For example, the `Read` trait provides a template for types that want to "read data". It lays out what to expect and what types of functions are available.

Let's take a closer look at [`Read` from the documentation](https://doc.rust-lang.org/std/io/trait.Read.html). It has a required method, `read`, which has the following function signature: `fn read(&mut self, buf: &mut [u8]) -> Result<usize>;`. 

```rust
...

pub trait Read {
    // Required method
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

...
```

The functions themselves are not actually implemented with any logic. You'll notice there's no function body, just the signature. The types that implement this trait are expected to provide the function logic for each of these methods. So the trait is really just a template. 

Now, if we look at the `slice` type from the documentation, we can see that it [*implements* `Read`](https://doc.rust-lang.org/std/primitive.slice.html#impl-Read-for-%26%5Bu8%5D) meaning it provides the function logic for the given trait template. Let's take a look at the [implementation](https://doc.rust-lang.org/src/std/io/impls.rs.html#235-250):

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

Don't worry if you don't understand how to read all of this just yet! Simply notice how we *implement* a trait with the `impl` keyword. So `impl Read for &[u8]` is the code block that provides the function logic for the trait. The other thing to notice is how the function signature matches the trait's function signature.

The idea here is that different types, not just the `&[u8]` type can implement the `Read` trait and then be expected to have a similar behavior. The function logic itself for each type might differ, but they are expected to take in the same arguments, return the same type and generally do the same thing, which in this case is to read some data and modify `self` and the buffer. You might notice some patterns here that you are not yet familiar with, such as the `&mut` keyword and asterisk `*` before `self` at the bottom of the function. Don't worry, we'll go into more detail about what these mean in the next lesson.

For now, let's experiment by following the second example from the [documentation for Read](https://doc.rust-lang.org/std/io/trait.Read.html#examples). We'll comment out our original code and experiment with these new lines:
```rust
fn main() {
    let bytes_slice: &[u8] = [1, 0, 0, 0, 2].as_slice();
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();

    let version = u32::from_le_bytes(buffer);

    println!("Version: {}", version);
    println!("Bytes Slice: {:?}", bytes_slice);

    // let version = read_version("010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000");
    // println!("Version: {}", version);
}
```

Try running this now with `cargo run`. This fail to compile. You'll get a compile error that the `read` method is not found for `&[u8]`. This is because the trait implementations only become available when the trait is brought into scope with a `use` import. So you just need to add a `use std::io::Read;` line at the top. Let's add that and run this again.

We'll get another compile error: 
```rust
error[E0596]: cannot borrow `bytes_slice` as mutable, as it is not declared as mutable
  --> src/main.rs:12:5
   |
12 |     bytes_slice.read(&mut buffer).unwrap();
   |     ^^^^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
10 |     let mut bytes_slice: &[u8] = [1, 0, 0, 0, 2].as_slice();
   |         +++
```

This is a straightforward idea that we haven't talked about until now, but in Rust if any variable is going to be modified, we have to explicitly declare it as *mutable* with the `mut` keyword. This just means that the variable is allowed to change. The `read` method will attempt to modify our `bytes_slice` so we have to declare it as *mutable*. 

Ok, if we add that and run it again, it should work and will print out the following:
```shell
Version: 1
Bytes Slice: [2]
```

We converted the 4 bytes from the buffer into an unsigned 32-bit integer. And notice how the bytes slice has been modified after being read into the buffer. It no longer contains the first 4 elements, `[1, 0, 0, 0]`.

You may notice that the way this works is that you have to first create an array with a known size. Calling `read` will then extract the number of bytes equal to the size of the array, store that into our buffer and then modify our fat pointer reference to the underlying data.

Let's now update our program to print out the version number leveraging the `Read` trait. We can convert the `transaction_bytes` `Vec` to a `slice` type using the `as_slice` method. Here is the modified `read_version` function.

```rust
use std::io::Read;

fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();

    // Read contents of bytes_slice into a buffer.
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

But this doesn't seem ideal. How do we grab the modified `bytes_slice` and continue decoding the transaction? What we probably want to do is pass in the `bytes_slice` into this function as an argument and continue using it in the `main` function. We'll talk more about that and associated Rust concepts of references and borrowing in the next section.

### Quiz
*Consider the following block of code in which we create a Vec and then attempt to print it out:*
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
