# References and Borrowing: Part 1

So in the previous section, we were able to read the version by `read`ing the first 4 bytes and converting that into a u32 integer. However, we want to keep track of where we are in the transaction so that we can continue to do decode it. In order to do that we'll want to pass in the bytes slice to the `read_version` function and then continue using it in the `main` function. 

Let's experiment doing this similar to how we might coming from another language such as Python or Javascript.

We'll start by modifying the main function:
```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);

    println!("Version: {}", version);
}
```

Notice how we first decode the hex string to get a `Vec` of bytes. We then convert that into a slice and then pass that into the `read_version` function. We'll now have to modify the `read_version` function in order to accept the correct argument. 

Before I show you the modified function, what do you think is the correct argument type for the slice? Take a moment and then check back here.

----------------------------------------------------------------------------------------------------

If you guessed it should be something like `&[u8]` you guessed correctly! This is the type signature for a slice of u8 integers. We'll see in a moment that this is not exactly the type we're looking for, but this is good enough for the moment. Let's take a look at what the modified `read_version` function looks like. 

```rust
fn read_version(transaction_bytes: &[u8]) -> u32 {
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

Let's see what happens if we try to compile this program. 

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

We cannot borrow the variable `transaction_bytes` as mutable. In Rust, if we're going to be modifying a variable, we *have* to declare that it is a mutable variable. Otherwise, the compiler will enforce that this is something that cannot be changed. The compiler gives us a suggestion with the `help` line suggesting to add the `mut` keyword before the argument, so let's try that and see what happens.

```rust
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

Ok that successfully compiles! Great. But is this what we actually want? Let's see what happens if we continue to call the `read` method in `main` function. It should pick up where it left off, reading the next byte to get the input count. Remember, the input count is what comes next after the version for a pre-segwit transaction.
*Note: technically, the next byte is the compact size, which will tell us how many more bytes to read to get the input count. However, if the size is small enough then it simply represents the input count.*

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
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    let mut input_count = [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

If we run this, we'll get an assertion error. And it will show that the input count is 1 instead of 2. Why might that be? Let's print out the `byte_slice` and see what it looks like. Since we read the first 4 bytes, those should no longer be there, and the slice should start with 2. Let's see.

```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    println!("bytes_slice: {:?}", bytes_slice); // first 4 bytes [1, 0, 0, 0] should no longer be returned after calling `read_version`.

    let mut input_count = [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

When we print the `bytes_slice`, we see the `[1,0,0,0]` is still there. But this should have been read and the pointer should have moved so that it is no longer returned by the slice. So what's happening here exactly? We'll dive into it in the next lesson. As we'll see, the slice in the `read_version` is not the same object as the slice in the `main` function. 

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="09_references_and_borrowing_02.md">>>> Next Lesson: References and Borrowing Part 2</a></p>
</div>
