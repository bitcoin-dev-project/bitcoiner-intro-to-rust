# Structs

A struct is a fairly straightforward type.
It's similar to classes in OOP languages, but there's no concept of inheritance or polymorphism here.
It simply allows you to group together related pieces of data - which can all be of different types - into a single name.
At a high level, we first define the struct by providing it's name, the names of the different fields and their associated types.
We then create an instance of the struct by specifying the values for each field.
We can also add methods to a struct by defining functions within an `impl` block.

Let's create an input struct to group together our related input fields.
We typically place this at the top of our file or we include it from another file:

```rust
use std::io::Read;

struct Input {
    txid: [u8; 32],
    output_index: u32,
    script: Vec<u8>,
    sequence: u32,
}
```

And all we have to do is create an instance of it in our `main` function:

```rust
...
    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);

        let input = Input {
            txid: txid,
            output_index: output_index,
            script: script,
            sequence: sequence,
        };
    }
...
```

Pretty simple right?
One neat modification we can make here is remove the key/value pair and just put a comma separated list of field names.
We can do this anytime the field names match the variable names:

```rust
...
    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);

        let input = Input {
            txid,
            output_index,
            script,
            sequence,
        };
    }
...
```

Let's change this up so that we have an `inputs` vector and we'll `push` each decoded input into that.
Remember, we have to declare it as *mutable*! We can declare a new vec in two ways, using a macro `vec![]` or calling the new method, `Vec::new()`.

```rust
...
    let mut inputs = vec![];

    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);

        inputs.push(Input {
            txid,
            output_index,
            script,
            sequence,
        });
    }
...
```

Now that we have a collection of `inputs`, let's try printing out the debug output:

```rust
...
    println!("Version: {}", version);
    println!("Inputs: {:?}", inputs);
...
```

Try running `cargo run` and see what happens.
We'll get a compiler error:
```console
error[E0277]: `Input` doesn't implement `Debug`
  --> src/main.rs:89:30
   |
89 |     println!("Inputs: {:?}", inputs);
   |                              ^^^^^^ `Input` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Input`
   = note: add `#[derive(Debug)]` to `Input` or manually `impl Debug for Input`
   = help: the trait `Debug` is implemented for `Vec<T, A>`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Input` with `#[derive(Debug)]`
   |
9  + #[derive(Debug)]
10 | struct Input {
   |
```

We're trying to print out a collection of `Input` structs, but that struct does not implement the `Debug` trait, so we can't get a printout.
In fact, it doesn't implement the `Debug` or the `Display` traits so how do we print it out? One way would be to manually implement the `Debug` trait ourselves.

Let's try that first.
We'll follow the example from the docs here for manual implementation: https://doc.rust-lang.org/std/fmt/trait.Debug.html#examples.
Don't forget to add `use std::fmt` at the top of the file to bring the `fmt` module into scope.

```rust
impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Input")
         .field("txid", &self.txid)
         .field("output_index", &self.output_index)
         .field("script", &self.script)
         .field("sequence", &self.sequence)
         .finish()
    }
}
```

Some of this syntax might feel a bit unfamiliar, so let's take a moment to go through it.
The key thing to understand is that we have to provide the `fmt` function implementation.
This is the function that will get called when we try to print the debugging printout.
It will be called with an argument type that is a `Formatter` (we'll ignore the `<'_>` syntax for now).

The `Formatter` has a method called [`debug_struct`](https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct) that we can use and pass in the various fields that we want to display for debugging purposes.
The field method takes in a field name to display along with the value.
The value itself must be something that implements the `Debug` trait.
Luckily, all of our fields do.
At the end, we call `finish` which returns the required `fmt::Result`.

If you try running `cargo run` now, this should work.
It won't be very pretty or easy to read though.

```shell
Version: 1
Inputs: [Input { txid: [248, 198, 147, 119, 27, 41, 146, 161, 27, 83, 192, 69, 54, 154, 179, 26, 146, 13, 225, 217, 33, 255, 60, 20, 138, 157, 4, 135, 200, 249, 11, 175], output_index: 16, script: [72, 48, 69, 2, 33, 0, 144, 74, 46, 14, 143, 89, 127, 193, 204, 39, 27, 98, 148, 176, 151, 166, 237, 201, 82, 227, 12, 69, 62, 53, 48, 249, 36, 146, 116, 151, 105, 168, 2, 32, 24, 70, 76, 34, 91, 3, 194, 135, 145, 175, 6, 188, 127, 237, 18, 157, 202, 174, 255, 158, 200, 19, 90, 218, 31, 177, 23, 98, 206, 8, 30, 169, 1, 65, 4, 218, 40, 145, 146, 176, 132, 93, 91, 137, 206, 130, 102, 93, 136, 172, 137, 215, 87, 207, 197, 253, 153, 123, 29, 232, 174, 71, 247, 120, 12, 230, 163, 34, 7, 88, 59, 116, 88, 209, 210, 243, 253, 107, 58, 59, 132, 42, 234, 158, 183, 137, 226, 190, 165, 123, 3, 212, 14, 104, 77, 142, 30, 5, 105], sequence: 4294967295 }, Input { txid: [229, 29, 33, 119, 51, 43, 175, 249, 207, 187, 192, 132, 39, 207, 13, 133, 210, 138, 253, 200, 20, 17, 205, 187, 132, 244, 12, 149, 133, 139, 8, 13], output_index: 1, script: [72, 48, 69, 2, 32, 54, 157, 247, 212, 39, 149, 35, 158, 171, 249, 212, 26, 238, 117, 227, 255, 32, 82, 23, 84, 82, 43, 208, 103, 137, 15, 142, 237, 246, 4, 76, 109, 2, 33, 0, 154, 207, 189, 136, 213, 29, 132, 45, 184, 122, 185, 144, 164, 139, 237, 18, 177, 248, 22, 233, 85, 2, 208, 25, 142, 208, 128, 222, 69, 106, 152, 141, 1, 65, 4, 224, 236, 152, 138, 103, 153, 54, 206, 168, 10, 136, 230, 6, 61, 98, 220, 133, 24, 46, 84, 138, 83, 95, 174, 205, 110, 86, 159, 181, 101, 99, 61, 229, 180, 232, 61, 90, 17, 251, 173, 139, 1, 144, 140, 231, 30, 3, 116, 176, 6, 216, 70, 148, 176, 111, 16, 189, 193, 83, 202, 88, 165, 63, 135], sequence: 4294967295 }]
```

We'll talk more about returning a prettier hex encoded display for the end user in the next lesson.
For now, we can make a nice change to our code so that we don't have to write out the `impl Debug` for our struct.
Instead of doing all that, we can actually just add an attribute to our struct.
This is basically metadata that will tell the compiler to auto-generate the implementation for a particular **derivable** trait.
As long as all the fields of the struct implement the `Debug` trait, this will work.

Let's remove the `impl` block and the `use std::fmt` statement and just add this attribute to our struct:

```rust
#[allow(dead_code)]
#[derive(Debug)]
struct Input {
    txid: [u8; 32],
    output_index: u32,
    script: Vec<u8>,
    sequence: u32,
}
```

And voila! That works now and is much cleaner isn't it?
The attribute `#[allow(dead_code)]` is a hint to the compiler that we know we are not using the struct fields right now, but we will in the next section.

Now that we understand a bit more about structs and trait implementations, let's work on displaying this information in a more readable way.
Rather than print out the debugging output, we'll instead **serialize** the data according to a standard JSON format and display that.

### Additional Reading
* Defining Structs: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
* Struct Method Syntax: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
* Debug: https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
* Derive: https://doc.rust-lang.org/rust-by-example/trait/derive.html

<hr/>

<div>
    <p align="right"><a href="14_json_serialization.md">>>> Next Lesson: JSON Serialization</a></p>
</div>
