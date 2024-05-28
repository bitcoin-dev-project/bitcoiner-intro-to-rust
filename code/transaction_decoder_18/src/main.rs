use std::io::Read;
mod transaction;
use self::transaction::{Amount, Input, Output, Transaction, Txid};
use sha2::{Digest, Sha256};

fn read_u32(transaction_bytes: &mut &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Amount {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();

    Amount::from_sat(u64::from_le_bytes(buffer))
}

fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }
    }
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Txid {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    Txid::from_bytes(buffer)
}

fn read_script(transaction_bytes: &mut &[u8]) -> String {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    hex::encode(buffer)
}

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
        let amount = read_amount(&mut bytes_slice);
        let script_pubkey = read_script(&mut bytes_slice);

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    let lock_time = read_u32(&mut bytes_slice);

    let txid = hash_transaction(&transaction_bytes);

    let transaction = Transaction {
        txid,
        version,
        inputs,
        outputs,
        lock_time,
    };

    println!("Transaction: {}", serde_json::to_string_pretty(&transaction).unwrap());
}

#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;

    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes);
        assert_eq!(length, 256_u64.pow(7));

        // https://mempool.space/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1
        // fd is 253
        // transaction has 20,000 empty inputs
        let transaction_hex = "fd204e";
        let decoded = hex::decode(transaction_hex).unwrap();
        let mut bytes = decoded.as_slice();
        let length = read_compact_size(&mut bytes);
        let expected_length = 20_000_u64;
        assert_eq!(length, expected_length);
    }
}
