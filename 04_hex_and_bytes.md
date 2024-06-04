# Hex and Bytes

Alright, we have some working code and are printing a version number, but this is hardcoded.
We want to actually extract the version field from the given raw transaction and properly implement the `read_version` function.
Before we do so, we need to review a few key concepts.

### Hexadecimal Format
We mentioned earlier that transactions are represented as a string of hexadecimal characters.
But what does "hexadecimal" mean exactly?
Well, it's a number system, but instead of being base 10, with digits 0 through 9, it is base 16 with digits 0-9 and characters a-f.
The integer 10 in hexadecimal format is `a` (also written as 0xa), the integer 11 is `b`, and so on.
If we wanted to express the integer 71, the math would be `4 * 16^1 + 7 * 16^0`.
So 71 is represented as `47` in hexadecimal format.

### Bytes
What is a byte?
A byte is a unit of computer memory.
It holds 8 bits of data.
Each bit is the most basic computing unit and represents a logical state of one of two values.
Therefore the maximum number of states we can hold in 1 byte of data is 2^8 or 256.
Each state can be expressed as a base 10 integer from 0 to 255 (256 total states).
Rust provides us with a useful data type to represent a byte, the `u8`, which is an unsigned 8 bit integer.
This will come in handy for our project.
https://doc.rust-lang.org/std/primitive.u8.html

### Converting Hexadecimal to Bytes
If we want to extract the version from a transaction, we want the first 4 bytes of the bitstream.
However, our program isn't given the bytes data in decimal format, but in a hexadecimal string format.
As it turns out, every two hexadecimal characters represents one byte.
This is because the maximum value for two hexadecimal characters is `ff` which is equivalent to 255.
Using base 16 math: `15 * 16^1 + 15 * 16^0`.
So we can just look at our raw transaction and look at the first 8 characters (first 4 pairs of hexadecimal characters) to see what the version is.
For example, from the previous lesson's transaction we see:

<ins><i>01000000</i></ins>0242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000

We can convert `01000000` to an integer using base 16 math.
But at a quick glance this looks like a much bigger integer than `1` doesn't it?
Doing base 16 math as we did before will tell us it would be version 16777216.
After all, the `01` seems to be in the high value position (the most significant byte).

Currently Bitcoin only supports version 1 and version 2 transactions.
Wouldn't we expect this to look more like `00000001` so that it is correctly interpreted as a version 1 transaction?
Well, this gets us into the topic of endianness.

### Little Endian vs Big Endian
Bytes can be stored in two different orders: little vs. big endian.
When bytes are stored as little-endian, the least significant byte is stored in the lowest memory address.
That's how modern CPUs usually store numbers which make sense from a logical engineering perspective.
When bytes are stored as big-endian, it's the opposite: the most significant byte is stored in the lowest memory address.

At its heart, Bitcoin is a communication protocol.
For communication protocols, endianness is crucial to understand because bytes will be transmitted in a specific sequence.
If the receiver expects to see data in a different order from the sender, it will misinterpret that data.
Thus, communication protocols specify which order bytes will be sent.
In the Bitcoin protocol, bytes are transmitted in little-endian order.
That means that the least significant byte is sent first.

When looking at decimal numbers, humans in the West typically read it from left to right with the most significant digit first to the least significant digit last.
For example, in the number, `201`, the most significant digit is `2` and the least significant digit is `1`.
But in the case of this transaction version data, the least significant byte will appear first as we just saw.
This is a common cause of confusion when learning the Bitcoin protocol: bytes seem to be written in reverse in many parts of the system.

So, the version 1 field will be sent in the following order: `01` followed by three `00`s.
`01` is the least significant byte and is sent first.
When doing base math to convert these bytes to integers, we can conceptualize it in reverse so that the `01` is in the least significant position.
In other words, we can think of it as `00 00 00 01` and then do our normal base 16 math to convert this to an integer.

Now, we can do our normal base math which is `0* 16^7 + ... + 1 * 16^0`.
We don't need to write all of it out since the other values will just be zero.
Therefore the version number is just `1 * 16^0` which is `1`.

We can also convert each byte to its binary representation and then do some math to convert the binary number to a decimal number.
For example `01` in hexadecimal is `00000001` in binary (8 bits).
Remember, binary is just a number with base 2.
So we can do base 2 math: `0 * 2^7 + ... + 1 * 2^0`.
This also equals 1.

### Quiz
*What is the version number indicated by the following 4 bytes little endian in hexadecimal format: `10000000`?*

### Additional Reading
* Bits, bytes and the dreaded little-endian: https://edil.com.br/blog/bits-bytes-and-the-dreaded-little-endian
* learnmeabitcoin.com:
    - hexadecimal overview: https://learnmeabitcoin.com/technical/hexadecimal
    - little endian overview: https://learnmeabitcoin.com/technical/little-endian

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="05_vectors_and_result_enum.md">>>> Next Lesson: Vectors and the Result Enum</a></p>
</div>
