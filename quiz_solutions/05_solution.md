# 5 Solution

### Quiz
*Notice the last line of this function. What will the compiler complain is wrong with this function? And why?*
```rust
fn read_version(transactionhex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transactionhex);
    1;
}
```

### Solution
Notice how there's no `return` statement at the bottom of the function. Rust will automatically return the last expression *without* a semicolon. However, if you look closely, there is a semicolon after the `1`. This means there is nothing being returned from our function. In this case, Rust assumes that you mean to return an empty [tuple](https://doc.rust-lang.org/std/primitive.tuple.html) type, `()`. So our function body is expected to return an empty tuple, but our function signature states that it must return a `u32` type. This `mismatch` is what produces a compile error.
```console
error[E0308]: mismatched types
 --> src/main.rs:1:43
  |
1 | fn read_version(transaction_hex: &str) -> u32 {
  |    ------------                           ^^^ expected `u32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
...
4 |     1;
  |      - help: remove this semicolon to return this value
```
