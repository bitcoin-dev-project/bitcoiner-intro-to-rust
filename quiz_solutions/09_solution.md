# 09 Solution

### Quiz
1. *Rust enforces a simple, yet important rule when it comes to passing references and that is <u>**single writer OR multiple readers**</u>. In other words, you can have many different immutable, shared references to an object OR you can have just *one* mutable reference at any given time. You can't have both a shared reference and a mutable reference at the same time. Why do you think that might be? What is an example of a problem that could occur if there is a mutable reference and shared reference to the same object?*
2. *What do you think would happen if we attempted to modify the vector while we have a slice that borrows a reference to it? Experiment by calling `.clear()` on the vector (after declaring it mutable). See example below. Run it and see what happens. Can you explain why the compiler is returning an error and the meaning of that error?*
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
3. *You will find that certain methods for manipulating the elements of a vector such as sorting are available only on the slice type and not the vector. However, if you call `.sort` on a vector, it will still work. Why is that? Hint: when method calls are made in Rust, it not only accesses the method on the specific data type, but any methods on the data type that it dereferences to as indicated by the `DeRef` trait implementation. So what does a vector dereference to? Can you find the relevant trait implementation?* <br/>
https://doc.rust-lang.org/std/vec/struct.Vec.html <br/>
https://doc.rust-lang.org/std/primitive.slice.html#method.sort <br/>


### Solution
1. Let's go through a simple example:
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = &v;
    let aside = v; // vector data on the heap is now moved into `aside` and `v` becomes uninitialized
    r[0] // r still points to v, which doesn't point to anything and so is a dangling pointer
}
```
If we have a shared reference to some variable and that variable goes out of scope or becomes uninitialized, we would end up with a dangling pointer. This can lead to unexpected behavior in a program and so Rust will attempt to avoid these possibilities. This is what the compiler will complain:
```console
error[E0505]: cannot move out of `v` because it is borrowed
  --> src/main.rs:16:17
   |
14 |     let v = vec![1, 2, 3, 4, 5, 6];
   |         - binding `v` declared here
15 |     let r = &v;
   |             -- borrow of `v` occurs here
16 |     let aside = v; // vector data on the heap is now moved into `aside` and `v` becomes uninitialized
   |                 ^ move out of `v` occurs here
17 |     r[0]; // r still points to v, which doesn't point to anything and so is a dangling pointer
   |     - borrow later used here
```
In other words, Rust will complain and enforce the rule that we cannot make any changes to `v` while it is being borrowed as an immutable, shared reference. This will prevent a case of a dangling pointer.

2. We'll get a similar error enforcing the same rule:
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
Remember, our slice which is of the type, `&[u8]` borrows a reference to the heap data, which is owned by `transaction_bytes`. However, by calling `clear` we are modifying the underlying heap data while we still have a shared reference to it. Rust does not allow this. 

3. In Rust, the `.` operator which calls a method on an object will follow as many references as it takes to find its target. For example, consider the following code:
```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let r = &v;
    let rr = &r;
    let rrr = &rr;
    println!("{}", rrr.len()); // print length of the vector
}
```
This will work and print the vector's length even though `rrr` is a reference of a reference of a reference to the vector. The `.` operator will actually look at what this object dereferences to and then call `len` on the appropriate object which is our `vec`. 

This is actually the equivalent of the following:
```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let r = &v;
    let rr = &r;
    let rrr = &rr;
    println!("{}", (***rrr).len()); // print length of the vector
}
```

Now the interesting thing is that the vector itself dereferences to a slice. This is because it implements the [`Deref Mut` trait](https://doc.rust-lang.org/std/ops/trait.DerefMut.html). Here is the [trait implementation](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#2711): 
```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}
```

So when we call `v.sort()`, this is equivalent of calling `*v.sort()` which is the slice and calling the `.sort()` method on it.
```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    v.sort(); // dereferences to a slice and calls the `sort` slice method
}
```