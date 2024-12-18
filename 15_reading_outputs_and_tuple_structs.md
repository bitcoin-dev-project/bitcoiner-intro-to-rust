# Reading Outputs and Tuple Structs

Reading the outputs is fairly straightforward now that we have all the other pieces in place. Similar to inputs, there is a compactSize integer which indicates the number of outputs. An output is comprised of an amount, indicating the number of satoshis to be transferred. This field is 8 bytes in length, which we'll represent as a `u64` type. This is followed by a compactSize integer indicating the length of the *output script*. The script, also known as the `scriptPubKey` contains the conditions that that will need to be  fulfilled in order to spend those satoshis. For example, if an output has a [`P2PKH` script](https://learnmeabitcoin.com/technical/script/p2pkh/), then the spender will need to present a signature and a public key in order to unlock those funds. 

So all we need is to do now is the following:
1. Create a new `Output` struct
2. Modify our `Transaction` struct 
3. Write a new `read_u64` function to return the `amount`
4. Use our existing `read_script` function to read the `script_pubkey`.

Let's make those changes:

```rust
...

#[derive(Debug, Serialize)]
struct Transaction {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

...

#[derive(Debug, Serialize)]
struct Output {
    amount: u64,
    script_pubkey: String,
}

...

fn read_u64(transaction_bytes: &mut &[u8]) -> u64 {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();

    u64::from_le_bytes(buffer)
}

...

fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);

    // Read inputs
    let input_length = read_compact_size(&mut bytes_slice);
    let mut inputs = vec![];

    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);

        inputs.push(Input {
            txid,
            output_index,
            script,
            sequence,
        });
    }

    // Read outputs
    let output_length = read_compact_size(&mut bytes_slice);
    let mut outputs = vec![];

    for _ in 0..output_length {
        let amount = read_u64(&mut bytes_slice);
        let script_pubkey = read_script(&mut bytes_slice);

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    let transaction = Transaction {
        version,
        inputs,
        outputs,
    };
    
    println!("Transaction: {}", serde_json::to_string_pretty(&transaction).unwrap());
}

...
```

Let's run this with `cargo run` and take a look at the printout.

```console
Transaction: {
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
      "amount": 1028587,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 2002000,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ]
}
```

Not bad! We're starting to see most of the transaction details. The amounts are a little hard to read in Satoshis and are typically printed in Bitcoin so let's update that. There are 100,000,000 satoshis in one bitcoin, so we'll modify the calculation. Let's try something like the following and see what happens `let amount = read_u64(&mut bytes_slice) / 100_000_000;`. 

We'll get a result we don't want. 

```console
...

  "outputs": [
    {
      "amount": 0,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 0,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ]

...
```

The amounts are missing the decimal values because we're doing math on unsigned integers which are whole numbers. What we really want to do is first convert the integer types into floating types. We could do some basic math by first converting the `u64` to an `f64` and then dividing that by `100_000_000.0`. 

However, let's use this as an opportunity to look at some open source code and see how a popular library, such as [Rust-Bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) deals with the amount field. 

From the source code, it appears they create an `Amount` tuple struct to represent the Satoshi / Bitcoin value. https://docs.rs/bitcoin-units/latest/src/bitcoin_units/amount.rs.html#865

```rust
pub struct Amount(u64);
```

A tuple struct is a struct type that wraps a tuple type. A tuple is a comma-separated list of different types in parentheses `()`. The values are accessed by calling `.0`, `.1`, etc. based on their position in the tuple. For example,
```rust
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
```

More info can be found here: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type

A **tuple struct** is simply a tuple wrapped in a struct. This allows us to write methods for that type. 

So let's implement a basic version of Rust-Bitcoin's `Amount` struct here:

```rust
struct Amount(u64);

impl Amount {
    pub fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

#[derive(Debug, Serialize)]
struct Output {
    amount: f64,
    script_pubkey: String,
}
```

Two things to note here:
1. Notice how the `to_btc` method has a `self` argument. The `self` is the specific *instance* of this struct. It will be passed in by default when the instance calls the `to_btc` method. This allows us to access the `u64` value of that instance with `self.0`.
2. We're setting the `self` argument as a *shared reference* with the `&` sign. We're doing this because we don't need ownership of the instance nor do we need to mutate the actual instance. All we need to do is read its contents and output an `f64` result.
2. We made sure to change the `amount` type in `Output` to `f64` from `u64`.

Let's now update our `read_u64` method. Since this is only being used for the amount, we'll rename it to `read_amount` and return the `Amount` type:

```rust
...

fn read_amount(transaction_bytes: &mut &[u8]) -> Amount {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();

    Amount(u64::from_le_bytes(buffer))
}

...
```

Lastly, we'll replace the `read_u64` call with `read_amount` and chain that with a call to `to_btc()`.

```rust
...

    for _ in 0..output_length {
        let amount = read_amount(&mut bytes_slice).to_btc();
        let script_pubkey = read_script(&mut bytes_slice);

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

...
```

If we run `cargo run` now, we should get a nice printout with the amounts in bitcoin and not satoshis. Great!

But there's a problem here. We're storing the amount in `Output` as an `f64` type. Ideally, we would keep this as an `Amount` type for internal purposes, readability and type safety. What we really want is to keep it as the `Amount` type, but convert it to an `f64` denominated in Bitcoin for serialization and display purposes. This will require us to do some custom serialization which we'll talk about in the next lesson. Onwards!

<hr/>

<div>
    <p align="right"><a href="16_custom_serialization_and_generic_functions.md">>>> Next Lesson: Custom Serialization and Generic Functions</a></p>
</div>
