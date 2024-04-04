use hex;
use std::io::Read;

#[derive(Debug)]
struct Input {
    txid: [u8; 32],
    output_index: u32,
    script: Vec<u8>,
    sequence: u32,
}

fn read_u32(transaction_bytes: &mut &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}

fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        1..=252 => {
            u8::from_le_bytes(compact_size) as u64
        },
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
        },
        _ => {
            panic!("invalid compact size");
        }
    }
}

fn read_txid(transaction_bytes: &mut &[u8]) -> [u8; 32] {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse(); // txids are formatted in big endian
    buffer
}

fn read_script(transaction_bytes: &mut &[u8]) -> Vec<u8> {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer
}

fn main() {
    let transaction_hex = "0100000002af0bf9c887049d8a143cff21d9e10d921ab39a3645c0531ba192291b7793c6f8100000008b483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569ffffffff0d088b85950cf484bbcd1114c8fd8ad2850dcf2784c0bbcff9af2b3377211de5010000008b4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87ffffffff02f6891b71010000001976a914764b8c407b9b05cf35e9346f70985945507fa83a88acc0dd9107000000001976a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac00000000";
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

    println!("Version: {}", version);
    println!("Inputs: {:?}", inputs);
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

        let result = std::panic::catch_unwind(|| {
            let mut bytes = [0_u8].as_slice();
            read_compact_size(&mut bytes);
        });
        assert!(result.is_err());
    }
}
