# Our First Function

Let's not worry at the moment about how our project is structured.
We'll come back to that.
Let's not even worry about how to accept and read a command line argument.
Let's start with a very simple approach.
We'll write a new function called `read_version` which will receive the transaction text in hexademical format and return the version.
For now, we won't actually implement the code for returning the version, we'll just return `1`.

For the example throughout our course, we're going to look at an arbitrarily selected legacy Testnet transaction, which can be viewed in detail here: https://mempool.space/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2.

Assuming you come from another programming language such as Python or Ruby or Javascript, you might be tempted to write something like this:

```rust
fn read_version(transaction_hex) {
    return 1;
}

fn main() {
    version = read_version("010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000");
    println!("Hello, world!");
}
```

Let's see what happens if we run the command `$ cargo run`.
Remember to run this command from the root directory, the cargo command has to find the file `Cargo.toml` to be able to compile and execute the program.

As you will see, the program will fail to compile and we'll get a number of different errors.
At first, it complains about the `transaction_hex` function argument and offers some suggestions for fixing it.

```console
1 | fn read_version(transactionhex) {
  |                                  ^ expected one of `:`, `@`, or `|`
```

It's expecting some more information about the argument.
One of the suggestions that's relevant to us is this one:
```console
help: if this is a parameter name, give it a type
  |
1 | fn read_version(transactionhex: TypeName) {
  |                                  ++++++++++
```

Rust is a *statically typed* language: you have to explicitly provide the type of data you expect the function arguments to be.
Is it an integer?
What kind of integer?
Is it a string?
An array?
A vector?
You can read up more on Rust's data types here: https://doc.rust-lang.org/book/ch03-02-data-types.html.

The argument is obviously a text data type of some sort.
In other programming languages we might call this a `string`, but in Rust the `String` data type means something more nuanced.
We won't go into that here just yet.
Let's just say that when you enter some text inside quotes, Rust interprets that as the type `&str`.

So, let's specify the function argument by doing the following:

```rust
fn read_version(transactionhex: &str) {
    return 1;
}
```

What happens when we call `$ cargo run` again?

Looks like that's no longer an issue.
The next error is fairly straightforward.

```console
error[E0425]: cannot find value `version` in this scope
 --> src/main.rs:6:5
  |
6 |     version = read_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732...
  |     ^^^^^^^
```

The compiler is looking for the declaration of `version` somewhere.
But what we mean is that we want to declare a new variable.
Any time we are declaring a new variable to use, we must use the `let` keyword.
This is not needed if we are referring to a variable that has already been declared.
It should look like this:

```rust
let version = read_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");
```

Ok, `let`'s do that, rerun `$ cargo run` and see what happens.

That should work now.
The last error we see is related to mismatched types.

```console
error[E0308]: mismatched types
 --> src/main.rs:2:12
  |
1 | fn read_version(transactionhex: &str) {
  |                                         - help: try adding a return type: `-> i32`
2 |     return 1;
  |            ^ expected `()`, found integer
```

In keeping with the theme of providing explicit types for everything, we have to do this for function return types as well.
The compiler needs to know what the function is expected to return so that it can enforce that.
With the function we've written, we didn't tell the compiler exactly what type the function should return.
When this is not provided, Rust assumes that you mean to return the empty tuple type, `()`.
In other words, this function is the equivalent of the following:

```rust
fn read_version(transactionhex: &str) -> () {
    return 1;
}
```

The compiler is complaining because we've told it to expect an empty tuple as the return type, but we are instead returning an integer type.
To fix that, let's assume that we're returning an unsigned 32 bit integer.
I chose this particular type for a reason: if we refer back to chapter 6 of Mastering Bitcoin, we will learn that the version field in a transaction is represented as 4 bytes.
Since each byte is 8 bits, we know the size of the version field is 32 bits.
`unsigned` means the number is always positive, we don't have negative version transactions.

Rust provides the `u32` data type as a primitive type, meaning it's available out-of-the-box and we don't need to use any external libraries.
Let's use that and see what happens when we run `$ cargo run` again.

```rust
fn read_version(transactionhex: &str) -> u32 {
    return 1;
}
```

Ok! Looks like the program is now compiles successfully and prints `Hello, world!`!
That's great.

Let's make one final change.
We'll modify the `println!` function to print the version.

```rust
fn main() {
    let version = read_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");
    
    println!("Version: {}", version);
}
```

`println!` is actually a `macro` rather than a Rust function.
Macros are a form of metaprogramming, where we write code that writes other code.
We don't need to go into too much detail with how these work right now, at least not yet! 

One useful thing about a macro is that it can take a variable number of parameters.
In the case of `println!`, we can place many different brackets, `{}`, inside the quotes of the first argument, and it will replace each bracket with an argument from the following comma separated list of arguments.
For example:

```rust
println!("{}, {}!", "Hello", "world")
```

This will print `Hello, world!`.
You can read up more on the differences between macros and functions here: https://doc.rust-lang.org/book/ch19-06-macros.html#the-difference-between-macros-and-functions

Ok, so if we run `$ cargo run` one last time, we can now see that our program prints out the version! 

```console
Version: 1
```

Look at that!
You've written some Rust code and are starting to get the hang of the basics.
That wasn't so bad was it? 

You battled with the compiler and won.
For now, bask in your victory.
You are well on your way to becoming a master of both Bitcoin and Rust.
Do not fear the compiler.
Fear is the mind-killer.
Just remember to stay calm and read the errors carefully.
You will figure it all out eventually.

Let's move on to the next lesson.
We'll go into more detail about hexadecimal format as well as converting to and working with bytes.


----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="04_hex_and_bytes.md">>>> Next Lesson: Hex and Bytes</a></p>
</div>
