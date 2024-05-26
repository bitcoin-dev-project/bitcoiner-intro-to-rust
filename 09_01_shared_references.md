# Shared References

In addition to a *mutable* reference, as indicated by the `&mut` keyword, we can also pass around a *shared* reference to a variable, which can be done by simply prefacing a variable with the `&` symbol.
A mutable reference can modify the underlying data where as the shared reference cannot and is read-only.
We already saw an example of a shared reference in chapter 6 where we created a slice reference to data on the heap by prepending `[u8]` with the `&` symbol.

A reference is a kind of pointer.
It points to some data elsewhere and "borrows" it instead of "owning" it.
What does this mean?
Well let's see with an [example](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing):

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

You can run this code online with [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf786ab3bacd43e260e73ae5efa49d50).
If you run it, you'll notice that there will be a compiler error:
```console
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:6:43
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |
4 |     let len = calculate_length(s1);
  |                                -- value moved here
5 |
6 |     println!("The length of '{}' is {}.", s1, len);
  |                                           ^^ value borrowed here after move
```

The error points out that a move occurs and that we are unable to "borrow" the value after a move.
It also mentions that the type, `String` does not implement the `Copy` trait.

By default, any type that does not implement the `Copy` trait will get "moved" when passed as an argument.
This means the variable that contains it will no longer be available within the scope of original function.
It's "ownership" is now passed to the function being called, which in this case is `calculate_length`.

Why does Rust do this?
Well this has to do with how Rust handles memory management internally.
This way, it's always clear which variable "owns" data so that when that variable goes out of scope, the memory associated with it can be automatically freed.
This is different from how other languages handle memory with garbage collection or reference counting to know when memory should be freed.

Since `s1` was moved to the scope of `calculate_length`, the `String` data will be deallocated when the function returns.
That's why we can't print it when we are back in `main`, the data would not exist anymore.
If we want to keep referring to the string after it has been passed to the `calculate_length` function, we need to pass a *reference* as argument instead.
The reference will "borrow" the value and won't actually "own" the underlying data.
This means that when the reference goes out of scope and is no longer in use, the heap data and its owner will still remain.

We can create a reference by placing the `&` symbol in front of an identifier and modifying the function argument type:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This will work now and correctly print:
```console
The length of 'hello' is 5.
```

`s1` remains the owner of the String.
`s` in the `calculate_length` will merely borrow a shared, immutable reference to the String.
This means that we wouldn't be able to mutate the String in `calculate_length`.
For example this won't work, 

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.pop();
    s.len()
}
```

There will be a compiler error:

```console
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
  --> src/main.rs:10:5
   |
10 |     s.pop();
   |     ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
9  | fn calculate_length(s: &mut String) -> usize {
   |                         +++
```

Let's try that suggestion:

```rust
fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    s.pop();
    s.len()
}
```

That will work now! And see what the `main` function will print:

```console
The length of 'hell' is 4.
```

That's the basics of shared and mutable references! 

It's important to point out here that there is an exception to this rule, which we alluded to earlier and that is any type that implements the `Copy` trait, such as an integer type or an array `[u8; N]`.
Typically, these are types that are stack allocated and don't require any heap allocations.

Let's take a look at an example (see [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bbc4ad315069566df4b5d853280492d1)):

```rust
fn main() {
    let arr = [0, 1, 2, 3];

    let len = calculate_len(arr);
    println!("The len of {:?} is {}.", arr, len);
}

fn calculate_len(mut arr: [u8; 4]) -> usize {
    arr.len()
}
```

This will run just fine without needing to pass a shared reference.
Why? Because a copy will automatically be made in the `calculate_len` function and Rust won't have to worry about or think about ownership of data on the heap.

### Single Writer or Multiple Readers
Rust enforces a simple, yet important rule when it comes to passing references and that is single writer OR multiple readers.
In other words, you can have many different immutable, shared references to an object OR you can have just one mutable reference at any given time.
You can't have both a shared reference and a mutable reference at the same time.

Let's walk through an example to see why:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = &v;
    let aside = v; // vector data on the heap is now moved into `aside` and `v` becomes uninitialized
    r[0] // r still points to v, which doesn't point to anything and so is a dangling pointer
}
```
The statement `let aside = v` *moves* the vec from `v` into `aside`.
However, `r` still references the variable `v` which is now uninitialized.

If we have a shared reference to some variable and that variable goes out of scope or becomes uninitialized, we would end up with a dangling pointer.
This can lead to unexpected behavior in a program and so Rust will attempt to avoid these possibilities.

This is what the compiler will complain:
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
In other words, Rust will complain and enforce the rule that we cannot make any changes to `v` while it is being borrowed as an immutable, shared reference.
This will prevent a case of a dangling pointer.

You may see this compiler error from time to time.
Just remember the rule: you can only have a single writer (mutable reference) OR multiple readers (shared references).

Ok! That's it for references!
Take a breather as you just got past one of the hardest aspects of understanding the Rust programmming language.
Don't worry if it hasn't fully "sunk in" yet.
This is something that takes time and practice to get familiar with.
Just know that with time these concepts will make more sense and you might even begin to start appreciating them.

### Quiz
*What do you think would happen if we attempted to modify the vector in our project while we have a slice that borrows a reference to it?
Experiment by calling `.clear()` on the vector (after declaring it mutable).
See example below.
Run it and see what happens.
Can you explain why the compiler is returning an error and the meaning of that error?*
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

<hr/>

<div>
    <p align="right"><a href="10_compact_size_unsigned_integers.md">>>> Next Lesson: CompactSize Unsigned Integers</a></p>
</div>
