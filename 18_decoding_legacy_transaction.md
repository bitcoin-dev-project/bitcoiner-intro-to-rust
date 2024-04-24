# Decoding A Legacy Transaction

So we have two fields left for decoding a legacy transaction:
1. Lock Time
2. Transaction ID

The lock time field is fairly straightforward. It's 4 bytes which we'll interpret as a `u32` integer. We won't go into too much detail about what this field represents. There's a [good explanation](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#lock-time) in Mastering Bitcoin.

The final field we'll want to decode and display to the user is the transaction ID. This one is interesting because it's not actually contained in the raw transaction, but is instead *calculated* based on the raw transaction fields. A great overview of how this works can be found on [Learn Me A Bitcoin](https://learnmeabitcoin.com/technical/transaction/input/txid/). The basic idea is that the ID is calculated by *hashing* the transaction data. More specifically:
1. For legacy transactions you hash all of the transaction data twice using the SHA256 algorithm.
2. For segwit transactions you hash all of the transaction data twice excluding the marker, flag, witness fields.

For more information on hashing and what it's all about, check out this explanation from [Learn Me A Bitcoin](https://learnmeabitcoin.com/technical/cryptography/hash-function/#hash256)

We'll start by creating a new method `hash_raw_transaction` with the following function signature: `fn hash_raw_transaction(bytes: &[u8]) -> [u8; 32]`. We'll take a slice reference as an argument and then return an array of 32 bytes. The `sha256` algorithm always returns a 32 byte hash so we can always know what the size will be at compile time.

Next, we'll want to use an external library to help us execute the sha256 algorithm. We'll leverage this [popular crate](https://docs.rs/sha2/latest/sha2/).

```toml
name = "transaction_decoder_18"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.115"
sha2 = "0.10.8"
```

Let's bring it into scope by adding a `use sha2::Sha256;` statement. And then let's follow the example from the docs [here](https://docs.rs/sha2/latest/sha2/).

```rust
...

use sha2::Sha256;

...

fn hash_transaction(raw_transaction: &[u8]) -> [u8; 32] {
    // create a sha256 object
    let mut hasher = Sha256::new();

    // write the input message
    hasher.update(&raw_transaction);

    // read digest, consumer hasher
    let hash1 = hasher.finalize();

    // hash1 becomes our new input to be hashed again
    // prepare a new hasher object
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();

    // hash is of the type GenericArray<u8, Self::OutputSize>
    // convert to [u8; 32]
    hash2.into()
}

...
```

Two things to note here:

1. We have to hash the transaction twice. So we'll first hash the transaction and then hash the first result, `hash1`.

2. The `Sha256` object implements the `Digest` trait which defines the `finalize` method. If we look at this method [in the docs](https://docs.rs/sha2/latest/sha2/trait.Digest.html#tymethod.finalize), we see that it returns a type `GenericArray<u8, Self::OutputSize>`. This comes from the `generic_array` Rust crate, which defines the [`GenericArray` struct](https://docs.rs/generic-array/latest/generic_array/struct.GenericArray.html). If we look closer at this struct, we can see that it implements the [`From`](https://docs.rs/generic-array/latest/generic_array/struct.GenericArray.html#impl-From%3CGenericArray%3CT,+%3CConst%3CN%3E+as+IntoArrayLength%3E::ArrayLength%3E%3E-for-%5BT;+N%5D) trait for converting to an array `[T; N]`. This gives us access to the trait method `into` which will convert the `GenericArray` into the expected array type from the function signature, which in this case is `[u8; 32]`. For more details on the `From` trait and how it works, [see here](https://doc.rust-lang.org/std/convert/trait.From.html).

Let's update our `main` fn now to read the lock time and calculate the transaction ID. 

```rust
...

    let lock_time = read_u32(&mut bytes_slice);
    
    let transaction_id = hash_transaction(&transaction_bytes);

    let transaction = Transaction {
        version,
        inputs,
        outputs,
    };

...
```

If we try running this now, we will get a compiler error that looks something like this:

```console
error[E0599]: no function or associated item named `new` found for struct `CoreWrapper` in the current scope
  --> src/main.rs:65:30
   |
65 |     let mut hasher = Sha256::new();
   |                              ^^^ function or associated item not found in `CoreWrapper<CtVariableCoreWrapper<Sha256VarCore, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, OidSha256>>`
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
1  + use sha2::Digest;
   |
```

Whoops! We have to remember to bring the `Digest` trait into scope so that we can call its trait methods. We'll modify our `use` statement:

```rust
use sha2::{Digest, Sha256};
```

If we run this now, the program will compile, but we will get some warnings about the unused variables `lock_time` and `transaction_id`. Let's use them by adding them to our `Transaction` struct. We can simply add the `lock_time` field as a `u32` field. 

However, let's think a bit more about the transaction ID field. Remember, when we serialize this field and display it to a user, it is always returned in Big Endian format. This was a decision Satoshi made early on that was probably not necessary, but is too hard to reverse at this point. What this means is that we'll have to reverse the bytes first before converting to hex format for display to the user.

So we'll do something similar to what we did with the `Amount` struct. We'll represent the transaction id as a separate `Txid` tuple struct. And implement a custom serialization for that struct. We'll end up with something like the following:

*transaction.rs*
```rust
...

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: u32,
}

#[derive(Debug)]
pub struct Txid([u8; 32]);

impl Txid {
   pub fn from_bytes(bytes: [u8; 32]) -> Txid {
      Txid(bytes)
   }
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>
    {
        let mut bytes = self.0.clone();
        bytes.reverse();
        s.serialize_str(&hex::encode(bytes))
    }
}

...
```

Some things to note:
1. We do a setup similar to `Amount` in that we have a type associated function, `from_bytes` to set the tuple field, which remains private.
2. We manually implement the `Serialize` trait for `Txid` instead of auto deriving it with the `derive` attribute.
3. In the `serialize` method, notice how we clone `bytes` first and then reverse it. The `reverse` method reverses the bytes in place. But we don't want to actually modify the ordering in memory. Instead, we want to reverse a copy of the array so that we can properly encode that into hex format for display purposes. And keep the ordering in tact for the `Txid` for internal purposes. 
4. We're using the `serialize_str` method. This requires the `&str` type and not a `String`. However, `hex::encode` returns a `String` type. All we need to do is add an `&` in front to get the type we need. 

Let's modify our `hash_transaction` function:

*main.rs*
```rust
...

fn hash_transaction(raw_transaction: &[u8]) -> Txid {
    // create a sha256 object
    let mut hasher = Sha256::new();

    // write the input message
    hasher.update(&raw_transaction);

    // read digest, consumer hasher
    let hash1 = hasher.finalize();

    // hash1 becomes our new input to be hashed again
    // prepare a new hasher object
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();

    // hash is of the type GenericArray<u8, Self::OutputSize>
    // convert to [u8; 32] with into()
    Txid::from_bytes(hash2.into())
}

...
```

Ok! Let's run this now and see what happens. It should work! And print out the correct transaction ID!

```console
Transaction: {
  "txid": "3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2",
  "version": 1,
  "inputs": [
    {
      "txid": "8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542",
      "output_index": 1,
      "script": "4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5",
      "sequence": 4294967294
    },
    {
      "txid": "9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875",
      "output_index": 0,
      "script": "483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf",
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.01028587,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 0.02002,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ],
  "lock_time": 0
}
```

Pretty cool!

Since we're doing something almost identical with the `txid` field of the `Input` struct, why don't we simply replace it with the `Txid` type instead of the `String` type? 

*transaction.rs*
```rust
#[derive(Debug, Serialize)]
pub struct Input {
    pub txid: Txid,
    pub output_index: u32,
    pub script: String,
    pub sequence: u32,
}
```

Let's also modify the `read_txid` function in the `main.rs` file:
```rust
...

fn read_txid(transaction_bytes: &mut &[u8]) -> Txid {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    Txid::from_bytes(buffer)
}

...
```

We no longer need to reverse the bytes or convert to hex format. That is all handled now with our custom serialization. This is a better separation of concerns as we store the id properly for internal purposes but display it in big endian for the user. 

Run it now and you should get the same result! Much better!

Now that we know the transaction ID of our raw transaction hex, let's look it up on the [blockstream testnet explorer](https://blockstream.info/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2) and verify some of the details. For the inputs, you can check the transaction ids and output indexes (1 and 0). For the outputs, verify the amounts are the same as what is displayed in your terminal. If you click on the "Details +" button, you can also verify the ScriptSigs and the ScriptPubKeys. Nicely done so far! 

The one thing we haven't done and probably won't do in this course is parse the `scriptPubKey` and display the [decoded script](https://en.bitcoin.it/wiki/Script). For example, `bitcoin-cli` will return a `scriptPubKey` field that looks like the following:
```console
...

      "scriptPubKey": {
        "asm": "OP_DUP OP_HASH160 4ef88a0b04e3ad6d1888da4be260d6735e0d3084 OP_EQUALVERIFY OP_CHECKSIG",
        "desc": "addr(mniWjppVtvB5sp9hCcrtwgMCJE2cngUggc)#wstlfjz6",
        "hex": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac",
        "address": "mniWjppVtvB5sp9hCcrtwgMCJE2cngUggc",
        "type": "pubkeyhash"
      }

...
```

We'll ignore that for now, but this could be a fun extra practice challenge for you at the end of the course!

Alright so our `main.rs` is looking much simpler now and there's a greater separation of concerns making our codebase easier to follow and maintain! Let's start talking about error handling next.

<hr/>

<div>
    <p align="right"><a href="19_error_handling.md">>>> Next Lesson: Error Handling</a></p>
</div>
