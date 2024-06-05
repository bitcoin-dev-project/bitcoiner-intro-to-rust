# JSON Serialization

If you have set up Bitcoin Core and run a `bitcoin-cli` command, you will notice that the output is returned in a certain format.
For example:
```shell
$ bitcoin-cli decoderawtransaction 010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000
``` 

This will print the following:
```console
{
  "txid": "3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2",
  "hash": "3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2",
  "version": 1,
  "size": 371,
  "vsize": 371,
  "weight": 1484,
  "locktime": 0,
  "vin": [
    {
      "txid": "8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542",
      "vout": 1,
      "scriptSig": {
        "asm": "30440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b[ALL] 030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5",
        "hex": "4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5"
      },
      "sequence": 4294967294
    },
    {
      "txid": "9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875",
      "vout": 0,
      "scriptSig": {
        "asm": "3045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47[ALL] 03c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf",
        "hex": "483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf"
      },
      "sequence": 4294967294
    }
  ],
  "vout": [
    {
      "value": 0.01028587,
      "n": 0,
      "scriptPubKey": {
        "asm": "OP_DUP OP_HASH160 4ef88a0b04e3ad6d1888da4be260d6735e0d3084 OP_EQUALVERIFY OP_CHECKSIG",
        "desc": "addr(mniWjppVtvB5sp9hCcrtwgMCJE2cngUggc)#wstlfjz6",
        "hex": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac",
        "address": "mniWjppVtvB5sp9hCcrtwgMCJE2cngUggc",
        "type": "pubkeyhash"
      }
    },
    {
      "value": 0.02002000,
      "n": 1,
      "scriptPubKey": {
        "asm": "OP_HASH160 76c0c8f2fc403c5edaea365f6a284317b9cdf725 OP_EQUAL",
        "desc": "addr(2N458frTrxCbHmhvhZWqdTMFdpnjJ3Tt68F)#ud5r67fc",
        "hex": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587",
        "address": "2N458frTrxCbHmhvhZWqdTMFdpnjJ3Tt68F",
        "type": "scripthash"
      }
    }
  ]
}
```

This is the [JSON](https://www.json.org/json-en.html) format, a human-readable data-interchange format that transmits data objects consisting of key-value pairs and arrays.

In Rust, there is a popular crate available for taking some data and converting it (or "**serializing**" it) into the JSON format.
We can use the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate for this.

Let's add this to our Cargo.toml file.
We'll need to bring in the `derive` feature from `serde` so that we can derive the `Serialize` implementation for our structs.
We'll also need to bring `serde_json` so that we can convert our struct to a JSON formatted `String`.

```toml
[package]
name = "transaction_decoder_14"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.115"
```

We can follow the example from the documentation here: https://docs.rs/serde_json/latest/serde_json/index.html#creating-json-by-serializing-data-structures.

First, we'll have to import the `Serialize` feature with the `use serde::Serialize;` statement.
Then, we can add the `#[derive(Serialize)]` attribute to our `Input` struct.
This will work as long as all of the primitive types it contains can be serialized.
Finally, we'll call `to_string()` method on `serde_json` to convert inputs into a JSON formatted string.
Let's try that and see what happens.

*Note: We don't need to import top-level modules such as `serde_json` as those will already be included by default in our `main.rs` file.*

```rust
use std::io::Read;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Input {
    txid: [u8; 32],
    output_index: u32,
    script: Vec<u8>,
    sequence: u32,
}
...
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
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

    let json_inputs = serde_json::to_string(&inputs).unwrap();

    println!("Version: {}", version);
    println!("Inputs: {}", json_inputs);
}
...
```

Ok if we print this out now, we'll see that it's now JSON formatted, but it's still hard to read:

```console
Version: 1
Inputs: [{"txid":[128,115,205,249,71,172,151,194,59,119,176,85,33,125,167,141,58,215,29,48,225,246,192,149,190,139,48,247,214,193,213,66],"output_index":1,"script":[71,48,68,2,32,119,19,97,170,229,94,132,73,107,158,123,6,224,165,61,209,34,161,66,95,133,132,10,247,165,43,32,250,50,152,22,7,2,32,34,29,217,33,50,232,46,249,193,51,203,26,16,107,100,137,56,146,161,26,207,44,250,26,219,118,152,220,220,2,240,27,1,33,3,0,119,190,37,220,72,46,127,74,186,214,1,21,65,104,129,254,78,249,138,243,60,146,76,216,178,12,164,229,126,139,213],"sequence":4294967294},{"txid":[156,180,20,202,244,166,51,179,68,108,34,214,23,75,230,112,179,224,231,70,2,76,192,193,239,14,21,243,197,124,200,117],"output_index":0,"script":[72,48,69,2,33,0,224,216,95,236,230,113,211,103,200,212,66,169,98,48,149,76,221,164,185,207,149,233,237,199,99,97,109,5,217,62,148,67,2,32,35,48,213,32,64,141,144,149,117,197,246,151,108,196,5,179,4,38,115,182,1,244,242,20,11,46,77,68,126,103,28,71,1,33,3,196,58,252,205,55,170,231,16,127,90,67,245,183,178,35,208,52,231,88,59,119,200,205,16,132,216,104,149,167,52,26,191],"sequence":4294967294}]
```

Our `txid` and `script` fields need to be converted to hexadecimal format so that they appear more human readable.
For now, we can store them as `String` types instead.
This way we can leverage the `hex` library we've already included, and call `hex::encode` to encode both of these types as hex strings.

*Note: It's probably a better idea to store these as their original types for internal purposes and calculations and separate the logic for how we display or serialize data from how it is stored.
We'll revisit this separation of concerns in lesson 16 and talk about how to do that.*

First, we'll update our `Input` string to change both of these fields to `String` types:
```rust
#[derive(Debug, Serialize)]
struct Input {
    txid: String,
    output_index: u32,
    script: String,
    sequence: u32,
}
```

Next, we'll update our `read_txid` and `read_script` methods to return a string and call `hex::encode` on the buffers:

```rust
fn read_txid(transaction_bytes: &mut &[u8]) -> String {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse(); // txids are formatted in big endian
    hex::encode(buffer)
}

fn read_script(transaction_bytes: &mut &[u8]) -> String {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    hex::encode(buffer)
}
```

Let's run this now and see what happens.

```console
Version: 1
Inputs: [{"txid":"8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542","output_index":1,"script":"4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5","sequence":4294967294},{"txid":"9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875","output_index":0,"script":"483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf","sequence":4294967294}]
```

Ok, a little better but it's still hard to read.
Let's look at the documentation and see if there's a better method we can use: https://docs.rs/serde_json/latest/serde_json/#functions.
Looks like there's a method called `to_string_pretty`.
Let's try that instead of `to_string` and see how that looks.

```console
Version: 1
Inputs: [
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
]
```

Ok, that's way better! Starting to look much more similar to the output from `bitcoin-cli`.

Let's make one more modification to place all the different pieces of a transaction, such as the version, inputs and outputs all into a `Transaction` struct.
First, declare the `Transaction` struct:
```rust
#[derive(Debug, Serialize)]
struct Transaction {
    version: u32,
    inputs: Vec<Input>,
}
```

Next, we'll update our `main` function:
```rust
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
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

    let transaction = Transaction {
        version,
        inputs,
    };
    
    println!("Transaction: {}", serde_json::to_string_pretty(&transaction).unwrap());
}
```

Now we should see an output with everything neatly printed out under one `Transaction` JSON object! 

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
  ]
}
```

Pretty neat! Let's keep moving and finish decoding the transaction in the next lesson.
After that, we'll set up our program to handle user inputs and deal with handling errors.
Onwards!

<hr/>

<div>
    <p align="right"><a href="15_reading_outputs_and_tuple_structs.md">>>> Next Lesson: Reading Outputs and Tuple Structs</a></p>
</div>
