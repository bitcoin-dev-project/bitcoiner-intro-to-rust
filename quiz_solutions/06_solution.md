# 6 Solution

### Quiz
*How is a `String` implemented in Rust? How is it different from a string slice `&str`?*

### Solution
A `String`, like a `Vec` is a smart pointer to data on the heap. Like a `Vec`, it owns the data on the heap and contains additional metadata such as the length and capacity. In addition, it enforces that all the data it holds are valid `UTF-8` characters. A string slice, on the other hand, is a fat pointer reference. It points to and merely borrows data on the heap that the `String` type owns. 

This section from the online Rust book has a good explanation and helpful diagram: https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

<img src="https://doc.rust-lang.org/book/img/trpl04-06.svg" width=250>