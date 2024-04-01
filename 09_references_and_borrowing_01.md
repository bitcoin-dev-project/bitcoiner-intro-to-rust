# References and Borrowing: Part 1

So in the previous section, we were able to read the version by `read`ing the first 4 bytes and converting that into a u32 integer. However, we want to keep track of where we are in the transaction so that we can continue to do decode it. In order to do that we'll want to pass in the bytes slice to the `read_version` function and then continue using it in the `main` function. 

Let's experiment doing this similar to how we might coming from another language such as Python or Javascript.

We'll start by modifying the main function:
```
fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
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

```
fn read_version(transaction_bytes: &[u8]) -> u32 {
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read_exact(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

Let's see what happens if we try to compile this program. 

We should get an error that looks like the following:
```
8 |     transaction_bytes.read_exact(&mut buffer).unwrap();
  |     ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 | fn read_version(mut transaction_bytes: &[u8]) -> u32 {
  |                    +++
```

We cannot borrow the variable `transaction_bytes` as mutable. In Rust, if we're going to be modifying a variable, we *have* to declare that it is a mutable variable. Otherwise, the compiler will enforce that this is something that cannot be changed. The compiler gives us a suggestion with the `help` line suggesting to add the `mut` keyword before the argument, so let's try that and see what happens.

```
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read_exact(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
```

Ok that successfully compiles! Great. But is this what we actually want? Let's see what happens if we continue to call the `read` method in `main` function. It should pick up where it left off, reading the next byte to get the input count. Remember, the input count is what comes next after the version for a v1 transaction.
*Note: technically, the next byte is the compact size, which will tell us how many more bytes to read to get the input count. However, if the size is small enough then it simply represents the input count.*

If we eyeball the transaction hex, we should be able to see what the correct input count is. Let's take a look at this more closely:
```
*01000000* *02* af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000
```

The first 8 hex characters, which represent the first 4 bytes, clearly show a version 1 transaction. The next byte is 02 as you can see. So if we continue reading the transaction in the main function, we should get an input count of 2. Let's see if that's what happens.

We'll print out the next byte, which should be `2_u8`. We can actually leverage the `assert!` macro to confirm this. This line will ensure that the next byte is actually the u8 value of 2 or panic, stop the program and print an error. 

```
fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    let mut input_count = [0; 1];
    bytes_slice.read_exact(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

If we run this, we'll get an assertion error. And it will show that the input count is 1 instead of 2. Why might that be? Let's print out the `byte_slice` and see what it looks like. Since we read the first 4 bytes, those should no longer be there, and the slice should start with 2. Let's see.

```
fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);
    println!("Version: {}", version);

    println!("bytes_slice: {:?}", bytes_slice); // first 4 bytes [1, 0, 0, 0] should no longer be returned after calling `read_version`.

    let mut input_count = [0; 1];
    bytes_slice.read_exact(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
```

When we print the `bytes_slice`, we see the [1,0,0,0] is still there. But this should have been read and the pointer should have moved so that it is no longer returned by the slice. So what's happening here exactly? We'll dive into it in the next lesson. As we'll see, the slice in the `read_version` is not the same object as the slice in the `main` function. 

### [Next Lesson: References and Borrowing Part 2](09_references_and_borrowing_02.md)
