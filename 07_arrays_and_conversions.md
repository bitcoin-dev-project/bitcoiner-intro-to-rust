# Arrays and Conversions

There's a simple method we can use on the `u32` data type called [`from_le_bytes`](https://doc.rust-lang.org/std/primitive.u32.html#method.from_le_bytes).
This will convert a collection of bytes represented in little endian into an integer.

Let's use that and see what happens:

```rust
fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    u32::from_le_bytes(&transaction_bytes[0..4])
}
```

This won't work unfortunately.
We'll get a compiler error:
```shell
expected `[u8; 4]`, found `&[u8]`
```

If we look at the `from_le_bytes` method in the documentation and check the function signature, we'll see that the parameter expected is of the type `[u8; 4]`.
However, we're passing in a slice `&[u8]`.
What is the difference between these two?

Well, in Rust, the data type `[T; N]` where `T` is any type and `N` is the number of elements, is called an *array*.
Now we have to be careful because this is not the same as an array in other languages, such as Javascript and it's not the same as a list in Python.
An array here is a fixed size collection that is stored on the stack as opposed to the heap.
This means the data is available more efficiently at runtime as there is no need to lookup that data on the heap with the use of a pointer.
An array's size is constant, cannot be changed and must be known and defined at compile time.

So the method `from_le_bytes` only works with arrays, which makes sense.
It wants to be assured that it is only working with 4 bytes at compile time because that is exactly what is needed to create a `u32` integer on the stack.
So how do we convert a slice to an array?
One way is to initialize an array of 4 elements and then modify it by iterating over our slice and reading each value.
But there's an easier way.
Most primitive and standard data types implement the [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait, which means they have methods which allow you to convert between types.

So we can do something like the following:
```rust
<[u8; 4]>::try_from(&transaction_bytes[0..4])
```

Now remember, this method returns a `Result` type because the conversion could fail.
So we need to handle that.
We can do so by calling `unwrap` again.

```rust
<[u8; 4]>::try_from(&transaction_bytes[0..4]).unwrap()
```

If a type implements the `TryFrom` it also provides a `try_into` method that can be used in the other direction.
For example, we can also do something like this by being explicit about our variable's data type:

```rust
let version_bytes: [u8; 4] = &transaction_bytes[0..4].try_into().unwrap();
```

This way of doing conversions tends to be more common and is slightly more readable so we'll go with that.

Let's update our function now:
```rust
fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes: [u8; 4] = &transaction_bytes[0..4].try_into().unwrap();
    u32::from_le_bytes(version_bytes)
}
```

If we run this, we'll get an error expecting the conversion type to be `&[u8; 4]` instead of `[u8; 4]`.
This is because of the `&` in front of `transaction_bytes` which is incorrectly interpeted as a reference to everything that follows.
What we need to do is ensure that it only refers to the slice.
We'll add some parentheses:

```rust
fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes: [u8; 4] = (&transaction_bytes[0..4]).try_into().unwrap();
    u32::from_le_bytes(version_bytes)
}
```

Let's run it now.

And voila! It prints out the correct version number! Congratulations!

```console
Version: 1
```

### Quiz
*Can we easily convert an `array` into a `vec` similar to how we converted a `slice` into an `array` above?
If so, how does it work?*

### Additional Reading
* Array Documentation: https://doc.rust-lang.org/std/primitive.array.html
* Arrays from The Rust Book: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type

----------------------------------------------------------------------------------------------------------------------------------------------------

<div style="text-align: right">
    <p align="right"><a href="08_traits_and_reading_bytes.md">>>> Next Lesson: Traits and Reading Bytes</a></p>
</div>
