# 7 Solution

### Quiz
*Can we easily convert an `array` into a `vec` similar to how we converted a `slice` into an `array` above? If so, how does it work?*

### Solution
Indeed, we can, using a similar `into()` method. If we look at the [documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-From%3C%26%5BT;+N%5D%3E-for-Vec%3CT%3E), we can see that `From<&[T; N]>` is implemented for `Vec<T>`. Note that the `From` trait is different from `TryFrom` in that it returns the expected data type and does not wrap it in a `Result`. This is because the conversion is not likely to fail. So because the `From` trait implementation is there, we should be able to call the `from` and `into` methods like so:

```rust
fn main() {
    let array = [1_u8, 2_u8, 3_u8];
    let vec1 = Vec::from(&array);
    let vec2: Vec<u8> = array.into();
}
```
