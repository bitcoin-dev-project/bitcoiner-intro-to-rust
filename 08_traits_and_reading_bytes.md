# Traits and Reading Bytes

One way you might consider reading the rest of the data from the transaction is to use various ranges. For example, consider the following code:

```
let transaction_bytes = hex::decode(transaction_hex).unwrap();

let version = u32::from_le_bytes(&transaction_bytes[0..4]);
let number_of_inputs = u32::from_le_bytes(&transaction_bytes[5..6]);
```

Notice how we're grabbing different ranges of `transaction_bytes`. We have to repeatedly reference `transaction_bytes` and we have to keep track of the start and end indexes for each component. This is not ideal. Transactions are presented in hex format for a reason. They are designed to be serialized as byte streams that can be transmitted over the network and read one byte at a time in order.

One way to read a byte stream is to leverage Rust's standard library's `Read` trait: https://doc.rust-lang.org/std/io/trait.Read.html. What is a trait? Well, it's a way to define shared behavior. So any type that *implements* the trait provides the method implementations for the defined trait behaviors. For more information on traits, see here: https://doc.rust-lang.org/book/ch10-02-traits.html.

The slice data type in Rust implements the `Read` trait. What benefit does this give us? Well, as we will see, we can read some bytes from the slice by calling the `read` method and store that into a variable. When we call `read` again, it will start from where it left off. In other words, it keeps track of where we are in the stream and modifies the pointer as it reads. This means we don't need to keep track of any indexes.

Let's see an example:
```
fn main() {
    let bytes_slice: &[u8] = [1, 0, 0, 0, 2].as_slice();

    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();

    let version = u32::from_le_bytes(&buffer);

    println!("Version: {}", version);
    println!("Bytes Slice: {:?}", bytes_slice);
}
```

This won't run if you try to compile it. You'll get a compile error that the `read` method is not found for `&[u8]`. This is because the trait implementations only become available when the trait is brought into scope with a `use` import. So you just need to add a `use std::io::Read;` line at the top. Let's add that and run this again.

This should print out the following:
```
Version: 1
Bytes Slice: [2]
```

We converted the 4 bytes from the buffer into an unsigned 32-bit integer. And notice how the bytes slice has been modified after being read into the buffer. 

You may notice that the way this works is that you have to first create an array (which serves as the buffer) with a known size. Calling `read` will then extract the number of bytes equal to the size of the array and shift the pointer to the underlying data forward by that same amount.

Let's now modify our program to print out the version number leveraging the `Read` trait. We can convert the `transaction_bytes` `Vec` to a `slice` type using the `as_slice` method. Here is the modified `extract_version` function.

```
fn extract_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice: &[u8] = transaction_bytes.as_slice();

    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

And voila, this will print `Version: 1` as expected. But this doesn't seem ideal. How do we grab the modified `bytes_slice` and continue decoding the transaction? What we probably want to do is pass in the `bytes_slice` into this function as an argument and continue using it in the `main` function. We'll talk more about that and associated Rust concepts of references and borrowing in the next section.

### Quiz
*Consider the following block of code in which we create a Vec and then attempt to print it out:*
```
fn main() {
    let vec = vec![0, 0, 0, 0, 0];
    println!("Vec: {}", vec);
}
```
*The compiler will return an error that the Vec cannot be formatted with the default formatter.*

*1. Which trait is not implemented for the Vec that is required for it to be printed?*

*2. How else can you print out the vector for debugging purposes?*

*3. Try and implement the correct trait for Vec so that it can be printed for standard display purposes.*

### [Next Lesson: References and Borrowing Part 1](09_references_and_borrowing_01.md)