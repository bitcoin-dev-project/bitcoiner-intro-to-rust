# The Bitcoiner's Introduction to Rust
*Learn Rust the ~~Hard~~ Bitcoin Way*

---

*This course is now available in video format with recorded coding sessions, more visual diagrams, quizzes and extra practice problems! And still free! Check it out at http://btcdemy.thinkific.com*

---

### Goal
Learn Rust and review Bitcoin fundamentals by building a command-line program.

### Project
We are going to build a relatively small command line program that decodes a raw transaction. Don't worry if you don't exactly know what that means, we'll review all the basic Bitcoin concepts and walk you through it step by step. If you are familiar with `bitcoin-cli`, this is the equivalent of `bitcoin-cli decoderawtransaction [raw transaction hex]`.

### Prerequisites
Ideally, you have some experience with programming and have read Mastering Bitcoin. This is not required, but it will make the learning process a lot smoother.

### Why Rust
Rust is becoming very popular for new Bitcoin projects. Some examples of popular open source Bitcoin projects being built in Rust include [Rust-Bitcoin](https://github.com/rust-bitcoin/rust-bitcoin), [BDK](https://github.com/bitcoindevkit/bdk), [LDK](https://github.com/lightningdevkit/rust-lightning), and [Fedimint](https://github.com/fedimint/fedimint). But it's not just Bitcoin. For 8 years in a row, Rust topped the charts as the most desired programming language in Stack Overflow's annual developer survey (https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/). Why is that? Well, Rust allows developers to write systems-level programs in a way that is extremely performant, memory-safe, reliable and readable. It has great documentation, tooling, and a large and growing ecosystem. These are all very important features, especially for an important protocol like Bitcoin which will ultimately run the entire world's money system! 

### But Rust is Hard
It's not as difficult as you might think. It just takes some time to get familiar with a new way of doing things. At first, it might feel more restrictive and tedious to write Rust code, but this is because the language makes certain tradeoffs that yield many long-term benefits in terms of performance, safety, effectiveness and readability. The compiler forces you to reason more carefully about your program and what it's actually doing under the hood. My hope is that by the end of this project you come to appreciate Rust and feel more confident writing Rust programs. I transitioned from Ruby and Javascript to Rust and I can't see myself ever going back!

### What You Will Learn
By the end of this project, you will have understood the fundamental concepts of Rust such as types, data structures, references, stack and the heap, traits, error handling and more. We will also review some basic aspects of the Bitcoin protocol, specifically the components of a transaction. In future courses, we will dive deeper into the Bitcoin protocol and write more complex programs, exploring the full extent of Bitcoin's capabilities. 

### Table of Contents
* [1.0: Project Overview](01_project_overview.md)
* [2.0: Setup](02_setup.md)
* [3.0: Our First Function](03_our_first_function.md)
* [4.0: Hex and Bytes](04_hex_and_bytes.md)
* [5.0: Vectors and the Result Enum](05_vectors_and_result_enum.md)
* [6.0: Pointers and Slices](06_pointers_and_slices.md)
* [7.0: Arrays and Type Conversions](07_arrays_and_conversions.md)
* [8.0: Traits and Reading Bytes](08_traits_and_reading_bytes.md)
* [9.0: Mutable References](09_00_mutable_references.md)
* [9.1: Shared References](09_01_shared_references.md)
* [10.0: CompactSize Unsigned Integers](10_compact_size_unsigned_integers.md)
* [11.0 Unit Testing](11_unit_testing.md)
* [12.0 Reading Inputs and Type Coercion](12_reading_inputs_and_type_coercion.md)
* [13.0 Structs](13_structs.md)
* [14.0 JSON Serialization](14_json_serialization.md)
* [15.0 Reading Outputs and Tuple Structs](15_reading_outputs_and_tuple_structs.md)
* [16.0 Custom Serialization and Generic Functions](16_custom_serialization_and_generic_functions.md)
* [17.0 File Organization and Modules](17_file_organization_and_modules.md)
* [18.0 Decoding a Legacy Transaction](18_decoding_legacy_transaction.md)
* [19.0 Error Handling](19_error_handling.md)
* [20.0 Command Line Arguments](20_command_line_arguments.md)
* [21.0 Refactoring and the Rust-Bitcoin Library](21_refactoring_and_rust_bitcoin.md)
* [22.0 Decoding Segwit](22_decoding_segwit.md)
* [Quiz Solutions](quiz_solutions/)
