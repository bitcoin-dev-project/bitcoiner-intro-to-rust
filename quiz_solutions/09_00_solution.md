# 9.0 Solution

### Quiz
*You will find that certain methods for manipulating the elements of a vector such as sorting are available only on the slice type and not the vector. However, if you call `.sort` on a vector, it will still work. Why is that? Hint: when method calls are made in Rust, it not only accesses the method on the specific data type, but any methods on the data type that it dereferences to as indicated by the `DeRef` trait implementation. So what does a vector dereference to? Can you find the relevant trait implementation?* <br/>
https://doc.rust-lang.org/std/vec/struct.Vec.html <br/>
https://doc.rust-lang.org/std/primitive.slice.html#method.sort <br/>


### Solution
In Rust, the `.` operator which calls a method on an object will follow as many references as it takes to find its target. For example, consider the following code:
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

And remember, the `*` operator is shorthand for calling the `deref` method of the `Deref` or `Deref Mut` traits.

Now the interesting thing is that the vector itself dereferences to a slice. This is because it implements the [`Deref Mut` trait](https://doc.rust-lang.org/std/ops/trait.DerefMut.html). Here is the [trait implementation](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#2769): 
```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}
```

So when we call `v.sort()`, this is similar to calling `*v.sort()` since Rust automatically follows references on method calls. `*v` will dereference to a slice and then Rust will call the `.sort` method on the slice.
```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    v.sort(); // dereferences to a slice and calls the `sort` slice method
}
```
