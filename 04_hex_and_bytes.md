# Hex and Bytes

Alright, we have some working code and are printing a version number, but this is hardcoded. We want to actually extract the version from the given raw transaction and properly implement the `read_version` function. But before we do so, we need to review a few key concepts.

### Hexadecimal Format
We mentioned earlier that transactions are represented as a string of hexadecimal characters. But what does "hexadecimal" mean exactly? Well, it's a number system, but instead of being base 10, with digits 0 through 9, it is base 16 with digits 0-9 and characters a-f. So the integer 10 in a hexadecimal format is 0a (can also be written as 0xa). Mathematically, we can see that `0a` converts to 10 using base math: `0 * 16^1 + 10 * 16^0 = 10`. If we wanted to express the integer 71, we know the math would be `4 * 16^1 + 7 * 16^0`. So 71 is represented as `47` in hexadecimal format.

### Bytes
What is a byte? A byte is a unit of computer memory. It holds 8 bits of data. Each bit is the most basic computing unit and represents a logical state of one of two values. Therefore the maximum number of states we can hold in 1 byte of data is 2^8 or 256. Each state can be expressed as a base 10 integer from 0 to 255 (256 total states). Rust provides us with a useful data type to represent a byte, the `u8`, which is an unsigned 8 bit integer. This will come in handy for our project. https://doc.rust-lang.org/std/primitive.u8.html

### Converting Hexadecimal to Bytes
Remember, if we want to extract the version from a transaction, we want the first 4 bytes. However, our program isn't given the bytes data in decimal format, but in hexadecimal format. As it turns out, every two hexadecimal characters represents one byte. This is because the maximum value for two hexadecimal characters is `ff` which translates to 255 using base 16 math: `15 * 16^1 + 15 * 16^0`. So we can actually just look at our raw transaction and look at the first 8 characters (first 4 pairs of hexadecimal characters) to see what the version is. For example, from the previous lesson's transaction we see:

<ins><i>01000000</i></ins>01c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000

We can convert `01000000` to an integer using base 16 math. But at a quick glance this looks like a much bigger integer than `1` doesn't it? After all, the `01` seems to be in the high position. Wouldn't we expect this to look more like `00000001` so that it is correctly intepreted as a version 1 transaction? (*Currently Bitcoin only supports version 1 and version 2 transactions*) Well, this gets us into the topic of endianness.

### Little Endian vs Big Endian
Bytes can be stored in two different orders. When bytes are stored as little-endian, the least significant byte is stored first (in the lowest memory address). When bytes are stored as big-endian, the most significant byte is stored first. So when we look at a transaction, the version is defined by the protocol as being 4 bytes in *little endian*. This means the least significant position appears first. What do we mean by *position*? Well let's look at decimal numbers for comparison.

When looking at decimal numbers we typically read it from left to right with the most sigificant digit first to the least significant digit last. For example, in the number, `201`, the most significant digit is `2` and the least significant digit is `1`. But in the case of this transaction version data, the least significant byte appears first. So we'd have to take this into account when doing base math. One way to do this is to simply reverse the order of the bytes and then do the base math we're familiar with. For example `01 00 00 00` can be reversed and become `00 00 00 01`. Now, we can do our normal base math which is `0* 16^7 + ... + 1 * 16^0`. We don't need to write all of it out since the other values will just be zero. Therefore the version number is just `1 * 16^0` which is `1`.

We can also convert each byte to its binary representation and then do some math to convert the binary number to a decimal number. For example `01` in hexadecimal is `00000001` in binary (8 bits). Remember, binary is just a number with base 2. So we can do base 2 math. `0 * 2^7 + ... + 1 * 2^0`. This also equals 1.

### Quiz
*What is the version number indicated by the following 4 bytes little endian in hexadecimal format: `10000000`?*

### Other Helpful Resources
* Check out this great site, https://learnmeabitcoin.com, for tutorials that discuss these topics. Hexadecimal overview: https://learnmeabitcoin.com/technical/hexadecimal. Little endian overview: https://learnmeabitcoin.com/technical/little-endian


----------------------------------------------------------------------------------------------------------------------------------------------------

<div style="text-align: right">
    <a href="05_vectors_and_results.md">>>> Next Lesson: Vectors and Results</a>
</div>
