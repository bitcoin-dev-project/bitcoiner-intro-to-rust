# 9.1 Solution

### Quiz
*What do you think would happen if we attempted to modify the vector while we have a slice that borrows a reference to it? Experiment by calling `.clear()` on the vector (after declaring it mutable). See example below. Run it and see what happens. Can you explain why the compiler is returning an error and the meaning of that error?*
```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let mut transaction_bytes = hex::decode(transaction_hex).unwrap(); // declare the vector as mutable
    let mut bytes_slice = transaction_bytes.as_slice();
    transaction_bytes.clear(); // clear the vector elements while there is another reference to its elements
    
    let version = read_version(&mut bytes_slice);

    println!("Main: Bytes Slice Memory Address: {:p}", bytes_slice);
    println!("Main: Bytes Slice: {:?}", bytes_slice);

    println!("Version: {}", version);
}
```

### Solution
We'll get a familiar error enforcing the Single Writer or Multiple Readers rule:
```console
error[E0502]: cannot borrow `transaction_bytes` as mutable because it is also borrowed as immutable
  --> src/main.rs:17:5
   |
16 |     let mut bytes_slice = transaction_bytes.as_slice();
   |                           ----------------- immutable borrow occurs here
17 |     transaction_bytes.clear(); // clear the vector elements while there is another reference to its elements
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
18 |     
19 |     let version = read_version(&mut bytes_slice);
   |                                ---------------- immutable borrow later used here
```
Remember, our slice which is of the type, `&[u8]` borrows a reference to the heap data, which is actually owned by `transaction_bytes`. However, by calling `clear` we are modifying the underlying heap data while we still have a shared reference to it. Rust does not allow this. 
