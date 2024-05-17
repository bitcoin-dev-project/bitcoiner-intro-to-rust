# 8 Solution

### Quiz
*Consider the following block of code in which we create a Vec and then attempt to print it out:*
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
If we run this and look at the compiler error, we'll get a better sense of what's going on here.
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

Let's attempt to implement the `Display` trait for a `vec`:
```rust
use std::fmt;

impl Display for Vec<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Values:\n")?;
        for v in &self {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec: Vec::<u8> = vec![0, 0, 0, 0, 0];
    println!("Vec: {}", vec);
}
```

The basic idea is that we leverage the `write!` macro which takes the `Formatter` instance and writes some information to it. If any step fails, an error will be returned. Otherwise, if we iterate through the vector and are able to write all values successfully we can simply return the `Ok(())` result. This might still be a bit confusing at this stage, so consider coming back to revisit this solution after you've gone through the course. 
