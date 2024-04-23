# JSON Serialization

If you have set up Bitcoin Core and run a `bitcoin-cli` command, you will notice that the output is returned in a certain format. For example:
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

In Rust, there is a popular crate available for taking some data and converting it into or "**serializing**" it in the JSON format. We can use the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate for this.

Let's add this to our Cargo.toml file. We'll need to bring in the `derive` feature from `serde` so that we can derive the `Serialize` implementation for our structs. We'll also need to bring `serde_json` so that we can convert our struct to a JSON formatted `String`.

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

First, we'll have to import the `Serialize` feature with the `use serde::Serialize;` statement. Then, we can add the `#[derive(Serialize)]` attribute to our `Input` struct. This will work as long as all of the primitive types it contains can be serialized. Finally, we'll call `to_string()` method on `serde_json` to convert inputs into a JSON formatted string. Let's try that and see what happens.

*Reminder: We don't need to import top-level modules such as `serde_json` as those will already be included by default in our `main.rs` file.*

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
Inputs: [{"txid":[248,198,147,119,27,41,146,161,27,83,192,69,54,154,179,26,146,13,225,217,33,255,60,20,138,157,4,135,200,249,11,175],"output_index":16,"script":[72,48,69,2,33,0,144,74,46,14,143,89,127,193,204,39,27,98,148,176,151,166,237,201,82,227,12,69,62,53,48,249,36,146,116,151,105,168,2,32,24,70,76,34,91,3,194,135,145,175,6,188,127,237,18,157,202,174,255,158,200,19,90,218,31,177,23,98,206,8,30,169,1,65,4,218,40,145,146,176,132,93,91,137,206,130,102,93,136,172,137,215,87,207,197,253,153,123,29,232,174,71,247,120,12,230,163,34,7,88,59,116,88,209,210,243,253,107,58,59,132,42,234,158,183,137,226,190,165,123,3,212,14,104,77,142,30,5,105],"sequence":4294967295},{"txid":[229,29,33,119,51,43,175,249,207,187,192,132,39,207,13,133,210,138,253,200,20,17,205,187,132,244,12,149,133,139,8,13],"output_index":1,"script":[72,48,69,2,32,54,157,247,212,39,149,35,158,171,249,212,26,238,117,227,255,32,82,23,84,82,43,208,103,137,15,142,237,246,4,76,109,2,33,0,154,207,189,136,213,29,132,45,184,122,185,144,164,139,237,18,177,248,22,233,85,2,208,25,142,208,128,222,69,106,152,141,1,65,4,224,236,152,138,103,153,54,206,168,10,136,230,6,61,98,220,133,24,46,84,138,83,95,174,205,110,86,159,181,101,99,61,229,180,232,61,90,17,251,173,139,1,144,140,231,30,3,116,176,6,216,70,148,176,111,16,189,193,83,202,88,165,63,135],"sequence":4294967295}]
```

Our `txid` and `script` fields need to be converted to hexadecimal format so that they appear more human readable. For now, we can store them as `String` types instead. This way we can leverage the `hex` library we've already included, and call `hex::encode` to encode both of these types as hex strings. 

*Note: It's probably a better idea to store these as their original types for internal purposes and calculations and separate the logic for how we display or serialize data from how it is stored. We'll revisit this separation of concerns in lesson 16 and talk about how to do that.*

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
Inputs: [{"txid":"f8c693771b2992a11b53c045369ab31a920de1d921ff3c148a9d0487c8f90baf","output_index":16,"script":"483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569","sequence":4294967295},{"txid":"e51d2177332baff9cfbbc08427cf0d85d28afdc81411cdbb84f40c95858b080d","output_index":1,"script":"4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87","sequence":4294967295}]
```

Ok, a little better but it's still hard to read. Let's look at the documentation and see if there's a better method we can use: https://docs.rs/serde_json/latest/serde_json/#functions. Looks like there's a method called `to_string_pretty`. Let's try that instead of `to_string` and see how that looks. 

```console
Version: 1
Inputs: [
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
]
```

Ok, that's way better! Starting to look much more similar to the output from `bitcoin-cli`.

Let's make one more modification to place all the different pieces of a transaction, such as the version, inputs and outputs all into one `Transaction` struct. 

We'll add the `Transaction` struct:
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

And now we should see an output with everything neatly printed out under one `Transaction` JSON object! 

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

Pretty neat! Let's keep moving and finish decoding the transaction in the next lesson. After that, we'll set up our program to handle user inputs and deal with handling errors. Onwards!

<hr/>

<div>
    <p align="right"><a href="15_reading_outputs_and_tuple_structs.md">>>> Next Lesson: Reading Outputs and Tuple Structs</a></p>
</div>
