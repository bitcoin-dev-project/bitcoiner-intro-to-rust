# 8 Solution

### Quiz
1. *Take another look at the `Read` trait and the implementation of the `Read` trait for a slice in the documentation. What are the required and provided methods for the trait? What provided methods are being overwritten by the slice?*

2. *Consider the following block of code in which we create a Vec and then attempt to print it out:*
```rust
fn main() {
    let vec: Vec::<u8> = vec![0, 0, 0, 0, 0];
    println!("Vec: {}", vec);
}
```
*The compiler will return an error that the Vec cannot be formatted with the default formatter.*
*1. Which trait is not implemented for the Vec that is required for it to be printed?*
*2. How else can you print out the vector for debugging purposes?*
*3. Try and implement the correct trait for Vec so that it can be printed for standard display purposes.*

### Solution
1. According to the [`Read` trait documentation](https://doc.rust-lang.org/std/io/trait.Read.html), `read` is the only required method. The rest, `read_vectored`, `is_read_vectored`, `read_to_end`, `read_to_string`, `read_exact`, `read_buf`, `read_buf_exact`, `by_ref`, `bytes`, `chain`, and `take` are all provided methods with default implementations. The provided methods that the [slice](https://doc.rust-lang.org/src/std/io/impls.rs.html#233-323) overwrites are `read_vectored`, `is_read_vectored`, `read_to_end`, `read_to_string`, `read_exact` and `read_buf`. 

2. If we run this and look at the compiler error, we'll get a better sense of what's going on here.
```console
   Compiling playground v0.0.1 (/playground)
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:25
  |
3 |     println!("Vec: {}", vec);
  |                         ^^^ `Vec<{integer}>` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

The trait that is not implemented is the [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait, which is defined as:

```rust
pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

For now, if we wanted to print a `vec` we would have to use `{:?}` inside the println! string slice argument. This will print the `Debug` output and a `vec` does implement `Debug`. 

Let's attempt to implement the `Display` trait for a `vec` by implementing the required method `fmt`:
```rust
use std::fmt::{self, Display, Formatter};

struct CustomVec<T>(Vec<T>);

impl Display for CustomVec<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Values:\n")?;
        for v in &self.0 {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec: CustomVec<u8> = CustomVec(vec![0, 0, 0, 0, 0]);
    println!("Vec: {}", vec);
}
```

The basic idea is that we leverage the `write!` macro which takes the `Formatter` instance and writes some information to it. If any step fails, an error will be returned (we'll talk more about error handling and the `?` operator in chapter 19). If we iterate through the vector and are able to write all values successfully we can simply return the `Ok(())` result, which matches the the expected result type `fmt::Result`. We need to create a `CustomVec<T>` using the [Newtype Pattern](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) to implement external traits on external types.

This might still be a bit confusing at this stage, so consider coming back to revisit this solution after you've gone through the course.