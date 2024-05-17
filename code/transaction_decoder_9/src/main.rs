use std::io::Read;

fn read_version(transaction_bytes: &mut &[u8]) -> u32 { // the argument type is a mutable reference to a slice of u8 integers
    println!("transaction_bytes memory address before reading {:p}", *transaction_bytes); // make sure to dereference the transaction_bytes variable to see the memory address of the object it is referring to
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    println!("transaction bytes memory address after reading {:p}\n", *transaction_bytes);
    println!("transaction bytes: {:?}\n", transaction_bytes);

    u32::from_le_bytes(buffer)
}

fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    println!("bytes slice memory address before calling read_version: {:p}\n", bytes_slice);
    let version = read_version(&mut bytes_slice); // pass in a mutable reference to the bytes_slice
    println!("bytes slice memory address after calling read_version: {:p}\n", bytes_slice);
    println!("bytes slice after calling read_version: {:?}", bytes_slice);
    println!("Version: {}", version);

    let mut input_count = [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count, [2_u8]);
}
