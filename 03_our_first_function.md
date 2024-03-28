# Our First Function

Let's not worry at the moment about how our project is structured. We'll come back to that. Let's not even worry about how to accept and read a command line argument. Let's start with a very simple approach. We'll write another function called `extract_version` which will receive the transaction text in hexademical format and return the version. For now, we won't actually implement the code for returning the version, we'll just return 1.

Assuming you come from another language such as Python or Ruby or Javascript, you might want to write something like this and see what happens:

```
fn extract_version(transactionhex) {
    return 1;
}

fn main() {
    version = extract_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");
    println!("Hello, world!");
}
```

Let's see what happens if we run the command `$ cargo run`. Again remember to run this command from the root directory. The cargo command needs to look for the `Cargo.toml` to be able to compile and execute the program. 

As you will see, the program will fail to compile and we'll get a number of different errors here. The first is that it complains about the `transactionhex` function argument and offers some suggestions for fixing it. 
```
1 | fn extract_version(transactionhex) {
  |                                  ^ expected one of `:`, `@`, or `|`
```

It's expecting some more information about the argument. One of the suggestions that's relevant to us is this one:
```
help: if this is a parameter name, give it a type
  |
1 | fn extract_version(transactionhex: TypeName) {
  |                                  ++++++++++
```

Remember, Rust is a *statically typed* language. You have to explicitly provide the type of data you expect the function arguments to be. Is it an integer? What kind of integer? Is it a string? An array? A vector? You can read up more on Rust's data types here: https://doc.rust-lang.org/book/ch03-02-data-types.html.

The argument is obviously a text data type of some sort. In other languages we might call this a `string`, but in Rust the `String` data type means something more nuanced. We won't go into that here just yet. Let's just say that when you enter some text inside quotes, Rust interprets that as the type `&str`.

So let's specify the function argument by doing the following: 
```
fn extract_version(transactionhex: &str) {
    return 1;
}
```

And let's see what happens when we call `$ cargo run` again.

Looks like that's no longer an issue. The next error now is pretty easy to handle.
```
error[E0425]: cannot find value `version` in this scope
 --> src/main.rs:6:5
  |
6 |     version = extract_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732...
  |     ^^^^^^^
```

The compiler is looking for the `version` declaration somewhere. But what we really mean is to declare a new variable. Any time we are declaring a new variable to use, we must use the `let` keyword. This is not needed if we are referring to variable that has already been declared. It should look something like this:

`let version = extract_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");`

Ok, `let`'s do that, rerun `$ cargo run` and see what happens.

That seems to work and we now get a different error. This last error we get is related to mismatched types. 
```
error[E0308]: mismatched types
 --> src/main.rs:2:12
  |
1 | fn extract_version(transactionhex: &str) {
  |                                         - help: try adding a return type: `-> i32`
2 |     return 1;
  |            ^ expected `()`, found integer
```

In keeping with the theme of providing explicit types for everything, you have to do this for function return types as well. The compiler needs to know what the function is expected to return so that it can enforce that. 

With the function we've written, we didn't tell the compiler exactly what type the function should return. When this is not provided, Rust interprets that you mean it will return the empty tuple type, `()`. In other words, this function is the equivalent of the following:
```
fn extract_version(transactionhex: &str) -> () {
    return 1;
}
```

So the compiler is complaining because we've told it that it should expect an empty tuple as the return type, but we are instead returning an integer type. So let's fix that. 

For now, let's assume that we're returning an integer, which is an unsigned 32 bit integer. I chose this particular type for a reason. Remember, if we refer back to chapter 6 of Mastering Bitcoin, we know the version is represented as 4 bytes. And each byte is 8 bits. So we know the maximum size of the version is 32 bits. "Unsigned" means the number is always positive. We don't have negative version transactions. 

Rust provides the `u32` data type as a primitive type, meaning it's available out-of-the-box and we don't need to use any external libraries. So let's use that and see what happens when we run `$ cargo run` again. 

Ok! Looks like the program is compiling successfully now and still printing `Hello, world!`! That's great. Let's make one final change. We'll modify the `println!` function to print the version. 
```
fn main() {
    let version = extract_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");
    
    println!("Version: {}", version);
}
```

`println!` is actually a `macro` not a Rust function. Macros are a form of metaprogramming, in which we write code that writes other code. We don't need to go into detail with how this works at the moment, just something to be aware of. A macro can also take a variable number of parameters. In the case of `println!`, we can place many different brackets, `{}`, inside the quotes of the first argument, and it will replace each bracket with an argument from the following comma separated list of arguments. For example:

`println!("{}, {}!", "Hello", "world")`

This will print `Hello, world!`. You can read up more on the differences between macros and functions here: https://doc.rust-lang.org/book/ch19-06-macros.html#the-difference-between-macros-and-functions

Ok, so if we run `$ cargo run` one last time, we can now see that our program prints out the version! 
```
Version: 1
```

Look at that! You've written some Rust code and are starting to get the hang of the basics. That wasn't so bad was it? 

You battled with the compiler and won. For now, bask in your victory. You are well on your way to becoming a master of both Bitcoin and Rust. Do not fear the compiler. Fear is the mind-killer. Just remember to stay calm and read the errors carefully. You will figure it all out eventually.

Let's move on to the next lesson. We'll go into more detail about hexadecimal format as well as converting to and working with bytes. 

<hr/>

### [Next Lesson: Hex and Bytes](04_hex_and_bytes.md)