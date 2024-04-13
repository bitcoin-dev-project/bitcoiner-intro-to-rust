# Finalizing Decoding A Legacy Transaction

So we have two fields left for decoding a pre-segwit transaction:
1. Lock Time
2. Transaction ID

The lock time field is fairly straightforward. It's 4 bytes which we'll interpret as a `u32` integer. We won't go into too much detail about this field represents. There's a [good explanation](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#lock-time) in Mastering Bitcoin.

The final field we'll want to decode and display to the user is the transaction ID. This one is interesting because it's not actually contained in the raw transaction, but is instead *calculated* based on the raw transaction fields. A great overview of how this works can be found on [Learn Me A Bitcoin](https://learnmeabitcoin.com/technical/transaction/input/txid/). The basic idea is that the ID is calculated by *hashing* all of the transaction data. More specifically:
1. For legacy transactions you hash all of the transaction data twice using the SHA256 algorithm.
2. For segwit transactions you hash all of the transaction data twice excluding the marker, flag, witness fields.

For more information on hashing and what it's all about, check out this explanation from [Learn Me A Bitcoin](https://learnmeabitcoin.com/technical/cryptography/hash-function/#hash256)

We'll start by creating a new method `hash_raw_transaction` with the following function signature: `fn hash_raw_transaction(bytes: &[u8]) -> [u8; 32]`. We'll take a slice reference as an argument and then return an array of 32 bytes. The sha256 algorithm always returns a 32 byte hash so we know we can always know what the size will be at compile time.

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

2. The `Sha256` object implements the `Digest` trait which defines the `finalize` method. If we look at this method [in the docs](https://docs.rs/sha2/latest/sha2/trait.Digest.html#tymethod.finalize), we see that it returns a type `GenericArray<u8, Self::OutputSize>`. This comes from the `generic_array` Rust crate, which defines the [`GenericArray` struct](https://docs.rs/generic-array/latest/generic_array/struct.GenericArray.html). If we look closer at this struct, we can see that it implements the [`From`](https://docs.rs/generic-array/latest/generic_array/struct.GenericArray.html#impl-From%3CGenericArray%3CT,+%3CConst%3CN%3E+as+IntoArrayLength%3E::ArrayLength%3E%3E-for-%5BT;+N%5D) trait for converting to an array `[T; N]`. This gives us access to the trait method `into` which will convert the `GenericArray` into the expected array type from the function signature, which in this case is `[u8; 32]`. For more details on the `From` trait and how it works, [see here](https://doc.rust-lang.org/nightly/core/convert/trait.From.html).

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

If we try running this now, you might get a compiler error that looks something like this:

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

If we run this now, the program will compile, but we will get some warnings about the unused variables `lock_time` and `transaction_id`. Let's use them by adding them to our `Transaction` struct. We can simply add the `lock_time` field as a `u32` field. However, let's think a bit more about the transaction ID field. Remember, when we serialize this field and display it to a user, it is always returned in Big Endian format. This was a decision Satoshi made early on that was probably not necessary, but is too hard to reverse at this point. What this means is that we'll have to reverse the bytes first before converting to hex format for display to the user. 

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
  "version": 1,
  "inputs": [
    {
      "txid": "f8c693771b2992a11b53c045369ab31a920de1d921ff3c148a9d0487c8f90baf",
      "output_index": 16,
      "script": "483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569",
      "sequence": 4294967295
    },
    {
      "txid": "e51d2177332baff9cfbbc08427cf0d85d28afdc81411cdbb84f40c95858b080d",
      "output_index": 1,
      "script": "4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87",
      "sequence": 4294967295
    }
  ],
  "outputs": [
    {
      "amount": 61.92597494,
      "script_pubkey": "76a914764b8c407b9b05cf35e9346f70985945507fa83a88ac"
    },
    {
      "amount": 1.27,
      "script_pubkey": "76a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac"
    }
  ],
  "lock_time": 0,
  "transaction_id": "54dc90aa618ea1c300aac021399c66f5f5152848a57984a757075036e3046147"
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

Now that we know the transaction ID of our raw transaction hex, let's look it up on the [blockstream explorer](https://blockstream.info/tx/54dc90aa618ea1c300aac021399c66f5f5152848a57984a757075036e3046147) and verify some of the details. For the inputs, you can check the transaction ids and output indexes (16 and 1). For the outputs, verify the amounts are the same as what is displayed in your terminal. If you click on the "Details +" button, you can also verify the ScriptSigs and the ScriptPubKeys. Nicely done so far! 

Our `main.rs` looks much simpler now and there's a greater separation of concerns making our codebase easier to follow and maintain! Let's start talking about error handling next.

### Extra Practice
* Take a stab at simplifying our `main.rs` file even more. Try separating out the compact size logic into a separate file and as a separate struct. Maybe the `hash_transaction` method can actually be a type-associated function on the `Transaction` struct.