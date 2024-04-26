# Refactoring and the Rust-Bitcoin Library

We have a working program now that can take in a user-given input and return a decoded transaction. That's pretty neat. However, this will still only work with legacy, pre-segwit transactions. We want to update our program to work with any valid transaction, including segwit transactions. Since this will require some refactoring, let's take this opportunity to look at how the popular open source [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) library handles transaction decoding. 

Before we do that, however, let's add a quick integration test. This will help us easily check that our program is still working correctly after all of our changes. As a reminder, an integration test - unlike a unit test - is a test that operates from the "outside" and tests our application more holistically, ensuring various interactions and interfaces between modules are working as intended.

We'll add a `tests` folder next to the `src` directory and add two files in there:
```console
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── lib.rs
│   ├── main.rs
│   └── transaction.rs
└── tests
    ├── integration_test.rs
    └── test_transaction.json
```

Here is the integration test in `integration_test.rs`:
```rust
use std::fs;

#[test]
fn test_json() {
    let raw_transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let json = transaction_decoder::run(raw_transaction_hex.to_string()).unwrap();
    let expected = fs::read_to_string("tests/test_transaction.json").unwrap();
    assert_eq!(expected, json);
}
```

We're making use of the standard filesystem module `fs`. This will allow us to read the contents of a file and compare it to what's returned by our library. We'll skip test error handling for now and just call `unwrap` where needed. Feel free to copy the contents of the file in `tests/test_transaction.json` from the `code` directory of this course lesson. 

When we run `cargo test`, this test should pass. 

Now, let's move on to our refactoring. Let's start by taking a look at the rust-bitcoin library. The main block of code that I want to call attention to is [here](https://docs.rs/bitcoin/latest/src/bitcoin/blockdata/transaction.rs.html#1239) in the `transaction.rs` file.
```rust
impl Decodable for Transaction {
    fn consensus_decode_from_finite_reader<R: BufRead + ?Sized>(
        r: &mut R,
    ) -> Result<Self, encode::Error> {
        let version = Version::consensus_decode_from_finite_reader(r)?;
        let input = Vec::<TxIn>::consensus_decode_from_finite_reader(r)?;
        // segwit
        if input.is_empty() {
            let segwit_flag = u8::consensus_decode_from_finite_reader(r)?;
            match segwit_flag {
                // BIP144 input witnesses
                1 => {
                    let mut input = Vec::<TxIn>::consensus_decode_from_finite_reader(r)?;
                    let output = Vec::<TxOut>::consensus_decode_from_finite_reader(r)?;
                    for txin in input.iter_mut() {
                        txin.witness = Decodable::consensus_decode_from_finite_reader(r)?;
                    }
                    if !input.is_empty() && input.iter().all(|input| input.witness.is_empty()) {
                        Err(encode::Error::ParseFailed("witness flag set but no witnesses present"))
                    } else {
                        Ok(Transaction {
                            version,
                            input,
                            output,
                            lock_time: Decodable::consensus_decode_from_finite_reader(r)?,
                        })
                    }
                }
                // We don't support anything else
                x => Err(encode::Error::UnsupportedSegwitFlag(x)),
            }
        // non-segwit
        } else {
            Ok(Transaction {
                version,
                input,
                output: Decodable::consensus_decode_from_finite_reader(r)?,
                lock_time: Decodable::consensus_decode_from_finite_reader(r)?,
            })
        }
    }
}
```

Take some time to go through it and see if you can decipher what it's doing. Don't worry if you don't understand every aspect of this block of code. And you can ignore the segwit portion for now. We'll return to that in the next lesson. 

At a high level, it's helpful to see that we're accepting some argument that implements `BufRead`. This is similar to the `Read` trait, in that it's some type that can call a `read` method, such as the `&mut &[u8]` that we've been using. Then what we're doing is calling `consensus_decode` on the various transaction components by passing in our reader, `r`. So calling `Transaction::consensus_decode_from_finite_reader` will trigger a series of `consensus_decode` calls on the various components. Those components in turn will also call `consensus_decode` on their own components, which will ultimately call `consensus_decode` on primitive types such as a `u32` or a `String`. We pass in our reader into each of these methods and they will all do the same thing, which is to read some data, decode it, modify the reader (so that it keeps progressing) and return the decoded type. We are essentially implementing the `Decodable` trait on all of these different types.

So what we are going to do here is remove all of our functions in the `lib.rs` and move all the logic to our `transaction.rs` where we will implement a `Decodable` trait everywhere.

Let's first modify `lib.rs`:
```rust
mod transaction;
use self::transaction::{Decodable, Transaction,};
use std::error::Error;
use clap::{arg, value_parser, Command};

pub fn get_arg() -> String {
    let matches = Command::new("Bitcoin Transaction Decoder")
        .version("1.0")
        .about("Decodes a raw transaction")
        .arg(
            arg!([RAW_TRANSACTION])
                .value_parser(value_parser!(String))
                .required(true)
        )
        .get_matches();

    matches
        .get_one::<String>("RAW_TRANSACTION")
        .cloned()
        .expect("raw transaction is required")
}

pub fn decode(raw_transaction_hex: String) -> Result<Transaction, Box<dyn Error>> {
    let transaction_bytes = hex::decode(raw_transaction_hex).map_err(|e| format!("Hex decoding error: {}", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    Ok(Transaction::consensus_decode(&mut bytes_slice)?)
}

pub fn run(raw_transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction = decode(raw_transaction_hex)?;
    Ok(serde_json::to_string_pretty(&transaction)?)
}
```

Notice how we add a separate `decode` function. This will allow some other program that is using our library to receive the decoded `Transaction` struct instead of the `json` output. Our `run` function will now call `decode` first and then return the `json` output.

Let's take a look at our `transaction.rs` file now. The first thing we want to do is add our own custom error. This will allow us to add more custom error types based on bitcoin transaction validation logic if we want:

*transaction.rs*
```rust
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use std::io::{Read, Write};
use std::fmt;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e)
        }
    }
}

impl std::error::Error for Error {}

...
```

We'll need to implement the `Display` trait so that we can successfully print it out to the terminal. We'll also need to implement the `std::error::Error` trait because our function definitions are looking for a `Result` `Err` type that implements the standard Error trait (`Box<dyn Error>>`).

Next let's update our `Transaction` and `Txid` structs. We'll remove the `txid` field and turn this into a method instead, which will calculate the `txid` based on the correct fields and return that. Remember, with Segwit transactions, we can't simply hash the entire raw transaction. The `txid` is a hash of only certain components of our transaction, excluding the marker, flag and witness fields (the witness is "segregated"). For now, we'll return some dummy data and implement this later.

*transaction.rs*
```rust
...

#[derive(Debug)]
pub struct Transaction {
    pub version: Version,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn txid(&self) -> Txid {
        let txid_data = vec![0; 32];
        Txid::new(txid_data)
    }
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tx = serializer.serialize_struct("Transaction", 5)?;
        tx.serialize_field("transaction id", &self.txid())?;
        tx.serialize_field("version", &self.version)?;
        tx.serialize_field("inputs", &self.inputs)?;
        tx.serialize_field("outputs", &self.outputs)?;
        tx.serialize_field("locktime", &self.lock_time)?;
        tx.end()
    }
}

#[derive(Debug)]
pub struct Txid(pub [u8; 32]);

impl Txid {
    fn new(data: Vec<u8>) -> Txid {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let hash2 = hasher.finalize();

        Txid(hash2.into())
    }
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        s.serialize_str(&hex::encode(bytes))
    }
}

#[derive(Debug, Serialize)]
pub struct Version(pub u32);

#[derive(Debug, Serialize)]
pub struct TxIn {
    pub previous_txid: Txid,
    pub previous_vout: u32,
    pub script_sig: String,
    pub sequence: u32,
}

#[derive(Debug, Serialize)]
pub struct TxOut {
    pub amount: Amount,
    pub script_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct CompactSize(pub u64);

...
```

We have moved the `hash_transaction` function logic into the `new` method on our `Txid` struct. We have also added a custom serialization for our `Transaction`. Notice how the first field we serialize is the `transaction id` which calls the `Transaction` method `txid()`.

Also, we've renamed the `Input` and `Output` structs to `TxIn` and `TxOut` to match the rust-bitcoin library. Finally, we've added a `CompactSize` tuple struct.

Moving on, let's get into the core of our decoding logic. First we'll create the `Decodable` trait:
```rust
...

pub trait Decodable: Sized {
    fn consensus_decode<R: Read>(reader: &mut R) -> Result<Self, Error>;
}

...
```

Ours is a simpler version than the `rust-bitcoin` library. We'll simply take in some type that implements `Read` and return a `Result` of the same type or our custom error. You'll notice we added the `Sized` trait bound. If we don't include this, Rust will complain and ask us to restrict this trait implementation only for types that have a size known at compile time. This is because our `Result` type is returning `Self` and Rust needs to ensure that whatever `Self` is has a size known at compile time. The syntax `Decodable: Sized` is known as a `supertrait`. This means that we can only implement this `Decodable` trait for types that also implement the [`Sized` trait](https://doc.rust-lang.org/std/marker/trait.Sized.html).

Let's start simple. We have our new `Version` struct. Let's implement `Decodable` for that.

```rust
...

impl Decodable for u32 {
    fn consensus_decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl Decodable for Version {
    fn consensus_decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Version(u32::consensus_decode(reader)?))
    }
}

...
```

Notice how we did this. Calling `consensus_decode` triggers a `consensus_decode` call on the `u32` primitive type. This is nice because if any other transaction component is a `u32`, we can just call this method again. Two other things you might notice here:
1. We replace the `read` method with `read_exact`. This ensures that we read the exact amount of bytes to fill the buffer. There are cases where we may only partially fill the buffer if we simply call `read` and we want those cases to return an error. 
2. We have to call `map_err` to convert our standard IO error into our custom error type. This is because our function definition is expecting our custom error type and not the standard one that `read_exact` returns. 

Let's take a look at some of the remaining `Decodable` implementations. Take some time to go through these. I've intentionally omitted the `Decodable` implementations for the primitive types: `u8`, `u16`, `u64` and `String` types. After going through the code below, why don't you take a stab at implementing those yourself? 

*Hint: In order to familiarize yourself with the code below, start by reading the `Decodable` implementation for `Transaction` as a starting point.*

*transaction.rs*
```rust
...

impl Decodable for CompactSize {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let n = u8::consensus_decode(r)?;

        match n {
            0xFF => {
                let x = u64::consensus_decode(r)?;
                Ok(CompactSize(x))
            }
            0xFE => {
                let x = u32::consensus_decode(r)?;
                Ok(CompactSize(x as u64))
            }
            0xFD => {
                let x = u16::consensus_decode(r)?;
                Ok(CompactSize(x as u64))
            }
            n => Ok(CompactSize(n as u64)),
        }
    }
}

impl Decodable for Vec<TxIn> {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let len = CompactSize::consensus_decode(r)?.0;
        let mut ret = Vec::with_capacity(len as usize);
        for _ in 0..len {
            ret.push(TxIn::consensus_decode(r)?);
        }
        Ok(ret)
    }
}

impl Decodable for Txid {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let mut buffer = [0; 32];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(Txid(buffer))
    }
}

impl Decodable for TxIn {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(TxIn {
            previous_txid: Txid::consensus_decode(r)?,
            previous_vout: u32::consensus_decode(r)?,
            script_sig: String::consensus_decode(r)?,
            sequence: u32::consensus_decode(r)?,
        })
    }
}

impl Decodable for Vec<TxOut> {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let len = CompactSize::consensus_decode(r)?.0;
        let mut ret = Vec::with_capacity(len as usize);
        for _ in 0..len {
            ret.push(TxOut::consensus_decode(r)?);
        }
        Ok(ret)
    }
}

impl Decodable for TxOut {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(TxOut {
            amount: Amount::from_sat(u64::consensus_decode(r)?),
            script_pubkey: String::consensus_decode(r)?
        })
    }
}

impl Decodable for Transaction {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(Transaction {
            version: Version::consensus_decode(r)?,
            inputs: Vec::<TxIn>::consensus_decode(r)?,
            outputs: Vec::<TxOut>::consensus_decode(r)?,
            lock_time: u32::consensus_decode(r)?,
        })
    }
}

...
```

After implementing `Decodable` for the primitive types, you should be able to run your code and get a result. The integration test will still fail because we're not yet returning the correct transaction id.

In order to do that, we're going to want to implement the `Encodable` trait, similar to how the rust-bitcoin library does. What this does is re-encode our data types back into bytes according to bitcoin's transaction serialization format. We'll use this to encode the select fields that are required to produce a `txid`. 

This is what our new `Transaction` `txid` method implementation looks like:
```rust
...

impl Transaction {
    pub fn txid(&self) -> Txid {
        let mut txid_data = Vec::new();
        self.version.consensus_encode(&mut txid_data).unwrap();
        self.inputs.consensus_encode(&mut txid_data).unwrap();
        self.outputs.consensus_encode(&mut txid_data).unwrap();
        self.lock_time.consensus_encode(&mut txid_data).unwrap();
        Txid::new(txid_data)
    }
}

...
```

And here's our `Encodable` trait along with two implementations:
```rust
...

pub trait Encodable {
    fn consensus_encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}

...
```

Notice instead of passing a `Reader`, we're passing a `Writer`. So what we're doing is passing some type that we can write data into. In other words, we're collecting bytes. 

Here are two implementations we need to encode the version:
```rust
...

impl Encodable for u32 {
    fn consensus_encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let b = self.to_le_bytes();
        let len = w.write(b.as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for Version {
    fn consensus_encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let len = self.0.consensus_encode(w)?;
        Ok(len)
    }
}

...
```

Notice a similar pattern? We call `consensus_encode` on the first element of the `Version` tuple which is a `u32`. The [`write` method](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write) returns the number of bytes written wrapped in a `Result` so we'll simply return that to indicate a successful encoding. The returned length is of the primitive type `usize` so our `Result` has that as the `Ok` variant. 

In order to finish encoding the transactions `txid`, we'll need to implement `Encodable` for the remaining types: `u8`, `u16`, `u64`, `[u8; 32]`, `String`, `CompactSize`, `Vec<TxIn>`, `Vec<TxOut>`, `Txid`, `TxIn`, `TxOut`, and `Amount`. This seems like an opportunity for some good practice. Why don't you try completing this and then running `cargo test` to ensure the integration test still passes. Feel free to compare your code to what I've written in the `code` directory under `transaction_decoder_21`. 

In the next lesson, we'll finish up by including the logic to decode a Segwit transaction. Get ready to finish up and complete your first command line program written in Rust! 

<hr/>

<div>
    <p align="right"><a href="22_decoding_segwit.md">>>> Next Lesson: Decoding Segwit</a></p>
</div>
