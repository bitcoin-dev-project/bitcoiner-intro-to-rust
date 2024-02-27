# Pointers

When we call `[]` on a vector to reference some set of elements, we are actually returning a slice and not a vector. A slice is a region of a vector or array. It is the actual collection of that data in memory. However, in Rust we typically don't store that data from the heap directly into a variable. Instead we always *refer* to that data with the use of a pointer. In the case of a slice, we must prepend the data with the `&` sign which converts it into a *fat pointer*. A fat pointer is a two-word value comprising a pointer to the slice's first element and the number of elements in the slice. 

<img src="https://www.lurklurk.org/effective-rust/images/vecslice.svg"/>
<p>source: <a href="https://doc.rust-lang.org/book/ch15-00-smart-pointers.html">https://doc.rust-lang.org/book/ch15-00-smart-pointers.html</a></p>

Notice how the vector is also a pointer type to data stored on the heap. In Rust, the vector is actually a *smart pointer* instead of a *fat pointer*. A smart pointer contains additional metadata and capabilities. It also *owns* the data instead of just borrowing a reference to it. Don't worry if you're not quite sure what that means. We're going to go explore the concepts of borrowing and references in more detail later on in the course.

So, instead of storing the slice directly in the `version_bytes` variable, we must store a reference to the slice. We can do this by adding the `&` in front as the compiler suggested. Here's our full program now. We've also added a `println!` in there to see what the version bytes looks like.

*Note: for `println!` we can insert additional characters in the brackets to modify the how the output is displayed. `{:?}` will give us the debug output. As long as the variable type implements the `Debug` trait, we can see the debugging printout for that variable.*

```
use hex;

fn extract_version(transaction_hex: &str) -> u32 {
    // convert hex to bytes
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = &transaction_bytes[0..4];
    println!("version bytes: {:?}", version_bytes);
    1
}

fn main() {
    let version = extract_version("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000");
    println!("Version: {}", version);
}
```

We can now see a printout of the bytes in addition to the version:

```
version bytes: [1, 0, 0, 0]
Version: 1
```

Great! Let's keep moving and calculate the version number from the byte collection. 

### Quiz
*How is a String implemented in Rust?*

## Additional Reading
* https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

### [Next Lesson: Arrays and Type Conversions](07_arrays_and_conversions.md)
