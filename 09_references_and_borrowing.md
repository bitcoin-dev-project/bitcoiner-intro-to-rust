# References and Borrowing

So in the previous section, we were able to read the version by `read`ing the first 4 bytes and converting that into a u32 integer. However, we want to keep track of where we are in the transaction so that we can continue to do decode it. In order to do that we'll want to pass in the bytes slice to the `read_version` function and then continue using it in the `main` function.

Let's experiment doing this similar to how we might coming from another language such as Python or Javascript.

We'll start by modifying the `main` function before updating our `read_version` function:
```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);

    println!("Version: {}", version);
}
```

Notice how we first decode the hex string to get a `vec` of bytes. We then convert that into a slice and then pass that into the `read_version` function.

We'll now have to modify the `read_version` function in order to accept the correct argument. What do you think is the correct type for the the argument? Take a moment and then check back here.

<hr/>

If you guessed it should be something like `&[u8]` you guessed correctly! This is the type signature for a slice of u8 integers. We'll see in a moment that this is not exactly the type we're looking for, but this is good enough for the moment. Let's take a look at what the modified `read_version` function looks like. 

```rust
fn read_version(transaction_bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

And let's see what happens if we try to compile this program.

We should get an error that looks like the following:
```shell
8 |     transaction_bytes.read(&mut buffer).unwrap();
  |     ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 | fn read_version(mut transaction_bytes: &[u8]) -> u32 {
  |                    +++
```

Remember, if we're going to modify a variable, we have to declare that it is a mutable variable using the `mut` keyword. The compiler is suggesting that we add `mut` to the beginning of the `transaction_bytes` argument. Let's try that and see what happens.

```rust
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

Ok that successfully compiles now! Great.

But is this what we actually want? Let's see what happens if we continue to call the `read` method in `main` function. It should pick up where it left off, reading the next byte to get the input count. Remember, the input count is what comes next after the version for a legacy (pre-segwit) transaction.

If we eyeball the transaction hex, we should be able to see what the correct input count is. Let's take a look at this more closely:
```
*01000000* *02* 42d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000
```

The first 8 hex characters, which represent the first 4 bytes, clearly show a version 1 transaction. The next byte is 02 as you can see. So if we continue reading the transaction in the main function, we should get an input count of 2. Let's see if that's what happens.

We'll print out the next byte, which should be `2_u8`. We can actually leverage the `assert!` macro to confirm this. This line will ensure that the next byte is actually the u8 value of 2 or panic, stop the program and print an error. 

```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    let mut input_count = [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

If we run this, we'll get an assertion error. And it will show that the input count is 1 instead of 2.

```console
Version: 1
thread 'main' panicked at src/main.rs:19:5:
assertion `left == right` failed
  left: [1]
 right: [2]
```

Why might that be? Let's print out the debug output of `byte_slice` and see what it looks like. Since we read the first 4 bytes, those should no longer be there, and the slice should start with 2. Let's see.

```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    println!("bytes_slice: {:?}", bytes_slice); // first 4 bytes [1, 0, 0, 0] should no longer be returned after calling `read_version`.

    let mut input_count = [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

When we print the `bytes_slice`, we see the `[1,0,0,0]` is still there at the beginning. But this should have been `read` and the pointer should have moved so that this portion is no longer returned by the slice. What's happening here exactly? 

As it turns out, the slice in the `read_version` is not the same object as the slice in the `main` function. It's actually a *copy* of it. So when we modify it in the `read_version` function, we are not modifying the same one in the `main` version.

Let's add some print statements to see this more clearly. We're going to use a new format specifier, `{:p}` which instead of printing the debug output as we normally do, will print the address in memory for the given slice. This will let us know whether the slice in the `read_version` function is the same as the one in the `main` function. If it is, it should have the same address location in memory.

```rust
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    println!("Read Version: Transaction Bytes Memory Address: {:p}", transaction_bytes);
    println!("Read Version: Transaction Bytes: {:?}", transaction_bytes);

    u32::from_le_bytes(buffer)
}

fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```

Let's run this and look at the output.

First, let's compare the two different memory addresses. If you look closely, you'll notice they are slightly different (it might only the last digit that is different). The one in the `main` function is not the same as the one in the `read_version` function. So what's happening here is that in the `read_version` function a copy is being made. Specifically, the copy in that function is a mutable copy whereas the slice in the `main` function remains immutable. That's not what we want. You may also notice that the slice in the `read_version` function no longer returns the first 4 bytes, `[1, 0, 0, 0]` after being read, whereas the one in the `main` function still does.

What we really want here is a mutable <u>**reference**</u> to the slice, not a *mutable copy*. We want to pass around a reference so that the same object in memory is being updated. There are two types of references we can pass. An immutable reference or a mutable one. We can indicate what type by prefacing the type with `&` or `&mut`. Instead of the argument type being `mut transaction_bytes: &[u8]`, we want the the `transaction_bytes` variable to be of the type `&mut &[u8]`. 

Let's update our methods:

```rust
fn read_version(transaction_bytes: &mut &[u8]) -> u32 { // the argument type is a mutable reference to a slice of u8 integers
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    println!("Read Version: Transaction Bytes Memory Address: {:p}", *transaction_bytes); // make sure to dereference the transaction_bytes variable to see the memory address of the object it is referring to.
    println!("Read Version: Transaction Bytes: {:?}", transaction_bytes);

    u32::from_le_bytes(buffer)
}

fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice(); // the bytes_slice is mutable
    let version = read_version(&mut bytes_slice); // pass in a mutable reference to the bytes_slice

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```

A few things to note here:

1. It might be a bit odd to see the two `&`s together for the argument type in the `read_version` function signature. As a reminder, the `&[u8]` indicates a slice type, which is simply a *pointer* to some heap-allocated data. The `&mut` indicates that we are passing a mutable reference to this slice. So what this really represents is a mutable *reference* to a pointer. The pointer is what's being modified, rather than the data in heap memory that it points to.

2. The second thing to note is that when we print the memory address of the slice in the `read_version` function, we need to first *dereference* it. If we don't do that, we'll still get a different memory address. This makes sense as the reference and the object it refers to are two separate things. In order to dereference a reference and access the underlying object, we can add a `*` in front, which you'll notice in the `println!` statement above as `*transaction_bytes`. 

So what happens if we run this now? Run it and see.

You should now get exactly what you expect. The memory address of the two objects should be identical and the slice in the `main` function has been updated. It no longer returns the first 4 bytes and the first item is the integer, 2. 

Great work so far! It may not seem like much code, but you've learned a ton of Rust fundamentals! 

### Quiz
1. *Rust enforces a simple, yet important rule when it comes to passing references and that is <u>**single writer OR multiple readers**</u>. In other words, you can have many different immutable, shared references to an object OR you can have just *one* mutable reference at any given time. You can't have both a shared reference and a mutable reference at the same time. Why do you think that might be? What is an example of a problem that could occur if there is a mutable reference and shared reference to the same object?*
2. *What do you think would happen if we attempted to modify the vector while we have a slice that borrows a reference to it? Experiment by calling `.clear()` on the vector (after declaring it mutable). See example below. Run it and see what happens. Can you explain why the compiler is returning an error and the meaning of that error?*
```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let mut transaction_bytes = hex::decode(transaction_hex).unwrap(); // declare the vector as mutable
    let bytes_slice = transaction_bytes.as_slice();
    transaction_bytes.clear(); // clear the vector elements while there is another reference to its elements
    
    let version = read_version(bytes_slice);

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```
3. *You will find that certain methods for manipulating the elements of a vector such as sorting are available only on the slice type and not the vector. However, if you call `.sort` on a vector, it will still work. Why is that? Hint: when method calls are made in Rust, it not only accesses the method on the specific data type, but any methods on the data type that it dereferences to as indicated by the DeRef trait implementation. So what does a vector dereference to? Can you find the relevant trait implementation?* <br/>
https://doc.rust-lang.org/std/vec/struct.Vec.html <br/>
https://doc.rust-lang.org/std/primitive.slice.html#method.sort <br/>

### Additional Reading
* https://exercism.org/tracks/rust/concepts/mutability

<hr/>

<div>
    <p align="right"><a href="10_compact_size_unsigned_integers.md">>>> Next Lesson: CompactSize Unsigned Integers</a></p>
</div>
