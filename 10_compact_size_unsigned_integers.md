# Compact Size Unsigned Integers

We'll talk more about the Segwit soft fork and how the transaction format changed later on in this course. For now, we're going to assume the transactions we're decoding are serialized according to the legacy, pre-segwit format. This means the next field after the version will be the number of inputs. 

If you have read [Mastering Bitcoin 3rd Edition, Chapter 6](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#length-of-transaction-input-list), you'll remember that the next byte represents the length of the transaction input list encoded as a compactSize usigned integer. The compactSize integer indicates how many bytes to read to determine the number of inputs. For example, if the length is less than 253, then the next byte is simply interpreted as an unsigned 8-bit integer (the `u8` data type in Rust). If the length is greater than 252 and less than 2^16, then we would expect to see the byte `fd` (or the integer 253) followed by two additional bytes interpreted as a `u16` integer, etc. This is the table we can use as reference:

![Compact Size Unsigned Integer Type](images/compactSize.png)

So let's write a function to read a compactSize unsigned integer. Let's think about this a bit. What kind of argument do we want to accept? And what should the return type be? Take a moment to fill out the function signature and come back.

----------------------------------------------------------------------------------------------------

For the argument type, we have to remember that we're still passing around the same mutable reference to the slice so that we can keep reading it and moving the pointer. So we'll keep the same argument type as in the `read_version` function.

Now, what should the return type be? Well, the input length can be an 8-bit, 16-bit, 32-bit or a 64-bit unsigned integer? So if we need to specify just one type for the length, let's choose the highest one as it will contain any other possibility.

```rust
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    // unimplemented!()
}
```

From here, it is fairly straightforward if/else logic. As the chart above shows in the Format column, we can tell how many bytes to read based on the byte value. If it is less than 253, then the byte is the length. If it is equal to 254, then we need to read the next two bytes. If it is equal to 255, then we need to read the next three bytes and so on. So let's implement this using a standard if/else statement block which you're probably familiar with.

```rust
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    if compact_size[0] < 253 {
        u8::from_le_bytes(compact_size) as u64
    } else if compact_size[0] == 253 {
        let mut buffer = [0; 2];
        transaction_bytes.read(&mut buffer).unwrap();
        u16::from_le_bytes(buffer) as u64
    } else if compact_size[0] == 254 {
        let mut buffer = [0; 4];
        transaction_bytes.read(&mut buffer).unwrap();
        u32::from_le_bytes(buffer) as u64
    } else if compact_size[0] == 255 {
        let mut buffer = [0; 8];
        transaction_bytes.read(&mut buffer).unwrap();
        u64::from_le_bytes(buffer)
    } else {
        panic!("invalid compact size");
    }
}
```

A few things to point out here:
1. The number of bytes read match the integer type. For example, 2 bytes give us a `u16` type. 4 bytes give us a `u32` type. 
2. We **cast** each type into a `u64`. We can convert between primitive types in Rust using the `as` keyword. https://doc.rust-lang.org/std/keyword.as.html
3. Notice how there are are no semicolons for each ending line, such as `u32::from_le_bytes(buffer) as u64`. This is the equivalent of returning that value from the function. We could also write it as `return u32::from_le_bytes(buffer) as u64;` but implicit return without semicolon is more idiomatic.
4. We will call `panic!` and crash our program if the compact size number does not match the specification. This is exactly what happens when `unwrap` is called on a failed result. This is obviously not an ideal way to handle the error case. Instead of crashing the program, we would ideally print out a nice error to the user. This is something we'll discuss more in the error handling section later in the course.

We're going to make one more change. While standard if/else statements work fine, Rust provides pattern matching via the `match` keyword and this is a good opportunity to use it as it is commonly used in Rust codebases. https://doc.rust-lang.org/book/ch06-02-match.html

```rust
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        1..=252 => {
            u8::from_le_bytes(compact_size) as u64
        },
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        },
        _ => {
            panic!("invalid compact size");          
        }
    }
}
```

What do you think? The `match` looks nicer doesn't it? Take a moment to get familiar with the syntax. Each of the `arm`'s has a pattern to match followed by `=>` and then some code to return for that given pattern. The `_` at the end is a catchall for any pattern not specified above. It's also nice that we don't have to repeat calling `compact_size[0]` for each scenario.

Now all we have to do is update our `main` function to call this and return the number of inputs. 

```rust
fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(&mut bytes_slice);
    let input_length = read_compact_size(&mut bytes_slice);

    println!("Version: {}", version);
    println!("Input Length: {}", input_length);
}
```

And if we run this, it should print the following to the terminal:

```shell
Version: 1
Input Length: 2
```

Pretty neat! We're making good progress. But even though our code compiles, how can we be sure we've written it correctly and that this function will return the appropriate number of inputs for different transactions? We want to test it with different inputs and ensure it is returning the appropriate outputs. We can do this with unit testing. So let's look into setting up our first unit test in the next section.

### Quiz
1. *How do nodes know whether the transaction is a legacy or a segwit transaction as they read it? How do they know whether to view the next field after the version as an input length encoded as compactSize or as the marker and flag for a Segwit transaction?*

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="11_unit_testing.md">>>> Next Lesson: Unit Testing</a></p>
</div>
