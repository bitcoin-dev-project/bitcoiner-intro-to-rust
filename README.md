# The Bitcoiner's Introduction to Rust
*Learn Rust the ~~Hard~~ Bitcoin Way*

### Goal
Learn Rust and review Bitcoin fundamentals by building a command-line program.

### Project
We are going to build a relatively small command line program that decodes a raw transaction. Don't worry if you don't exactly know what that means, we'll review all the basic Bitcoin concepts and walk you through it step by step. If you are familiar with `bitcoin-cli`, this is the equivalent of `bitcoin-cli decoderawtransaction [raw transaction hex]`.

### Prerequisites
Ideally, you have some experience with programming and have read Mastering Bitcoin. This is not required, but it will make the learning process a lot smoother.

### Why Rust
Rust is becoming very popular for new Bitcoin projects. Some examples of open source projects being built in Rust include BDK, LDK, Fedimint, to name just a few. If you are passionate about Bitcoin and want to build on the future of money, this is a valuable skill to have.

### But Rust is Hard
I promise it's not as difficult as you might think. It just takes some time to get familiar with a new way of doing things. At first, it might feel more restrictive and tedious to write Rust code, but this is because the language makes certain tradeoffs that yield many long-term benefits in terms of performance, safety, effectiveness and readability. The compiler forces you to reason more carefully about your program and what it's actually doing under the hood. My hope is that by the end of this project you come to appreciate Rust and feel more confident writing Rust programs. I transitioned from Ruby and Javascript to Rust and I can't see myself ever going back!

### What You Will Learn
By the end of this project, you will have understood the fundamental concepts of Rust such as types, data structures, references, lifetimes, stack vs the heap, traits, error handling and more.

### Table of Contents
* [1.0: Background](01_background.md)
* [2.0: Setup](02_setup.md)
* [3.0: Our First Function](03_our_first_function.md)
* [4.0: Hex and Bytes](04_hex_and_bytes.md)
* [5.0: Vectors and the Result Type](05_vectors_and_results.md)
* [6.0: Pointers](06_pointers.md)
* [7.0: Arrays and Type Conversions](07_arrays_and_conversions.md)
* [8.0: Traits and Reading Bytes](08_traits_and_reading_bytes.md)
* [9.0: References and Borrowing Part 1](09_references_and_borrowing_01.md)
* [9.1: References and Borrowing Part 2](09_references_and_borrowing_02.md)
