# References and Borrowing: Part 2

As we mentioned at the end of the last lesson, the slice in the `read_version` function is not the same as the slice in the `main` function. It's a copy of it. So when we modify it in the `read_version` function, we are not modifying the one in the `main` version.

Let's add some print statements to see exactly what is happening here. We're going to use a new format specifier, `{:p}` which instead of printing the debug output as we normally do, will print the address in memory for the given slice. This will let us know whether the slice in the `read_version` function is the same as the one in the `main` function. If it is, it should have the same address location in memory. 

```
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    println!("Read Version: Transaction Bytes Memory Address: {:p}", transaction_bytes);
    println!("Read Version: Transaction Bytes: {:?}", transaction_bytes);

    u32::from_le_bytes(buffer)
}

fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let bytes_slice = transaction_bytes.as_slice();
    let version = read_version(bytes_slice);

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```

First, let's compare the two different memory addresses. If you look closely, you'll notice they are slightly different. The one in the `main` function is not the same as the one in the `read_version` function. So what's happening here is that in the `read_version` function a copy is being made. Specifically, the copy in that function is a mutable copy whereas the one in the main function remains immutable. That's not what we want. You may also notice that the slice in the `read_version` function no longer returns the first 4 bytes, `[1, 0, 0, 0]` after being read, whereas the one in the `main` function still does.

So what we really want here is a mutable <u>**reference**</u> to the slice. We want to pass around a reference so that the same object in memory is being updated. There are two types of references we can pass. An immutable reference or a mutable one. We can indicate what type by prefacing the type with `&` or `&mut`. Let's update our methods to do that.

```
fn read_version(transaction_bytes: &mut &[u8]) -> u32 { // the argument type to be accepted must be a mutable reference to a slice of u8 integers
    // Read contents of bytes_slice into a buffer.
    // Read only the exact number of bytes needed to fill the buffer.
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    println!("Read Version: Transaction Bytes Memory Address: {:p}", *transaction_bytes); // make sure to dereference the transaction_bytes to see the memory address of the object it is referring to.
    println!("Read Version: Transaction Bytes: {:?}", transaction_bytes);

    u32::from_le_bytes(buffer)
}

fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(&mut bytes_slice); // pass in a mutable reference to the bytes_slice

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```

A few things to note here.

For one, it might be a bit odd to see the two `&`s together for the argument type in the `read_version` function signature. As a reminder, the `&[u8]` indicates a slice type, which is simply a *pointer* to some heap-allocated data. The `&mut` indicates that we are passing a mutable reference to this slice. So what this really represents is a mutable reference to a pointer. The pointer is what's being modified, rather than the underlying bytes vector that it points to.

The second thing to note is that when we print the memory address of the slice in the `read_version` function, we want to first *dereference* that slice in order to get the memory address of the object it is referencing. If we don't do that, we'll still get a different memory address. This makes sense as the reference and the object it refers to are two separate items. In order to dereference a reference and access the underlying object, we can add a `*` in front, which you'll notice in the println! statement above as `*transaction_bytes`. 

So what happens if we run this now? Run it and see.

You should now get exactly what you expect. The memory address of the two objects should be identical and the slice in the `main` function has been updated. It no longer returns the first 4 bytes and the first item is the integer, 2. 

Great work so far! It may not seem like much code, but you've learned a ton of Rust fundamentals so far! 

### Quiz
1. *Rust enforces a simple, yet important rule when it comes to passing references and that is <u>**single writer OR multiple readers**</u>. In other words, you can have many different immutable, shared references to an object OR you can have just *one* mutable reference at any given time. You can't have both a shared reference and a mutable reference at the same time. Why do you think that might be? What is an example of a problem that could occur if there is a mutable reference and shared reference to the same object?* 
2. *What do you think would happen if we attempted to modify the vector while we have a slice that borrows a reference to it? Experiment by calling `.clear()` on the vector (after declaring it mutable). See example below. Run it and see what happens. Can you explain why the compiler is returning an error and the meaning of that error?*
```
fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
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

### [Next Lesson: CompactSize Unsigned Integers](10_compact_size_unsigned_integers.md)
