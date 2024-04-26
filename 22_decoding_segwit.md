# Decoding Segwit

Let's now decode a Segwit transaction. Given all the changes we made in the previous lesson, this should be relatively easy to set up! 

We'll start by updating our `Decodable` implementation for `Transaction`:

```rust
#[derive(Debug)]
impl Decodable for Transaction {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let version = Version::consensus_decode(r)?;
        let inputs = Vec::<TxIn>::consensus_decode(r)?;
        if inputs.is_empty() {
            let segwit_flag = u8::consensus_decode(r)?;
            match segwit_flag {
                1 => {
                    let mut inputs = Vec::<TxIn>::consensus_decode(r)?;
                    let outputs = Vec::<TxOut>::consensus_decode(r)?;
                    for txin in inputs.iter_mut() {
                        txin.witness = Witness::consensus_decode(r)?;
                    }
                    if !inputs.is_empty() && inputs.iter().all(|input| input.witness.is_empty()) {
                        Err(Error::ParseFailed("witness flag set but no witnesses present"))
                    } else {
                        Ok(Transaction {
                            version,
                            inputs,
                            outputs,
                            lock_time: u32::consensus_decode(r)?,
                        })
                    }
                }
                // We don't support anything else
                x => Err(Error::UnsupportedSegwitFlag(x)),
            }
        // non-segwit
        } else {
            Ok(Transaction {
                version,
                inputs,
                outputs: Vec::<TxOut>::consensus_decode(r)?,
                lock_time: u32::consensus_decode(r)?,
            })
        }
    }
}
```

Changes:
1. After decoding the version and the inputs, the next thing we want to do is check the input count. If it's zero, then this is likely a marker for a segwit transaction. If it's not, it's a legacy transaction, and we'll simply go ahead to decoding the outputs and the locktime. The following byte after the marker is the flag. Currently, the only supported flag is `1`. Anything else and we'll throw a new error that we create called the `UnsupportedSegwitFlag` (exactly how the rust-bitcoin library does it). We'll take a look at our `Error` enum in a moment.
2. With a valid segwit flag, we then move forward by decoding the inputs and the outputs. What follows after that is the witness data. We know that for every input there is a witness stack, or a collection of elements that comprise the witness for that particular input. Another way we can think about this is a `vec` of byte collections or a `Vec<Vec<u8>>` for each input. Though the witness is segregated towards the end of the raw transaction data, it's convenient to display the witness with each input for serialization. So we'll add this field to `TxIn` struct, which we'll look at below. Here's an example output of a decoded segwit transaction, which we use for another integration test case:
```console
{
  "transaction_id": "17e1fcaae34575d0d1566c1ae64bf4c9f7b7b9df0ff505015d3eb72460fe3a61",
  "version": 2,
  "inputs": [
    {
      "txid": "0c0fe4cc11c477231ad80de3496e20f40cc3088797c50aec8996e955c87e46d2",
      "vout": 1,
      "txinwitness": [
        "3044022036c03ad8796f865c9348403fb705d5b984a4ef9565e8b0c81a1069f0f36bbeeb022034e9d5679e9783a441586fae034c78c60854ed71b7b53e6ef169e4f58153356101",
        "0355dd8af3cbfe5c3d3424b441069455a59ce0c8d5fe628da0913dae55037ef928"
      ],
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.02034575,
      "script_pubkey": "00146f048d1381aa546a3e89e87f7549efc45f150b7f"
    },
    {
      "amount": 0.01035945,
      "script_pubkey": "0014d850c02b89821f0f189ca7e81756c102241f7f40"
    }
  ],
  "locktime": 2422463
}
```
3. If the witness flag is present but the witness stack if not available for all of the inputs, we'll return a new `Error::ParseFailed`. 
4. Two new methods you may not be familiar with but are very commonly used to iterate over collections of data are the [`iter`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter) and [`iter_mut`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter_mut) methods. These methods convert a collection, such as a `Vec` or an array, into an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) type. This is what allows us to step through a collection in a loop for example, and provides many other methods such as `map`, `filter`, `find`, etc. which are very useful for traversing a collection or modifying it. Take a look at the documentation for the [`all` method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all).

Let's take a look at our updated `Error` enum:
```rust
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseFailed(&'static str),
    UnsupportedSegwitFlag(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::ParseFailed(s) => write!(f, "parse failed: {}", s),
            Error::UnsupportedSegwitFlag(swflag) =>
                write!(f, "unsupported segwit version: {}", swflag),
        }
    }
}

impl std::error::Error for Error {}
```

Changes:
1. We've added the `Error::ParseFailed` and `Error::UnsupportedSegwitFlag` error variants and implemented `Display` for them. 
2. You might notice the syntax `&'static str`. You're probably familiar already with `&str`, so what does `&'static str` mean? Well, the tick mark indicates a `lifetime` parameter. Whenever we have a field in a struct or enum that is a borrowed reference to some data, we have to indicate to our program somehow how long that data is expected to live relative to encapsulating structure. Why? Because what would happen if we pass around a struct and then at some point the field's data goes out of scope. The struct would have a field with a dangling pointer which is a memory no-no. So our borrowed reference must always *outlive* the struct (or encapsulating structure). In this case, `'static` means that the borrowed reference will live for as long as the program runs. In other words, we won't have to worry about the underlying `String` data on the heap being deallocated while the program runs. This might sound pretty confusing. Don't worry if its difficult to understand. Lifetimes are one of the most challenging aspects of learning Rust and require lots of practice to really understand. We'll revisit this topic in a bonus section later on.

Next, let's look at our updated `TxIn` struct and new `Witness` struct:
```rust
...

#[derive(Debug)]
pub struct TxIn {
    pub previous_txid: Txid,
    pub previous_vout: u32,
    pub script_sig: String,
    pub sequence: u32,
    pub witness: Witness,
}

impl Serialize for TxIn {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut txin = s.serialize_struct("TxIn", 4)?;
        txin.serialize_field("txid", &self.previous_txid)?;
        txin.serialize_field("vout", &self.previous_vout)?;

        if self.witness.is_empty() {
            txin.serialize_field("scriptSig", &self.script_sig)?;
        } else {
            txin.serialize_field("txinwitness", &self.witness)?;
        }

        txin.serialize_field("sequence", &self.sequence)?;
        txin.end()
    } 
}

#[derive(Debug)]
pub struct Witness {
    content: Vec<Vec<u8>>,
}

impl Witness {
    pub fn new() -> Self {
        Witness { content: vec![] }
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl Serialize for Witness {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let mut seq = s.serialize_seq(Some(self.content.len()))?;

        for elem in self.content.iter() {
            seq.serialize_element(&hex::encode(&elem))?;
        }
        seq.end()
    }
}

...
```

Changes:
1. We've added custom serialization for our `TxIn` struct. We set field names that match what `bitcoin-cli` uses and will optionally show the `scriptSig` or the `witness` depending on what's present.
2. Our `Witness` struct has a `content` field which is essentially just a `Vec<Vec<u8>>`. We added a `new` method which we'll call when first decoding a `TxIn` and before adding the witness to it. An empty witness will be added to each `TxIn` when it's first decoded and we have the `is_empty` method as a convenient method we can call to determine if the witness is empty or not. This eventually calls the [`is_empty` method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty) on a `Vec`.
3. We want to serialize the `Witness` so that it shows up as an array of hex-encoded bytes. We can use the [`serialize_seq`](https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map) method to do this and call `hex::encode` on each byte collection in the encapsulating `Vec`.

Lastly, let's look at our `Decodable` implementations:
```rust
...

impl Decodable for TxIn {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(TxIn {
            previous_txid: Txid::consensus_decode(r)?,
            previous_vout: u32::consensus_decode(r)?,
            script_sig: String::consensus_decode(r)?,
            sequence: u32::consensus_decode(r)?,
            witness: Witness::new(),
        })
    }
}

impl Decodable for Witness {
    fn consensus_decode<R: Read>(r: &mut R) -> Result<Self, Error> {
        let mut witness_items = vec![];
        let count = u8::consensus_decode(r)?;
        for _ in 0..count {
            let len = CompactSize::consensus_decode(r)?.0;
            let mut buffer = vec![0; len as usize];
            r.read_exact(&mut buffer).map_err(Error::Io)?;
            witness_items.push(buffer);
        }
        Ok(Witness{ content: witness_items })
    }
}

...
```

Changes:
1. When we first decode a `TxIn`, we'll add a new empty `Witness` field. We'll fill this out later on while decoding a `Transaction`. 
2. The `Witness` has the following structure: a count of the stack items with each item preceded by a `CompactSize` to indicate it's length. So we'll decode the first byte to get the count of witness items. Then, we'll decode the variable length field and push that into our witness `vec`. Finally we'll return the `Witness` struct with the content of our `witness_items`. 

Alright! This should all work now! Let's update our integration test.

```rust
use std::fs;

#[test]
fn test_legacy() {
    let raw_transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let json = transaction_decoder_22::run(raw_transaction_hex.to_string()).unwrap();
    let expected = fs::read_to_string("tests/test_transaction_legacy.json").unwrap();
    assert_eq!(expected, json);
}

#[test]
fn test_segwit() {
    let raw_transaction_hex = "02000000000101d2467ec855e99689ec0ac5978708c30cf4206e49e30dd81a2377c411cce40f0c0100000000feffffff028f0b1f00000000001600146f048d1381aa546a3e89e87f7549efc45f150b7fa9ce0f0000000000160014d850c02b89821f0f189ca7e81756c102241f7f4002473044022036c03ad8796f865c9348403fb705d5b984a4ef9565e8b0c81a1069f0f36bbeeb022034e9d5679e9783a441586fae034c78c60854ed71b7b53e6ef169e4f58153356101210355dd8af3cbfe5c3d3424b441069455a59ce0c8d5fe628da0913dae55037ef928bff62400";
    let json = transaction_decoder_22::run(raw_transaction_hex.to_string()).unwrap();
    let expected = fs::read_to_string("tests/test_transaction_segwit.json").unwrap();
    assert_eq!(expected, json);
}
```

The `tests` directory has been updated with the expected json outputs for the legacy and segwit transactions. These should both pass now! 

Let's also test this out with `cargo run`.

```shell
$ cargo run -- 02000000000101d2467ec855e99689ec0ac5978708c30cf4206e49e30dd81a2377c411cce40f0c0100000000feffffff028f0b1f00000000001600146f048d1381aa546a3e89e87f7549efc45f150b7fa9ce0f0000000000160014d850c02b89821f0f189ca7e81756c102241f7f4002473044022036c03ad8796f865c9348403fb705d5b984a4ef9565e8b0c81a1069f0f36bbeeb022034e9d5679e9783a441586fae034c78c60854ed71b7b53e6ef169e4f58153356101210355dd8af3cbfe5c3d3424b441069455a59ce0c8d5fe628da0913dae55037ef928bff62400
```

This should output the following:
```console
{
  "transaction_id": "17e1fcaae34575d0d1566c1ae64bf4c9f7b7b9df0ff505015d3eb72460fe3a61",
  "version": 2,
  "inputs": [
    {
      "txid": "0c0fe4cc11c477231ad80de3496e20f40cc3088797c50aec8996e955c87e46d2",
      "vout": 1,
      "txinwitness": [
        "3044022036c03ad8796f865c9348403fb705d5b984a4ef9565e8b0c81a1069f0f36bbeeb022034e9d5679e9783a441586fae034c78c60854ed71b7b53e6ef169e4f58153356101",
        "0355dd8af3cbfe5c3d3424b441069455a59ce0c8d5fe628da0913dae55037ef928"
      ],
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.02034575,
      "script_pubkey": "00146f048d1381aa546a3e89e87f7549efc45f150b7f"
    },
    {
      "amount": 0.01035945,
      "script_pubkey": "0014d850c02b89821f0f189ca7e81756c102241f7f40"
    }
  ],
  "locktime": 2422463
}
```

Congratulations! Your program can now decode any transaction. Feel free to play with this a bit. Visit [mempool.space/testnet](https://mempool.space/testnet), find a random transaction hex, decode it and compare your results! 

### Additional Reading
* Lifetimes: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
* Iterators: https://doc.rust-lang.org/book/ch13-02-iterators.html

### Where To Go From Here
Check out these additional resources:
* Practice at Exercism.io: https://exercism.org/tracks/rust
* Effective Rust: https://effective-rust.com/cover.html
* Programming Rust: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/

